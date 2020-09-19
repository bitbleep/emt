#![no_std]

mod runtime_block;

use common::runtime::{Event, Meta, Runtime};
use runtime_block::RuntimeBlock;

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock::new();

pub struct Test {
    pub meta: common::Test,
    pub run: fn(),
}

pub fn start(id: &'static str, version: &'static str, tests: &'static [Test]) -> ! {
    let _runtime_meta = Meta {
        id,
        version,
        num_tests: tests.len() as u32,
    };

    unsafe {
        EMT_RUNTIME_BLOCK.init();
        loop {
            if let Err(err) = poll_runtime(&mut EMT_RUNTIME_BLOCK) {
                panic!("runtime error: {:?}", err);
            }
        }
    }
}

#[inline(always)]
fn poll_runtime(runtime_block: &mut RuntimeBlock) -> Result<(), Error> {
    match runtime_block.read()? {
        Event::MetaRequest => {
            let meta_response = Event::MetaResponse;
            runtime_block.respond(meta_response)?;
        }
        _ => return Err(Error::UnexpectedEvent),
    }
    Ok(())
}

#[derive(Debug)]
enum Error {
    UnexpectedEvent,
    RuntimeError(common::runtime::Error),
}

impl core::convert::From<common::runtime::Error> for Error {
    fn from(err: common::runtime::Error) -> Self {
        Error::RuntimeError(err)
    }
}
