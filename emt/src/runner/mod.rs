use common::{Meta, Test};

pub mod probe;
pub mod qemu;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    _Doh,
}

pub trait Runner {
    fn meta(&mut self) -> Meta;
    fn start(&mut self, id: u32) -> Result<Test, Error>;
}
