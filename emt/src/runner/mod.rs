use common::Meta;

pub mod probe;
pub mod qemu;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    _Doh,
}

pub trait Runner {
    fn meta(&mut self) -> Meta;
}
