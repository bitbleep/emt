use common::runtime;
use common::test::Context;

pub mod in_memory;
pub mod probe;

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

pub trait Runner {
    fn meta(&mut self) -> &RuntimeMeta;
    fn start(&mut self, id: u32) -> Result<Context, Error>;
}
