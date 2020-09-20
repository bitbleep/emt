#![no_std]

mod runtime_block;

use common::runtime::{Event, Meta, Runtime};
use common::test::{self, Context};
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
        EMT_RUNTIME_BLOCK.complete_request();
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

/// For testing purposes.
pub fn read() -> Result<Event<'static>, common::runtime::Error> {
    unsafe { EMT_RUNTIME_BLOCK.read() }
}

/// For testing purposes.
pub fn respond(event: Event) -> Result<(), common::runtime::Error> {
    unsafe { EMT_RUNTIME_BLOCK.respond(event) }
}

/// For testing purposes.
pub fn complete_request() {
    unsafe { EMT_RUNTIME_BLOCK.complete_request() }
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
            if id < tests.len() {
                let test = &tests[id];
                let context_response = Event::Context(test.context);
                runtime_block.respond(context_response)?;
                let did_pass = (test.run)();
                let result_response = Event::Result(test::Result { did_pass });
                runtime_block.request(result_response)?;
                runtime_block.complete_request();
            } else {
                // todo: should use a separate status for this
                let result_response = Event::Result(test::Result { did_pass: false });
                runtime_block.request(result_response)?;
            }
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
