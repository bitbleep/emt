use probe_rs::{MemoryInterface, Session};

use crate::link::{Error, Link};
use emt_core::runtime::MAGIC_SEQUENCE;

pub struct Probe {
    base_address: u32,
    session: Session,
}

impl Probe {
    pub fn new() -> Result<Self, Error> {
        let mut session = Session::auto_attach("nrf52").map_err(|_| Error::AttachFailed)?;
        println!("auto attached nrf52");

        print!("reset device.. ");
        session
            .core(0)
            .map_err(|_| Error::Io)?
            .reset()
            .map_err(|_| Error::Io)?;
        println!("ok");

        let base_address = detect_runtime(&mut session, 0x2000_0000, 0x10000)?;
        println!("found runtime at address: 0x{:08x}", base_address);

        Ok(Self {
            base_address,
            session,
        })
    }
}

impl Link for Probe {
    fn base_address(&self) -> u32 {
        self.base_address
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.session
            .core(0)
            .map_err(|_| Error::Io)?
            .reset()
            .map_err(|_| Error::Io)?;
        Ok(())
    }

    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error> {
        read(&mut self.session, address, data)
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error> {
        write(&mut self.session, address, data)
    }
}

fn read(session: &mut Session, address: u32, data: &mut [u8]) -> Result<usize, Error> {
    let mut core = session.core(0).map_err(|_| Error::Io)?;
    core.read_8(address, data).map_err(|_| Error::Io)?;
    Ok(data.len())
}

fn write(session: &mut Session, address: u32, data: &[u8]) -> Result<usize, Error> {
    let mut core = session.core(0).map_err(|_| Error::Io)?;
    core.write_8(address, data).map_err(|_| Error::Io)?;
    Ok(data.len())
}

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
    Err(Error::NoRuntimeFound)
}
