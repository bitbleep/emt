mod runner;

use runner::{probe, Runner};

fn main() {
    let mut runner = probe::Runner::attach().expect("failed to attach probe runner");

    let meta = runner.meta();
    println!(
        "running {} {} containing {} test(s)",
        meta.id, meta.version, meta.num_tests
    );

    for id in 0..meta.num_tests {
        let _test = runner.start(id).expect("failed to start test");
    }
}
