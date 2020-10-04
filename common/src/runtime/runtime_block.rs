use core::convert::{TryFrom, TryInto};

use crate::{runtime::*, Test, TestResult};

#[repr(C)]
pub struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: Status,
    test_status: u32,
    event_id: u32,
    data_size: u32,
    reserved: u32,
    data: [u8; 480],
}

impl RuntimeBlock {
    pub const fn new() -> Self {
        Self {
            magic_sequence: [0u8; 12],
            status: Status::NotReady,
            test_status: 0, // todo: TestStatus::NotRunning.try_into().expect("waah"),
            event_id: Event::None.id(),
            data_size: 0,
            reserved: 0,
            data: [0u8; 480],
        }
    }

    /// Initializes the runtime.
    ///
    /// This call sets up the magic sequence that lets the runner identify
    /// the runtime in device RAM.
    ///
    pub fn init(&mut self) {
        self.magic_sequence = *b"EMT-RUNTIME ";
        self.status = Status::Idle;
        self.event_id = Event::None.id();
    }

    pub fn begin_test(&mut self, should_panic: bool) {
        self.test_status = TestStatus::Running { should_panic }
            .try_into()
            .expect("waah");
    }

    pub fn end_test(&mut self) {
        self.test_status = TestStatus::NotRunning.try_into().expect("waah");
    }

    pub fn fail_test(&mut self) -> Result<(), Error> {
        if !self.test_status().is_running() {
            return Err(Error::NoTestRunning);
        }
        self.request(Event::Result(TestResult::AssertionFail))?;
        self.complete_request();
        self.end_test();
        Ok(())
    }

    pub fn output(&mut self, message: &str) -> Result<(), Error> {
        if !self.test_status().is_running() {
            return Err(Error::NoTestRunning);
        }
        self.request(Event::Output(message))?;
        self.complete_request();
        Ok(())
    }

    pub fn handle_panic(&mut self) -> Result<(), Error> {
        match self.test_status() {
            TestStatus::Running { should_panic } => {
                let result = match should_panic {
                    true => TestResult::Pass,
                    false => TestResult::Panic,
                };
                let result_response = Event::Result(result);
                self.request(result_response)?;
                self.complete_request();
                self.end_test();
            }
            TestStatus::NotRunning => {}
        }
        Ok(())
    }

    /// Polls the runtime for incoming events from the test runner.
    pub fn poll(&mut self, meta: Meta, tests: &[Test]) -> Result<(), Error> {
        match self.read()? {
            Event::MetaRequest => {
                let meta_response = Event::Meta(meta);
                self.respond(meta_response)?;
            }
            Event::Test(id) => {
                let id = id as usize;
                if id < tests.len() {
                    let test = &tests[id];
                    self.begin_test(test.context.should_panic);
                    let context_response = Event::Context(test.context);
                    self.respond(context_response)?;
                    (test.run)();
                    let result_response = Event::Result(TestResult::Pass);
                    self.request(result_response)?;
                    self.complete_request();
                    self.end_test();
                } else {
                    let result_response = Event::Result(TestResult::NotFound);
                    self.request(result_response)?;
                }
            }
            _ => return Err(Error::UnexpectedEvent),
        }
        Ok(())
    }
}

impl Runtime for RuntimeBlock {
    fn status(&mut self) -> Status {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    fn test_status(&mut self) -> TestStatus {
        TestStatus::try_from(self.test_status).expect("waah")
    }

    fn encode_event(&mut self, event: Event) -> Result<(), Error> {
        self.event_id = event.id();
        self.data_size = event.encode(&mut self.data)? as u32;
        Ok(())
    }

    fn decode_event(&mut self) -> Result<Event, Error> {
        Event::decode(self.event_id, &self.data[..self.data_size as usize])
    }
}
