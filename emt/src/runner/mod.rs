use common::{runtime, TestResult};

use crate::cli::RunOptions;

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

pub trait Runner {
    fn meta(&mut self) -> &RuntimeMeta;
    fn run(&mut self, id: u32, run_options: &RunOptions) -> Result<TestResult, Error>;
}
