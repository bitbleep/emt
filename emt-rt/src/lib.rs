#![no_std]

//! Runtime for embedded tests.
//!

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

pub use common::runtime::{Error, Meta, Test};
pub use common::test::{self, Context};

use common::runtime::{Event, Runtime, RuntimeBlock, TestStatus};

/// Tests two values for equality.
///
/// Use this macro for all test comparisons.
#[macro_export]
macro_rules! test_eq {
    ($lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            emt_rt::fail();
        }
    };
}

#[no_mangle]
static mut EMT_RUNTIME_BLOCK: RuntimeBlock = RuntimeBlock::new();

/// Starts the runtime.
pub fn start(id: &'static str, version: &'static str, tests: &'static [Test]) -> ! {
    let runtime_meta = Meta {
        id,
        version,
        num_tests: tests.len() as u32,
    };

    unsafe {
        EMT_RUNTIME_BLOCK.init();
        loop {
            if let Err(err) = EMT_RUNTIME_BLOCK.poll(runtime_meta, tests) {
                panic!("runtime error: {:?}", err);
            }
        }
    }
}

/// Fails the currently running test.
#[inline(always)]
// todo: can i make this not be pub
pub fn fail() {
    unsafe {
        // todo: state should be: test running
        EMT_RUNTIME_BLOCK
            .request(Event::Result(test::Result::AssertionFail))
            .expect("runtime request failed");
        EMT_RUNTIME_BLOCK.complete_request();
        EMT_RUNTIME_BLOCK.end_test();
    }
    panic!("fail()");
}

/// Outputs a message from the runtime to the test runner.
#[inline(always)]
pub fn output<'a>(message: &'a str) {
    unsafe {
        // todo: state should be: test running
        EMT_RUNTIME_BLOCK
            .request(Event::Output(message))
            .expect("runtime request failed");
        EMT_RUNTIME_BLOCK.complete_request();
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    unsafe {
        match EMT_RUNTIME_BLOCK.test_status() {
            TestStatus::Running { should_panic } => {
                let result = match should_panic {
                    true => test::Result::Pass,
                    false => test::Result::Panic,
                };
                let result_response = Event::Result(result);
                EMT_RUNTIME_BLOCK.request(result_response).ok();
                EMT_RUNTIME_BLOCK.complete_request();
                EMT_RUNTIME_BLOCK.end_test();
            }
            TestStatus::NotRunning => {}
        }
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
