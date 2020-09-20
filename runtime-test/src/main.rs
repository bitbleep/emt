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
    runtime::complete_request();

    for id in 0..num_tests {
        println!("injecting: test {}", id);
        match runtime::inject(Event::Test(id)).expect("failed to inject test") {
            Event::Context(context) => eprintln!("{}: {}", context.name, context.description),
            _ => panic!("unexpected event"),
        }
        runtime::complete_request();

        let mut done = false;
        loop {
            match runtime::read().expect("failed to read from runtime") {
                Event::Output(message) => eprintln!("{}", message),
                Event::Result(result) => {
                    match result.did_pass {
                        true => eprintln!("PASS"),
                        false => eprintln!("FAIL"),
                    }
                    done = true;
                }
                _ => panic!("unexpected event"),
            }
            runtime::respond(Event::None).expect("failed to respond to runtime");
            if done {
                break;
            }
        }
        // todo: timeout
    }

    eprintln!("all done");
}
