#![no_std]
#![no_main]

mod tests;

extern crate panic_rtt;

use nrf52832_hal::{
    pac::{Peripherals, TIMER0},
    timer::Timer,
};

/// These are the resources used by all tests in this test firmware.
pub struct Resources {
    timer: Timer<TIMER0>,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let board_peripherals = Peripherals::take().unwrap();
    let mut resources = Resources {
        timer: Timer::new(board_peripherals.TIMER0),
    };

    tests::simple_wait(&mut resources);

    loop {
        cortex_m::asm::wfi();
    }
}
