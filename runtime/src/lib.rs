#![no_std]

mod runtime_block;

use common::runtime::{Event, Meta, Runtime};
use common::test::Context;
use runtime_block::RuntimeBlock;

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock::new();

pub struct Test<'a> {
    pub context: Context<'a>,
    pub run: fn() -> bool,
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
            if let Err(err) = poll_runtime(&mut EMT_RUNTIME_BLOCK, runtime_meta, tests) {
                panic!("runtime error: {:?}", err);
            }
        }
    }
}

#[inline(always)]
pub fn output<'a>(message: &'a str) {
    unsafe {
        EMT_RUNTIME_BLOCK
            .request(Event::Output(message))
            .expect("runtime output failed");
    }
}

#[inline(always)]
pub fn assert_eq<T>(lhs: T, rhs: T) -> bool
where
    T: PartialEq,
{
    lhs == rhs
}

/// For testing purposes.
pub fn inject(event: Event) -> Result<Event, common::runtime::Error> {
    unsafe { EMT_RUNTIME_BLOCK.request(event) }
}

#[inline(always)]
fn poll_runtime(runtime_block: &mut RuntimeBlock, meta: Meta, tests: &[Test]) -> Result<(), Error> {
    match runtime_block.read()? {
        Event::MetaRequest => {
            let meta_response = Event::Meta(meta);
            runtime_block.respond(meta_response)?;
        }
        Event::Test(id) => {
            let id = id as usize;
            if id >= tests.len() {
                // todo: Result: FAIL
            }
            let test = &tests[id];
            let context_response = Event::Context(test.context);
            runtime_block.respond(context_response)?;
            (test.run)();
            // encode the context and respond
            // run the test
            // output the result
            unimplemented!();
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
