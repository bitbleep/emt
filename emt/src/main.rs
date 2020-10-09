mod cli;
mod host;
mod link;
mod runner;

use structopt::StructOpt;

use link::{Hosted, Probe};
use runner::{Runner, TestReport};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = cli::HostOptions::from_args();
    let base_url = format!("{}:{}", opt.domain, opt.port);
    let probe = Probe::new().expect("failed to attach probe");
    host::start(&base_url, probe).await
}

// fn main() {
//     let run_options = cli::RunOptions::from_args();

//     let link = Hosted::new("http://localhost:8080").expect("failed to connect to host");
//     // let link = Probe::new().expect("failed to attach probe");
//     let mut runner = Runner::new(link);
//     let mut report = TestReport::new();

//     let meta = runner.meta().expect("failed to get runtime meta");
//     println!(
//         "Attached to runtime {} {} containing {} test(s)",
//         meta.id, meta.version, meta.num_tests
//     );

//     for id in 0..meta.num_tests {
//         let result = runner.run(id, &run_options).expect("failed to run test");
//         report.append_result(result);
//     }

//     println!(
//         "Result: passed: {}, failed: {}, skipped: {}",
//         report.passed(),
//         report.failed(),
//         report.skipped(),
//     );
// }
