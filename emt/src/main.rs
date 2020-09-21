mod runner;

use runner::{probe, Runner, TestReport};

fn main() {
    let mut runner = probe::Runner::attach().expect("failed to attach probe runner");
    let mut report = TestReport::new();

    let meta = runner.meta();
    println!(
        "running {} {} containing {} test(s)",
        meta.id, meta.version, meta.num_tests
    );

    for id in 0..meta.num_tests {
        let result = runner.run(id).expect("failed to start test");
        report.append_result(result);
    }

    println!(
        "passed: {}, failed: {}, skipped: {}",
        report.passed(),
        report.failed(),
        report.skipped()
    );
}
