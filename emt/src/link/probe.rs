use probe_rs::{MemoryInterface, Session};

use crate::runner::{DeviceLink, Error};

pub struct Probe {
    base_address: u32,
    session: Session,
}

impl Probe {
    pub fn new() -> Result<Self, Error> {
        let mut session = Session::auto_attach("nrf52").map_err(|_err| Error::AttachFailed)?;
        println!("auto attached nrf52");

        print!("reset device.. ");
        session
            .core(0)
            .expect("failed to get core")
            .reset()
            .expect("failed to reset");
        println!("ok");

        let base_address = detect_runtime(&mut session, 0x2000_0000, 0x10000)?;
        println!("found runtime at address: 0x{:08x}", base_address);

        Ok(Self {
            base_address,
            session,
        })
    }
}

impl DeviceLink for Probe {
    fn base_address(&self) -> u32 {
        self.base_address
    }

    fn reset(&mut self) -> Result<(), Error> {
        self.session
            .core(0)
            .expect("failed to get core")
            .reset()
            .expect("failed to reset");
        Ok(())
    }

    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error> {
        let mut core = self.session.core(0).expect("failed to get core");
        core.read_8(address, data).expect("failed to read");
        Ok(data.len())
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error> {
        let mut core = self.session.core(0).expect("failed to get core");
        core.write_8(address, data).expect("failed to read");
        Ok(data.len())
    }
}

fn read(session: &mut Session, address: u32, data: &mut [u8]) -> Result<usize, Error> {
    let mut core = session.core(0).expect("bah"); // todo: should be errors
    core.read_8(address, data).expect("waaah");
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