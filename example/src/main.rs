#![no_std]
#![no_main]

mod tests;

extern crate panic_rtt;

use nrf52832_hal::{gpio::Level, pac::Peripherals, prelude::*, timer::Timer};
use tests::list_tests;

#[cortex_m_rt::entry]
fn main() -> ! {
    // let board_peripherals = Peripherals::take().unwrap();
    // let p0 = nrf52832_hal::gpio::p0::Parts::new(board_peripherals.P0);
    // let _ = p0.p0_17.into_push_pull_output(Level::Low).degrade();
    runtime::start("emt example tests", "1.0.0", list_tests());
}
