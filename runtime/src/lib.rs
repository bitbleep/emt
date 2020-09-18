#![no_std]

use common::link::{self, Buffer, Event, Link, Read, Write};

pub struct Test {
    pub meta: common::Test,
    pub run: fn(),
}

#[repr(C)]
struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: u32, // todo: should be an enum
    event_id: u32,
    data_size: u32,
    data: [u8; 488],
}

impl RuntimeBlock {
    fn init(&mut self) {
        self.magic_sequence = *b"EMT-RUNTIME ";
        self.status = 1;
    }
}

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock {
    magic_sequence: [0u8; 12],
    status: 0,
    event_id: 0,
    data_size: 0,
    data: [0u8; 488],
};

struct LinkIo {}

impl LinkIo {
    fn new() -> Self {
        Self {}
    }
}

impl link::Buffer for LinkIo {
    fn buf(&self) -> &[u8] {
        unimplemented!();
    }

    fn mut_buf(&mut self) -> &mut [u8] {
        unimplemented!();
    }
}

impl link::Read for LinkIo {
    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, link::Error> {
        // core::intrinsics::volatile_load()
        unimplemented!();
    }
}

impl link::Write for LinkIo {
    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, link::Error> {
        unimplemented!();
    }
}

pub fn start(id: &'static str, version: &'static str, tests: &'static [Test]) -> ! {
    let runtime_meta = common::Meta {
        id,
        version,
        num_tests: tests.len() as u32,
    };

    let mut link = unsafe {
        EMT_RUNTIME_BLOCK.init();
        link::Link::new(0, LinkIo::new()) // todo: 0 should be address of EMT_RUNTIME_BLOCK
    };

    loop {
        let event = link.receive().expect("failed to receive from link");
        match event {
            Event::Meta => unimplemented!("Meta"),
            Event::Test => unimplemented!("Test"),
            _ => panic!("received an unexpected event from link"),
        }
    }
}
