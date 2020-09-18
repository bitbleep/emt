use common::{link, Meta, Test};

pub mod probe;
pub mod qemu;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    AttachFailed,
    NoRuntime,
    LinkError(link::Error),
}

impl From<link::Error> for Error {
    fn from(error: link::Error) -> Self {
        Self::LinkError(error)
    }
}

pub trait Runner {
    fn meta(&mut self) -> Meta;
    fn start(&mut self, id: u32) -> Result<Test, Error>;
}

/// "EMT-RUNTIME "
// todo: this should live in common i think
const MAGIC_SEQUENCE: [u8; 12] = [
    0x45, 0x4d, 0x54, 0x2d, 0x52, 0x55, 0x4e, 0x54, 0x49, 0x4d, 0x45, 0x20,
];

fn detect_runtime<T>(link_io: &mut T, base_address: u32, size: u32) -> Result<u32, Error>
where
    T: link::Read,
{
    let mut buf = [0u8; 1024];
    let mut address = base_address;
    let mut offset = 0;
    while address < base_address + size {
        let len = link_io.read(address, &mut buf)?;
        for index in 0..len {
            offset = match buf[index] {
                value if value == MAGIC_SEQUENCE[offset] => offset + 1,
                _ => 0,
            };
            if offset == MAGIC_SEQUENCE.len() {
                return Ok(address + index as u32 - MAGIC_SEQUENCE.len() as u32);
            }
        }
        address += len as u32;
    }
    Err(Error::NoRuntime)
}
