use std::convert::TryFrom;
use std::time::{Duration, Instant};

use colored::*;

use rt_core::{
    runtime::{self, decode_u32, encode_u32, Event, Runtime, Status, TestStatus},
    TestResult,
};

#[derive(Debug, Copy, Clone)]
pub enum Error {
    AttachFailed,
    NoRuntime,
    RuntimeError(runtime::Error),
}

impl From<runtime::Error> for Error {
    fn from(error: runtime::Error) -> Self {
        Self::RuntimeError(error)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeMeta {
    pub id: String,
    pub version: String,
    pub num_tests: u32,
}

#[derive(Debug, Clone)]
pub struct TestContext {
    pub name: String,
    pub description: String,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}

#[derive(Debug, Clone)]
pub struct TestReport {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl TestReport {
    pub fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            skipped: 0,
        }
    }

    pub fn passed(&self) -> usize {
        self.passed
    }

    pub fn failed(&self) -> usize {
        self.failed
    }

    pub fn skipped(&self) -> usize {
        self.skipped
    }

    pub fn append_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn append_result(&mut self, result: TestResult) {
        if result.did_pass() {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
    }
}

pub trait DeviceLink {
    fn base_address(&self) -> u32;
    fn reset(&mut self) -> Result<(), Error>;
    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error>;
    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error>;
}

pub struct Runner<T>
where
    T: DeviceLink,
{
    device_link: T,
    io_buf: [u8; 512],
}

impl<T> Runner<T>
where
    T: DeviceLink,
{
    pub fn new(device_link: T) -> Self {
        Self {
            device_link,
            io_buf: [0u8; 512],
        }
    }

    pub fn meta(&mut self) -> Result<RuntimeMeta, Error> {
        print!("waiting for idle runtime.. ");
        while self.status() != Status::Idle {}
        println!("ok");
        let meta = match self.request(Event::MetaRequest)? {
            Event::Meta(meta) => RuntimeMeta {
                id: meta.id.to_string(),
                version: meta.version.to_string(),
                num_tests: meta.num_tests,
            },
            _ => return Err(Error::RuntimeError(runtime::Error::UnexpectedEvent)),
        };
        self.complete_request();
        Ok(meta)
    }

    pub fn run(&mut self, id: u32) -> Result<TestResult, Error> {
        // reset board before every test
        self.device_link.reset()?;

        let context = match self
            .request(Event::Test(id))
            .expect("failed to inject test")
        {
            Event::Context(context) => TestContext {
                name: context.name.to_owned(),
                description: context.description.to_owned(),
                requires_human_interaction: context.requires_human_interaction,
                should_panic: context.should_panic,
                timeout_ms: context.timeout_ms,
            },
            _ => panic!("unexpected event"),
        };
        self.complete_request();

        println!(
            "\n{} {}; {}",
            "Running".bold(),
            context.name,
            context.description
        );

        let start_instant = Instant::now();

        loop {
            let result = match self.try_read() {
                Ok(Some(Event::Output(message))) => {
                    println!("  {}", message);
                    self.respond(Event::None)
                        .expect("failed to respond to runtime");
                    None
                }
                Ok(Some(Event::Result(result))) => {
                    match result.did_pass() {
                        true => println!("  {}", "pass".green()),
                        false => println!("  {} ({:?})", "fail".red(), result),
                    }
                    self.respond(Event::None)
                        .expect("failed to respond to runtime");
                    Some(result)
                }
                Ok(Some(_)) => panic!("unexpected event"),
                Ok(None) => None,
                Err(err) => panic!("failed to read from runtime {:?}", err),
            };

            if let Some(result) = result {
                return Ok(result);
            }

            if start_instant.elapsed() >= Duration::from_millis(context.timeout_ms as u64) {
                let result = TestResult::Timeout;
                println!("  {} ({:?})", "fail".red(), result);
                return Ok(result);
            }
        }
    }
}

const OFFSET_STATUS_ID: usize = 12;
const OFFSET_TEST_STATUS: usize = 16;
const OFFSET_EVENT_ID: usize = 20;
const OFFSET_DATA_SIZE: usize = 24;
const OFFSET_DATA: usize = 32;

impl<T> Runtime for Runner<T>
where
    T: DeviceLink,
{
    fn status(&mut self) -> Status {
        self.device_link
            .read(
                self.device_link.base_address() + OFFSET_STATUS_ID as u32,
                &mut self.io_buf[OFFSET_STATUS_ID..OFFSET_STATUS_ID + 4],
            )
            .unwrap();
        let status_id =
            decode_u32(&self.io_buf[OFFSET_STATUS_ID..]).expect("failed to decode status");
        Status::from_u32(status_id)
    }

    fn set_status(&mut self, status: Status) {
        encode_u32(status.to_u32(), &mut self.io_buf[OFFSET_STATUS_ID..])
            .expect("failed to encode status");
        self.device_link
            .write(
                self.device_link.base_address() + OFFSET_STATUS_ID as u32,
                &mut self.io_buf[OFFSET_STATUS_ID..OFFSET_STATUS_ID + 4],
            )
            .unwrap();
    }

    fn test_status(&mut self) -> TestStatus {
        self.device_link
            .read(
                self.device_link.base_address() + OFFSET_TEST_STATUS as u32,
                &mut self.io_buf[OFFSET_TEST_STATUS..OFFSET_TEST_STATUS + 4],
            )
            .unwrap();
        let test_status = decode_u32(&self.io_buf[OFFSET_TEST_STATUS..]).unwrap();
        TestStatus::try_from(test_status).unwrap()
    }

    fn encode_event(&mut self, event: Event) -> Result<(), runtime::Error> {
        let event_id = event.id();
        let data_size = event.encode(&mut self.io_buf[OFFSET_DATA..])? as u32;

        encode_u32(event_id, &mut self.io_buf[OFFSET_EVENT_ID..])
            .expect("failed to encode event id");
        encode_u32(data_size, &mut self.io_buf[OFFSET_DATA_SIZE..])
            .expect("failed to encode data size");

        self.device_link
            .write(
                self.device_link.base_address() + OFFSET_EVENT_ID as u32,
                &mut self.io_buf[OFFSET_EVENT_ID..OFFSET_EVENT_ID + 12 + data_size as usize],
            )
            .unwrap();

        Ok(())
    }

    fn decode_event(&mut self) -> Result<Event, runtime::Error> {
        self.device_link
            .read(
                self.device_link.base_address() + OFFSET_EVENT_ID as u32,
                &mut self.io_buf[OFFSET_EVENT_ID..OFFSET_EVENT_ID + 12],
            )
            .unwrap();

        let event_id = decode_u32(&self.io_buf[OFFSET_EVENT_ID..])?;
        let data_size = decode_u32(&self.io_buf[OFFSET_DATA_SIZE..])? as usize;

        self.device_link
            .read(
                self.device_link.base_address() + OFFSET_DATA as u32,
                &mut self.io_buf[OFFSET_DATA..OFFSET_DATA + data_size],
            )
            .unwrap();

        Event::decode(event_id, &self.io_buf[OFFSET_DATA..OFFSET_DATA + data_size])
    }
}
