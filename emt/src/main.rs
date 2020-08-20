mod runner;

use runner::{probe, Runner};

fn main() {
    let mut _runner = probe::Runner::attach().expect("failed to attach probe runner");
    let _meta = _runner.meta();
}
