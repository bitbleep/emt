#[derive(Debug, Copy, Clone)]
pub struct Context<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Result {
    Pass,
    NotFound,
    AssertionFail,
    Panic,
    Timeout,
}

impl Result {
    pub fn did_pass(&self) -> bool {
        *self == Result::Pass
    }

    pub fn did_fail(&self) -> bool {
        !self.did_pass()
    }
}

impl core::convert::From<u32> for Result {
    fn from(value: u32) -> Self {
        match value {
            0 => Result::Pass,
            1 => Result::NotFound,
            2 => Result::AssertionFail,
            3 => Result::Panic,
            4 => Result::Timeout,
            _ => panic!("failed to convert from Result into u32"),
        }
    }
}

impl core::convert::Into<u32> for Result {
    fn into(self) -> u32 {
        match self {
            Result::Pass => 0,
            Result::NotFound => 1,
            Result::AssertionFail => 2,
            Result::Panic => 3,
            Result::Timeout => 4,
        }
    }
}
