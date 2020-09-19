use common::test::Context;
use runtime::Test;

const TESTS: [Test; 2] = [
    Test {
        context: Context {
            name: "blah",
            description: "Do some garbage testing",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: blah,
    },
    Test {
        context: Context {
            name: "should_fail",
            description: "This test should not pass",
            requires_human_interaction: false,
            should_panic: false,
            timeout_ms: 500,
        },
        run: should_fail,
    },
];

pub fn list_tests<'a>() -> &'a [Test<'a>] {
    &TESTS
}

fn blah() -> bool {
    runtime::output("hello from blah");
    runtime::assert_eq(true, true)
}

fn should_fail() -> bool {
    runtime::assert_eq(false, true)
}
