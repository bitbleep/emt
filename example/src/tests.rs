use nb::block;
use nrf52832_hal::{pac::Peripherals, prelude::*, timer::Timer};

use common::test::Context;
use runtime::Test;

const TESTS: [Test; 1] = [Test {
    context: Context {
        name: "blah",
        description: "Do some garbage testing",
        requires_human_interaction: false,
        should_panic: false,
        timeout_ms: 500,
    },
    run: blah,
}];

pub fn list_tests() -> &'static [Test] {
    &TESTS
}

fn blah() {
    unimplemented!();
}

// #[derive(Debug)]
// pub enum Error {
//     LifeIsNotFair,
// }

// pub fn simple_wait() -> Result<(), Error> {
//     let board_peripherals = Peripherals::take().unwrap();
//     let mut timer = Timer::new(board_peripherals.TIMER0);
//     timer.start(1000000_u32);
//     block!(timer.wait()).unwrap();
//     Ok(())
// }

// pub fn will_fail() -> Result<(), Error> {
//     Err(Error::LifeIsNotFair)
// }
