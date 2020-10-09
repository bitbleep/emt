use structopt::StructOpt;

use emt::{
    link::Hosted,
    runner::{Runner, TestReport},
};

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    /// Skip tests that require human interaction
    #[structopt(short, long)]
    pub no_human_interaction: bool,
}

fn main() {
    let opt = RunOptions::from_args();

    let link = Hosted::new("http://localhost:8080").expect("failed to connect to host");
    // let link = Probe::new().expect("failed to attach probe");
    let mut runner = Runner::new(link);
    let mut report = TestReport::new();

    let meta = runner.meta().expect("failed to get runtime meta");
    println!(
        "Attached to runtime {} {} containing {} test(s)",
        meta.id, meta.version, meta.num_tests
    );

    for id in 0..meta.num_tests {
        let result = runner.run(id).expect("failed to run test");
        report.append_result(result);
    }

    println!(
        "Result: passed: {}, failed: {}, skipped: {}",
        report.passed(),
        report.failed(),
        report.skipped(),
    );
}
