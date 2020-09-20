use nb::block;
use nrf52832_hal::{pac::Peripherals, prelude::*, timer::Timer};

pub fn always_pass() -> bool {
    runtime::assert_eq(true, true)
}

pub fn always_fail() -> bool {
    runtime::assert_eq(true, false)
}

pub fn always_panic() -> bool {
    panic!("oh, my");
}

pub fn timer_wait() -> bool {
    let board_peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(board_peripherals.TIMER0);
    runtime::output("starting TIMER0");
    timer.start(1000000_u32);
    block!(timer.wait()).unwrap();
    true
}

pub fn button_wait() -> bool {
    let board_peripherals = Peripherals::take().unwrap();
    let p0 = nrf52832_hal::gpio::p0::Parts::new(board_peripherals.P0);
    let button1 = p0.p0_13.into_pullup_input().degrade();
    runtime::output("now press button 1");
    while button1.is_high().expect("failed to read gpio pin") {}
    true
}
