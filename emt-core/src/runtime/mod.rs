mod event;
mod runtime_block;

pub use event::{decode_u32, encode_u32, Event};
pub use runtime_block::RuntimeBlock;

pub const MAGIC_SEQUENCE: [u8; 12] = [
    0x45, 0x4d, 0x54, 0x2d, 0x52, 0x55, 0x4e, 0x54, 0x49, 0x4d, 0x45, 0x20,
];

#[derive(Debug, Copy, Clone)]
pub struct Meta<'a> {
    pub id: &'a str,
    pub version: &'a str,
    pub num_tests: u32,
}

// todo: this should be a safer setup of TryFrom/Into etc.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Status {
    NotReady = 0,
    Idle,
    Send,
    Receive,
}

// todo: haxxx, do it properly
impl Status {
    pub fn from_u32(value: u32) -> Status {
        match value {
            0 => Status::NotReady,
            1 => Status::Idle,
            2 => Status::Send,
            3 => Status::Receive,
            _ => panic!("failed to convert from u32 into Status"),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match *self {
            Status::NotReady => 0,
            Status::Idle => 1,
            Status::Send => 2,
            Status::Receive => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum TestStatus {
    NotRunning,
    Running { should_panic: bool },
}

impl TestStatus {
    pub fn is_running(&self) -> bool {
        match *self {
            TestStatus::NotRunning => false,
            TestStatus::Running { .. } => true,
        }
    }
}

impl core::convert::TryInto<u32> for TestStatus {
    type Error = Error;

    fn try_into(self) -> Result<u32, Error> {
        match self {
            TestStatus::NotRunning => Ok(0),
            TestStatus::Running {
                should_panic: false,
            } => Ok(1),
            TestStatus::Running { should_panic: true } => Ok(2),
        }
    }
}

impl core::convert::TryFrom<u32> for TestStatus {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(TestStatus::NotRunning),
            1 => Ok(TestStatus::Running {
                should_panic: false,
            }),
            2 => Ok(TestStatus::Running { should_panic: true }),
            _ => Err(Error::BufferOverflow), // todo: not buffer overflow of course
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    Io,
    BufferOverflow,
    UnexpectedEvent,
    Encoding,
    Decoding,
    IllegalStatus { actual: Status, expected: Status },
    NoTestRunning,
}

pub trait Runtime {
    fn status(&mut self) -> Result<Status, Error>;
    fn set_status(&mut self, status: Status) -> Result<(), Error>;
    fn test_status(&mut self) -> Result<TestStatus, Error>;
    fn encode_event(&mut self, event: Event) -> Result<(), Error>;
    fn decode_event(&mut self) -> Result<Event, Error>;

    fn await_status(&mut self, status: Status) -> Result<(), Error> {
        while self.status()? != status {}
        Ok(())
    }

    fn request(&mut self, event: Event) -> Result<Event, Error> {
        self.await_status(Status::Idle)?;
        self.encode_event(event)?;
        self.set_status(Status::Send)?;
        self.await_status(Status::Receive)?;
        let event = self.decode_event()?;
        Ok(event)
    }

    fn complete_request(&mut self) -> Result<(), Error> {
        self.set_status(Status::Idle)
    }

    fn respond(&mut self, event: Event) -> Result<(), Error> {
        self.await_status(Status::Send)?;
        self.encode_event(event)?;
        self.set_status(Status::Receive)?;
        Ok(())
    }

    fn read(&mut self) -> Result<Event, Error> {
        self.await_status(Status::Send)?;
        self.decode_event()
    }

    fn try_read(&mut self) -> Result<Option<Event>, Error> {
        match self.status()? {
            Status::Send => Ok(Some(self.decode_event()?)),
            _ => Ok(None),
        }
    }
}
