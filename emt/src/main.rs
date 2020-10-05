mod cli;
mod runner;

use structopt::StructOpt;

use runner::{probe, Runner, TestReport};

fn main() {
    let run_options = cli::RunOptions::from_args();

    let mut runner = probe::Runner::attach().expect("failed to attach probe runner");
    let mut report = TestReport::new();

    let meta = runner.meta();
    println!(
        "Attached to runtime {} {} containing {} test(s)",
        meta.id, meta.version, meta.num_tests
    );

    for id in 0..meta.num_tests {
        let result = runner.run(id, &run_options).expect("failed to start test");
        report.append_result(result);
    }

    println!(
        "Result: passed: {}, failed: {}, skipped: {}",
        report.passed(),
        report.failed(),
        report.skipped(),
    );
}
