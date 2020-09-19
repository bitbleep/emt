#![no_std]

pub mod link;
pub mod runtime;

pub struct Test {
    pub name: &'static str,
    pub description: &'static str,
    pub requires_human_interaction: bool,
    pub should_panic: bool,
    pub timeout_ms: u32,
}
