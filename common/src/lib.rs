#![no_std]

pub mod runtime;

#[derive(Debug, Copy, Clone)]
pub struct TestContext<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}

pub struct Test<'a> {
    pub context: TestContext<'a>,
    pub run: fn(),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TestResult {
    Pass,
    NotFound,
    AssertionFail,
    Panic,
    Timeout,
}

impl TestResult {
    pub fn did_pass(&self) -> bool {
        *self == TestResult::Pass
    }

    pub fn did_fail(&self) -> bool {
        !self.did_pass()
    }
}

impl core::convert::From<u32> for TestResult {
    fn from(value: u32) -> Self {
        match value {
            0 => TestResult::Pass,
            1 => TestResult::NotFound,
            2 => TestResult::AssertionFail,
            3 => TestResult::Panic,
            4 => TestResult::Timeout,
            _ => panic!("failed to convert from TestResult into u32"),
        }
    }
}

impl core::convert::Into<u32> for TestResult {
    fn into(self) -> u32 {
        match self {
            TestResult::Pass => 0,
            TestResult::NotFound => 1,
            TestResult::AssertionFail => 2,
            TestResult::Panic => 3,
            TestResult::Timeout => 4,
        }
    }
}
