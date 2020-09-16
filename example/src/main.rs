#![no_std]
#![no_main]

mod tests;

extern crate panic_rtt;

#[cortex_m_rt::entry]
fn main() -> ! {
    runtime::start();
}
