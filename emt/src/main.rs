pub mod cli;
pub mod host;
pub mod link;
pub mod runner;

use structopt::StructOpt;

use crate::{
    cli::CliOptions,
    link::{Hosted, Link, Probe},
    runner::{Runner, TestReport},
};

fn main() {
    let opt = CliOptions::from_args();

    match opt {
        CliOptions::Run(opt) => match opt.link.to_lowercase().as_str() {
            "probe" => run(
                Probe::new(opt.probe_id, &opt.probe_target).expect("failed to attach probe"),
                opt.no_human_interaction,
            ),
            "hosted" => run(
                Hosted::new(&opt.domain, opt.port).expect("failed to connect to host"),
                opt.no_human_interaction,
            ),
            _ => panic!("unsupported link: supported values are probe or hosted"),
        },
        CliOptions::Host(opt) => futures::executor::block_on(async {
            crate::host::run(opt).expect("failed to start server");
        }),
    }
}

fn run<T>(link: T, no_human_interaction: bool)
where
    T: Link,
{
    let mut runner = Runner::new(link);
    let mut report = TestReport::new();

    let meta = match runner.meta() {
        Ok(meta) => meta,
        Err(err) => {
            eprintln!("Failed to get runtime meta: {:?}", err);
            std::process::exit(1);
        }
    };

    println!(
        "Attached to runtime {} {} containing {} test(s)",
        meta.id, meta.version, meta.num_tests
    );

    for id in 0..meta.num_tests {
        let result = match runner.run(id, no_human_interaction) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Failed to get run test {}: {:?}", id, err);
                std::process::exit(1);
            }
        };
        report.append_result(result);
    }

    println!(
        "Result: passed: {}, failed: {}, skipped: {}",
        report.passed(),
        report.failed(),
        report.skipped(),
    );

    match report.failed() {
        failed if failed > 0 => std::process::exit(1),
        _ => std::process::exit(0),
    }
}
