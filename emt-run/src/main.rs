use structopt::StructOpt;

use emt::{
    link::{Hosted, Probe},
    runner::{DeviceLink, Runner, TestReport},
};

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-run")]
pub struct RunOptions {
    #[structopt(short = "l", long = "link", default_value = "probe")]
    pub link: String,

    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,

    /// Skip tests that require human interaction
    #[structopt(short = "n", long)]
    pub no_human_interaction: bool,
}

fn main() {
    let opt = RunOptions::from_args();

    match opt.link.to_lowercase().as_str() {
        "probe" => run(Probe::new().expect("failed to attach probe")),
        "hosted" => run(Hosted::new(&opt.domain, opt.port).expect("failed to connect to host")),
        link_name => panic!("unknown link type {}", link_name),
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
