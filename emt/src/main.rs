pub mod cli;
pub mod host;
pub mod link;
pub mod runner;

use structopt::StructOpt;

use crate::{
    cli::CliOptions,
    link::{Hosted, Probe},
    runner::{DeviceLink, Runner, TestReport},
};

fn main() {
    let opt = CliOptions::from_args();

    match opt {
        CliOptions::Run(opt) => match opt.link.to_lowercase().as_str() {
            "probe" => run(Probe::new().expect("failed to attach probe")),
            "hosted" => run(Hosted::new(&opt.domain, opt.port).expect("failed to connect to host")),
            _ => panic!("unsupported link: supported values are probe or hosted"),
        },
        CliOptions::Host(opt) => futures::executor::block_on(async {
            crate::host::run(opt).expect("failed to start server");
        }),
    }
}

fn run<T>(link: T)
where
    T: DeviceLink,
{
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
