use common::runtime::{Error, Event, Runtime, Status};

#[repr(C)]
pub(crate) struct RuntimeBlock {
    pub magic_sequence: [u8; 12],
    pub status: Status,
    pub event_id: Event,
    pub data_size: u32,
    pub data: [u8; 488],
}

impl RuntimeBlock {
    pub fn init(&mut self) {
        self.magic_sequence = *b"EMT-RUNTIME ";
        self.status = Status::Idle;
        self.event_id = Event::None;
    }
}

impl Runtime for RuntimeBlock {
    fn status(&self) -> Status {
        // todo: volatile read of status
        unimplemented!();
    }

    fn set_status(&mut self, status: Status) {
        // todo: volatile write of status
        unimplemented!();
    }

    fn request(&mut self, event: Event) -> Result<Event, Error> {
        unimplemented!();
    }

    fn respond(&mut self, event: Event) -> Result<(), Error> {
        unimplemented!();
    }

    fn read(&mut self) -> Result<Event, Error> {
        unimplemented!();
    }
}
