use probe_rs::{MemoryInterface, Session};

use common::runtime::{self, decode_u32, encode_u32, Event, Runtime, Status};

use crate::runner::{Error, RuntimeMeta};

pub struct Runner {
    meta: RuntimeMeta,
    link: Link,
}

impl Runner {
    pub fn attach() -> Result<Self, Error> {
        let mut session = Session::auto_attach("nrf52").map_err(|_err| Error::AttachFailed)?;
        eprintln!("auto attached nrf52");

        let base_address = detect_runtime(&mut session, 0x2000_0000, 0x10000)?;
        eprintln!("found base address: 0x{:08x}", base_address);

        let mut link = Link::new(base_address, session);
        let meta = match link.request(Event::MetaRequest)? {
            Event::Meta(meta) => RuntimeMeta {
                id: meta.id.to_string(),
                version: meta.version.to_string(),
                num_tests: meta.num_tests,
            },
            _ => return Err(Error::RuntimeError(runtime::Error::UnexpectedEvent)),
        };
        link.complete_request();

        Ok(Self { meta, link })
    }
}

impl crate::runner::Runner for Runner {
    fn meta(&mut self) -> &RuntimeMeta {
        &self.meta
    }

    fn start(&mut self, id: u32) -> Result<(), Error> {
        // reset board before every test
        self.link.reset();

        println!("injecting: test {}", id);
        match self
            .link
            .request(Event::Test(id))
            .expect("failed to inject test")
        {
            Event::Context(context) => eprintln!("{}: {}", context.name, context.description),
            _ => panic!("unexpected event"),
        }
        self.link.complete_request();

        let mut done = false;
        loop {
            match self.link.read().expect("failed to read from runtime") {
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
            self.link
                .respond(Event::None)
                .expect("failed to respond to runtime");
            if done {
                break;
            }
        }
        // todo: timeout

        Ok(())
    }
}

const OFFSET_STATUS_ID: usize = 12;
const OFFSET_EVENT_ID: usize = 16;
const OFFSET_DATA_SIZE: usize = 20;
const OFFSET_DATA: usize = 24;

struct Link {
    base_address: u32,
    session: Session,
    io_buf: [u8; 512],
}

impl Link {
    fn new(base_address: u32, session: Session) -> Self {
        Self {
            base_address,
            session,
            io_buf: [0u8; 512],
        }
    }

    fn reset(&mut self) {
        self.session
            .core(0)
            .expect("failed to get core")
            .reset()
            .expect("failed to reset");
    }
}

impl common::runtime::Runtime for Link {
    fn status(&mut self) -> Status {
        read(
            &mut self.session,
            self.base_address + OFFSET_STATUS_ID as u32,
            &mut self.io_buf[OFFSET_STATUS_ID..OFFSET_STATUS_ID + 4],
        )
        .expect("waah");
        let status_id =
            decode_u32(&self.io_buf[OFFSET_STATUS_ID..]).expect("failed to decode status");
        // for i in OFFSET_STATUS_ID..OFFSET_STATUS_ID + 4 {
        //     eprint!("{:02x} ", self.io_buf[i]);
        // }
        // eprintln!("status(): {}", status_id);
        Status::from_u32(status_id)
    }

    fn set_status(&mut self, status: Status) {
        encode_u32(status.to_u32(), &mut self.io_buf[OFFSET_STATUS_ID..])
            .expect("failed to encode status");
        write(
            &mut self.session,
            self.base_address + OFFSET_STATUS_ID as u32,
            &mut self.io_buf[OFFSET_STATUS_ID..OFFSET_STATUS_ID + 4],
        )
        .expect("waah");
    }

    fn encode_event(&mut self, event: Event) -> Result<(), runtime::Error> {
        let event_id = event.id();
        let data_size = event.encode(&mut self.io_buf[OFFSET_DATA..])? as u32;

        encode_u32(event_id, &mut self.io_buf[OFFSET_EVENT_ID..])
            .expect("failed to encode event id");
        encode_u32(data_size, &mut self.io_buf[OFFSET_DATA_SIZE..])
            .expect("failed to encode data size");

        write(
            &mut self.session,
            self.base_address + OFFSET_EVENT_ID as u32,
            &mut self.io_buf[OFFSET_EVENT_ID..OFFSET_EVENT_ID + 8 + data_size as usize],
        )
        .expect("waah");
        Ok(())
    }

    fn decode_event(&mut self) -> Result<Event, runtime::Error> {
        read(
            &mut self.session,
            self.base_address + OFFSET_EVENT_ID as u32,
            &mut self.io_buf[OFFSET_EVENT_ID..OFFSET_EVENT_ID + 8],
        )
        .expect("waah");
        let event_id = decode_u32(&self.io_buf[OFFSET_EVENT_ID..])?;
        let data_size = decode_u32(&self.io_buf[OFFSET_DATA_SIZE..])? as usize;
        // eprintln!(
        //     "decode_event event_id: {}, data_size: {}",
        //     event_id, data_size
        // );
        read(
            &mut self.session,
            self.base_address + OFFSET_DATA as u32,
            &mut self.io_buf[OFFSET_DATA..OFFSET_DATA + data_size],
        )
        .expect("waah");
        Event::decode(event_id, &self.io_buf[OFFSET_DATA..OFFSET_DATA + data_size])
    }
}

fn read(session: &mut Session, address: u32, data: &mut [u8]) -> Result<usize, Error> {
    let mut core = session.core(0).expect("bah"); // todo: should be errors
    core.read_8(address, data).expect("waaah");
    Ok(data.len())
}

fn write(session: &mut Session, address: u32, data: &[u8]) -> Result<usize, Error> {
    let mut core = session.core(0).expect("bah"); // todo: should be errors
    core.write_8(address, data).expect("waaah");
    Ok(data.len())
}

/// "EMT-RUNTIME "
// todo: this should live in common i think
const MAGIC_SEQUENCE: [u8; 12] = [
    0x45, 0x4d, 0x54, 0x2d, 0x52, 0x55, 0x4e, 0x54, 0x49, 0x4d, 0x45, 0x20,
];

fn detect_runtime(session: &mut Session, base_address: u32, size: u32) -> Result<u32, Error> {
    let mut buf = [0u8; 1024];
    let mut address = base_address;
    let mut offset = 0;
    while address < base_address + size {
        let len = read(session, address, &mut buf)?;
        for index in 0..len {
            offset = match buf[index] {
                value if value == MAGIC_SEQUENCE[offset] => offset + 1,
                _ => 0,
            };
            if offset == MAGIC_SEQUENCE.len() {
                return Ok(address + index as u32 - MAGIC_SEQUENCE.len() as u32 + 1);
            }
        }
        address += len as u32;
    }
    Err(Error::NoRuntime)
}
