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
    let num_tests =
        match runtime::inject(Event::MetaRequest).expect("failed to inject meta request") {
            Event::Meta(meta) => {
                println!(
                    "running <{} {}> containing {} test(s)",
                    meta.id, meta.version, meta.num_tests
                );
                meta.num_tests
            }
            _ => panic!("unexpected event"),
        };

    for id in 0..num_tests {
        println!("injecting: test {}", id);
        let event = runtime::inject(Event::Test(id)).expect("failed to inject test");
        println!("received: {:?}", event);
        // todo: run until timeout or result
    }
}
