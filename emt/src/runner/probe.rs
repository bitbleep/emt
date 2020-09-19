use probe_rs::{MemoryInterface, Session};

use common::{/*link, link::Event,*/ runtime::Meta, test::Context};

use crate::runner::Error;

pub struct Runner {
    // link: link::Link<LinkIo>,
// meta: Meta,
}

impl Runner {
    pub fn attach() -> Result<Self, Error> {
        let session = Session::auto_attach("nrf52").map_err(|_err| Error::AttachFailed)?;
        eprintln!("auto attached nrf52");

        // let mut link_io = LinkIo::new(session);
        // let base_address = detect_runtime(&mut link_io, 0x2000_0000, 0x10000)?;
        // eprintln!("found base address: 0x{:08x}", base_address);

        // todo: read meta
        // let mut link = link::Link::new(base_address, link_io);
        // link.send(Event::None).expect("waah");

        Ok(Self {
            // link,
            // meta: Meta {},
        })
    }
}

impl crate::runner::Runner for Runner {
    fn meta(&mut self) -> Meta {
        unimplemented!();
    }

    fn start(&mut self, _id: u32) -> Result<Context, Error> {
        // let none_event = link::Event::None;
        // self.link.send(none_event)?;
        unimplemented!();
    }
}

// pub struct LinkIo {
//     session: Session,
//     io_buf: Vec<u8>,
// }

// impl LinkIo {
//     pub fn new(session: Session) -> Self {
//         Self {
//             session,
//             io_buf: vec![0u8; link::BLOCK_SIZE],
//         }
//     }
// }

// impl link::Buffer for LinkIo {
//     fn buf(&self) -> &[u8] {
//         &self.io_buf
//     }

//     fn mut_buf(&mut self) -> &mut [u8] {
//         &mut self.io_buf
//     }
// }

// impl link::Read for LinkIo {
//     fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, link::Error> {
//         let mut core = self.session.core(0).expect("bah"); // todo: should be errors
//         core.read_8(address, data).expect("waaah");
//         Ok(data.len())
//     }
// }

// impl link::Write for LinkIo {
//     fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, link::Error> {
//         let mut core = self.session.core(0).expect("bah"); // todo: should be errors
//         core.write_8(address, data).expect("waaah");
//         Ok(data.len())
//     }
// }
