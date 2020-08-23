#![no_std]

pub mod link;

pub struct Meta {
    pub firmware_id: &'static str,
    pub version: &'static str,
    tests: u32,
}

impl Meta {
    pub fn tests(&self) -> u32 {
        self.tests
    }
}

pub struct Test {
    pub name: &'static str,
    pub description: &'static str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}
