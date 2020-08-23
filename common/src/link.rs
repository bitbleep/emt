pub enum Event {}

pub enum Error {}

pub trait Link {
    fn send(&self, event: Event) -> Result<(), Error>;
    fn receive(&self) -> Result<(), Error>;
    fn poll(&self) -> Result<bool, Error>;
}
