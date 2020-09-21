#![no_std]

mod runtime_block;

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

use cortex_m::interrupt;

pub use common::runtime::{Event, Meta, Runtime};
pub use common::test::{self, Context};

use runtime_block::RuntimeBlock;

/// Syntactic sugar for your test assertions.
#[macro_export]
macro_rules! emt_assert_eq {
    ($lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            return false;
        }
    };
}

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock::new();
static mut EMT_TEST_STATE: TestState = TestState {
    is_running: false,
    should_panic: false,
};

pub struct Test<'a> {
    pub context: Context<'a>,
    pub run: fn() -> bool,
}

struct TestState {
    is_running: bool,
    should_panic: bool,
}

impl TestState {
    fn begin(&mut self, should_panic: bool) {
        self.is_running = true;
        self.should_panic = should_panic;
    }

    fn end(&mut self) {
        self.is_running = false;
    }
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

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    unsafe {
        if EMT_TEST_STATE.is_running {
            let did_pass = EMT_TEST_STATE.should_panic;
            let result_response = Event::Result(test::Result { did_pass });
            EMT_RUNTIME_BLOCK.request(result_response);
            EMT_RUNTIME_BLOCK.complete_request();
            unsafe {
                EMT_TEST_STATE.end();
            }
        }
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
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
                unsafe {
                    EMT_TEST_STATE.begin(test.context.should_panic);
                }
                let context_response = Event::Context(test.context);
                runtime_block.respond(context_response)?;
                let did_pass = (test.run)();
                let result_response = Event::Result(test::Result { did_pass });
                runtime_block.request(result_response)?;
                runtime_block.complete_request();
                unsafe {
                    EMT_TEST_STATE.end();
                }
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
