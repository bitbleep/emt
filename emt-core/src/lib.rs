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
    Skip,
}

impl TestResult {
    /// Returns `true` if the test result represents a passing test.
    pub fn did_pass(&self) -> bool {
        *self == TestResult::Pass
    }

    /// Returns `true` if the test result represents a skipped test.
    pub fn did_skip(&self) -> bool {
        *self == TestResult::Skip
    }

    /// Returns `true` if the test result represents a failing test.
    pub fn did_fail(&self) -> bool {
        !self.did_pass() && !self.did_skip()
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
            5 => TestResult::Skip,
            _ => panic!("failed to convert from u32 into TestResult"),
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
            TestResult::Skip => 5,
        }
    }
}
