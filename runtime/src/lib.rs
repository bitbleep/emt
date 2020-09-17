#![no_std]

#[repr(C)]
struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: u32,
    event_id: u32,
    data_size: u32,
    data: [u8; 488],
}

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock {
    magic_sequence: [0u8; 12],
    status: 0,
    event_id: 0,
    data_size: 0,
    data: [0u8; 488],
};

pub fn start() -> ! {
    unsafe {
        EMT_RUNTIME_BLOCK.magic_sequence = *b"EMT-RUNTIME ";
        loop {
            if EMT_RUNTIME_BLOCK.status == 1 {
                EMT_RUNTIME_BLOCK.status = 2;
            }
            // todo: needs to actually do something
            // process event
        }
    }
}
