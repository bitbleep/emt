#![no_std]
#![no_main]

mod tests;

use common::test::Context;
use runtime::*;

const TESTS: [Test; 6] = [
    Test {
        context: Context {
            name: "always_pass",
            description: "A test that always passes",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: tests::always_pass,
    },
    Test {
        context: Context {
            name: "always_fail",
            description: "A test that always fails",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: tests::always_fail,
    },
    Test {
        context: Context {
            name: "always_panic",
            description: "A test that always panics, but that's ok",
            requires_human_interaction: false,
            should_panic: true,
            timeout_ms: 500,
        },
        run: tests::always_panic,
    },
    Test {
        context: Context {
            name: "always_panic",
            description: "A test that always panics, and it's not ok",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: tests::always_panic,
    },
    Test {
        context: Context {
            name: "timer_wait",
            description: "Start a timer and wait for it to finish",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: tests::timer_wait,
    },
    Test {
        context: Context {
            name: "button_wait",
            description: "Wait for the user to push button 1 on the devkit",
            requires_human_interaction: true,
            should_panic: false,
            timeout_ms: 30000,
        },
        run: tests::button_wait,
    },
];

#[cortex_m_rt::entry]
fn main() -> ! {
    runtime::start("emt example tests", "1.0.0", &TESTS);
}
