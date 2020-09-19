use common::test::Context;
use runtime::Test;

const TESTS: [Test; 1] = [Test {
    context: Context {
        name: "blah",
        description: "Do some garbage testing",
        requires_human_interaction: false,
        should_panic: false,
        timeout_ms: 500,
    },
    run: blah,
}];

pub fn list_tests<'a>() -> &'a [Test<'a>] {
    &TESTS
}

fn blah() -> bool {
    runtime::output("hello from blah");
    runtime::assert_eq(true, true)
}