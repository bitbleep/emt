mod tests;

use std::time::Duration;

use common::runtime::Event;

use tests::list_tests;

fn main() {
    std::thread::spawn(|| {
        runtime::start("emt example tests", "1.0.0", list_tests());
    });
    std::thread::sleep(Duration::from_millis(100));
    println!("injecting: meta request");
    let event = runtime::inject(Event::MetaRequest).expect("failed to inject meta request");
    println!("received: {:?}", event);
}
