#![no_std]

#[repr(C)]
struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: u32,
    event_id: u32,
    data_size: u32,
    data: [u8; 488],
}

static mut RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock {
    magic_sequence: [
        0x45, 0x4d, 0x54, 0x2d, 0x52, 0x55, 0x4e, 0x54, 0x49, 0x4d, 0x45, 0x00,
    ],
    status: 0,
    event_id: 0,
    data_size: 0,
    data: [0u8; 488],
};

pub fn start() -> ! {
    unsafe {
        RUNTIME_BLOCK.magic_sequence[11] = 0x20;
    }
    loop {
        // todo: needs to actually do something
        // process event
    }
}
