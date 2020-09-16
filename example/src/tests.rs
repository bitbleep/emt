use nb::block;
use nrf52832_hal::{pac::Peripherals, prelude::*, timer::Timer};

#[derive(Debug)]
pub enum Error {
    LifeIsNotFair,
}

pub fn simple_wait() -> Result<(), Error> {
    let board_peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(board_peripherals.TIMER0);
    timer.start(1000000_u32);
    block!(timer.wait()).unwrap();
    Ok(())
}

pub fn will_fail() -> Result<(), Error> {
    Err(Error::LifeIsNotFair)
}
