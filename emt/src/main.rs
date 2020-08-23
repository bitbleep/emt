mod runner;

use runner::{probe, Runner};

fn main() {
    let mut runner = probe::Runner::attach().expect("failed to attach probe runner");

    let meta = runner.meta();
    println!("running {} test(s)", meta.tests());

    for id in 0..meta.tests() {
        let _test = runner.start(id).expect("failed to start test");
    }

    // foreach test:
    //   runner.start(id)?;
    //   loop and process events until either completion or timeout occurs
    // print test report
}
