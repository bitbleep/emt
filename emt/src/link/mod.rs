mod hosted;
mod probe;

pub use hosted::Hosted;
pub use probe::Probe;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    AttachFailed,
    FlashingFailed,
    NoRuntimeFound,
    Io,
    Encoding,
    Decoding,
}

pub trait Link {
    fn base_address(&self) -> u32;
    fn reset(&mut self) -> Result<(), Error>;
    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error>;
    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error>;
}
