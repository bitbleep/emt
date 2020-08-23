#![no_std]

use core::time::Duration;

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
    pub description: &'static str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout: Duration,
}

pub enum Event {}

pub enum Error {}

pub trait MemoryIo {
    fn send(&self, event: Event) -> Result<(), Error>;
    fn receive(&self) -> Result<(), Error>;
    fn poll(&self) -> Result<(), Error>;
}
