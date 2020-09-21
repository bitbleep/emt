use common::runtime;

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

pub trait Runner {
    fn meta(&mut self) -> &RuntimeMeta;
    fn start(&mut self, id: u32) -> Result<(), Error>;
}
