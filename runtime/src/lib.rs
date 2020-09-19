#![no_std]

mod runtime_block;

use common::runtime::{Event, Meta, Runtime};
use common::test::Context;
use runtime_block::RuntimeBlock;

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock::new();

pub struct Test<'a> {
    pub context: Context<'a>,
    pub run: fn(),
}

pub fn start(id: &'static str, version: &'static str, tests: &'static [Test]) -> ! {
    let runtime_meta = Meta {
        id,
        version,
        num_tests: tests.len() as u32,
    };

    unsafe {
        EMT_RUNTIME_BLOCK.init();
        loop {
            if let Err(err) = poll_runtime(&mut EMT_RUNTIME_BLOCK, runtime_meta) {
                panic!("runtime error: {:?}", err);
            }
        }
    }
}

/// For testing purposes.
pub fn inject(event: Event) -> Result<Event, common::runtime::Error> {
    unsafe { EMT_RUNTIME_BLOCK.request(event) }
}

#[inline(always)]
fn poll_runtime(runtime_block: &mut RuntimeBlock, meta: Meta) -> Result<(), Error> {
    match runtime_block.read()? {
        Event::MetaRequest => {
            let meta_response = Event::Meta(meta);
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
