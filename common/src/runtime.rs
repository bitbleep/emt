#[derive(Debug, Copy, Clone)]
pub enum Error {
    IllegalStatus { actual: Status, expected: Status },
}

pub struct Meta {
    pub id: &'static str,
    pub version: &'static str,
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

// todo: this should be a safer setup of TryFrom/Into etc.
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Event {
    None = 0,
    MetaRequest,
    MetaResponse,
    Test,
}

pub trait Runtime {
    fn status(&self) -> Status;
    fn set_status(&mut self, status: Status);
    fn request(&mut self, event: Event) -> Result<Event, Error>;
    fn respond(&mut self, event: Event) -> Result<(), Error>;
    fn read(&mut self) -> Result<Event, Error>;
}
