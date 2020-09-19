#![no_std]

mod runtime_block;

use common::runtime::{Event, Meta, Runtime, Status};
use runtime_block::RuntimeBlock;

pub struct Test {
    pub meta: common::Test,
    pub run: fn(),
}

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock {
    magic_sequence: [0u8; 12],
    status: Status::NotReady,
    event_id: Event::None,
    data_size: 0,
    data: [0u8; 488],
};

pub fn start(id: &'static str, version: &'static str, tests: &'static [Test]) -> ! {
    let _runtime_meta = Meta {
        id,
        version,
        num_tests: tests.len() as u32,
    };

    unsafe {
        EMT_RUNTIME_BLOCK.init();

        loop {
            let event = EMT_RUNTIME_BLOCK
                .read()
                .expect("failed to receive from runtime");

            match event {
                Event::Meta => unimplemented!("Meta"),
                Event::Test => unimplemented!("Test"),
                _ => panic!("received an unexpected event from runtime"),
            }
        }
    }
}
