use super::Resources;
use nb::block;
use nrf52832_hal::prelude::*;

#[derive(Debug)]
pub enum Error {
    LifeIsNotFair,
}

pub fn simple_wait(resources: &mut Resources) -> Result<(), Error> {
    resources.timer.start(1000000_u32);
    block!(resources.timer.wait()).unwrap();
    Ok(())
}

pub fn will_fail(_resources: &mut Resources) -> Result<(), Error> {
    Err(Error::LifeIsNotFair)
}
