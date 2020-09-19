use common::runtime::{Error, Event, Runtime, Status};

#[repr(C)]
pub(crate) struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: Status,
    event_id: u32,
    data_size: u32,
    data: [u8; 488],
}

impl RuntimeBlock {
    pub const fn new() -> Self {
        Self {
            magic_sequence: [0u8; 12],
            status: Status::NotReady,
            event_id: Event::None.id(),
            data_size: 0,
            data: [0u8; 488],
        }
    }

    pub fn init(&mut self) {
        self.magic_sequence = *b"EMT-RUNTIME ";
        self.status = Status::Idle;
        self.event_id = Event::None.id();
    }

    fn await_status(&mut self, status: Status) {
        while self.status != status {}
    }
}

impl Runtime for RuntimeBlock {
    fn status(&self) -> Status {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    fn request(&mut self, event: Event) -> Result<Event, Error> {
        self.await_status(Status::Idle);
        self.event_id = event.id();
        self.data_size = event.encode(&mut self.data)? as u32;
        self.status = Status::Send;
        while self.status != Status::Receive {}
        let event = Event::decode(self.event_id, &self.data[..self.data_size as usize])?;
        self.status = Status::Idle;
        Ok(event)
    }

    fn respond(&mut self, event: Event) -> Result<(), Error> {
        self.await_status(Status::Send);
        self.event_id = event.id();
        self.data_size = event.encode(&mut self.data)? as u32;
        self.status = Status::Receive;
        Ok(())
    }

    fn read(&mut self) -> Result<Event, Error> {
        self.await_status(Status::Send);
        let event = Event::decode(self.event_id, &self.data[..self.data_size as usize])?;
        Ok(event)
    }
}
