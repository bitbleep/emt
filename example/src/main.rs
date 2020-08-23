#![no_std]
#![no_main]

extern crate panic_rtt;

use nrf52832_hal::{
    clocks::Clocks,
    gpio,
    pac::{interrupt, CorePeripherals, Interrupt, Peripherals, TIMER1},
    saadc::{Saadc, SaadcConfig},
    spim,
    temp::Temp,
    timer::Timer,
    twim::{self, Twim},
    uarte::{Baudrate, Parity, Uarte},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // rtt::init();
    // rtt::set_level(LevelFilter::Debug);

    // info!("device starting");

    let cp = CorePeripherals::take().unwrap();
    let p = Peripherals::take().unwrap();
    // let mut board = Board::new(cp, p);
    // let _clock = Clocks::new(board.CLOCK).set_lfclk_src_rc().start_lfclk();
    // let mut timer = Timer::new(board.TIMER1);

    loop {
        cortex_m::asm::wfi();
    }
}
