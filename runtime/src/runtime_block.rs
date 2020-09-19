use common::runtime::{Error, Event, Runtime, Status};

#[repr(C)]
pub(crate) struct RuntimeBlock {
    magic_sequence: [u8; 12],
    status: Status,
    event_id: Event,
    data_size: u32,
    data: [u8; 488],
}

impl RuntimeBlock {
    pub const fn new() -> Self {
        Self {
            magic_sequence: [0u8; 12],
            status: Status::NotReady,
            event_id: Event::None,
            data_size: 0,
            data: [0u8; 488],
        }
    }

    pub fn init(&mut self) {
        self.magic_sequence = *b"EMT-RUNTIME ";
        self.status = Status::Idle;
        self.event_id = Event::None;
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
        expect_status(self.status, Status::Idle)?;
        // todo: encode and write to event id, size, data
        self.status = Status::Send;
        while self.status != Status::Receive {}
        // todo: read and decode event
        let event = Event::None;
        self.status = Status::Idle;
        Ok(event)
    }

    fn respond(&mut self, event: Event) -> Result<(), Error> {
        expect_status(self.status, Status::Send)?;
        // todo: encode and write to event id, size, data
        self.status = Status::Receive;
        Ok(())
    }

    fn read(&mut self) -> Result<Event, Error> {
        while self.status != Status::Send {}
        // todo: read and decode event
        let event = Event::None;
        self.status = Status::Receive;
        Ok(event)
    }
}

fn expect_status(actual: Status, expected: Status) -> Result<(), Error> {
    if actual != expected {
        Err(Error::IllegalStatus { actual, expected })
    } else {
        Ok(())
    }
}
