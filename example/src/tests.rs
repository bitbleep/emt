use nb::block;
use nrf52832_hal::{pac::Peripherals, prelude::*, timer::Timer};

use emt_rt::*;

pub fn always_pass() -> bool {
    true
}

pub fn always_fail() -> bool {
    false
}

pub fn always_panic() -> bool {
    panic!("oh, my");
}

pub fn timer_wait() -> bool {
    let board_peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(board_peripherals.TIMER0);
    emt_rt::output("starting TIMER0");
    timer.start(1000000_u32);
    block!(timer.wait()).unwrap();
    true
}

pub fn button_wait() -> bool {
    let board_peripherals = Peripherals::take().unwrap();
    let p0 = nrf52832_hal::gpio::p0::Parts::new(board_peripherals.P0);
    let button1 = p0.p0_13.into_pullup_input().degrade();
    emt_rt::output("now press button 1");
    while button1.is_high().expect("failed to read gpio pin") {}
    true
}
