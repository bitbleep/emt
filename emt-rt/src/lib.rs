#![no_std]

//! Runtime for embedded tests.
//!
//! Include this crate and call the `emt_rt::start` function.
//!
//! Note that this crate contains a panic handler which is necessary for the runtime to work properly.

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

pub use emt_core::{
    runtime::{Error, Meta},
    Test, TestContext, TestResult,
};

use emt_core::runtime::RuntimeBlock;

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
                panic!("fatal runtime error: {:?}", err);
            }
        }
    }
}

/// Fails the currently running test.
#[inline(always)]
pub fn fail() {
    unsafe {
        if let Err(err) = EMT_RUNTIME_BLOCK.fail_test() {
            panic!("fatal runtime error: {:?}", err);
        }
        panic!("test fail");
    }
}

/// Outputs a message from the runtime to the test runner.
#[inline(always)]
pub fn output(message: &str) {
    unsafe {
        if let Err(err) = EMT_RUNTIME_BLOCK.output(message) {
            panic!("fatal runtime error: {:?}", err);
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    unsafe {
        // we really don't really care about the
        // result of this because we can't do
        // anything about it anyway
        EMT_RUNTIME_BLOCK.handle_panic().ok();
    }

    loop {
        cortex_m::asm::wfi();
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
