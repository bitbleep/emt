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
}

impl Runtime for RuntimeBlock {
    fn status(&self) -> Status {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    fn encode_event(&mut self, event: Event) -> Result<(), Error> {
        self.event_id = event.id();
        self.data_size = event.encode(&mut self.data)? as u32;
        Ok(())
    }

    fn decode_event(&mut self) -> Result<Event, Error> {
        Event::decode(self.event_id, &self.data[..self.data_size as usize])
    }
}
