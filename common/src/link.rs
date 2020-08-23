pub const BLOCK_SIZE: usize = 512;
const STATUS_OFFSET: usize = 12;
const HEADER_OFFSET: usize = 16;
const BLOB_OFFSET: usize = 24;

pub enum Status {
    Idle,
    Sent,
    Received,
}

pub struct Header {
    status: Status,
    event_id: u32,
    size: u32,
}

impl Header {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        unimplemented!();
    }

    fn decode(data: &[u8]) -> Result<Self, Error> {
        unimplemented!();
    }
}

pub enum Event {
    None,
}

impl Event {
    pub fn id(&self) -> u32 {
        match *self {
            Event::None => 0,
        }
    }

    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        unimplemented!();
    }

    fn decode(data: &[u8]) -> Result<Self, Error> {
        unimplemented!();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {}

/// total of 512 byte(s)
/// "EMT-RUNTIME "
/// 4 bytes status
/// 4 bytes event_id 0 = none, ...
/// 4 bytes data size
/// 488 bytes of data space
pub struct Link<T>
where
    T: Read + Write + Buffer,
{
    base_address: u32,
    link_io: T,
}

impl<T> Link<T>
where
    T: Read + Write + Buffer,
{
    pub fn new(base_address: u32, link_io: T) -> Self {
        Self {
            base_address,
            link_io,
        }
    }

    pub fn base_address(&self) -> u32 {
        self.base_address
    }

    pub fn status_address(&self) -> u32 {
        self.base_address() + STATUS_OFFSET as u32
    }

    pub fn header_address(&self) -> u32 {
        self.base_address() + HEADER_OFFSET as u32
    }

    pub fn blob_address(&self) -> u32 {
        self.base_address() + BLOB_OFFSET as u32
    }

    pub fn send(&mut self, event: Event) -> Result<(), Error> {
        // todo: expect idle status
        // let buf = self.buffer();
        // let size = event.encode(&mut buf[DATA_OFFSET..])?;
        // let header = Header {
        //     event_id: event.id(),
        //     size: size as u32,
        // };
        // header.encode(&mut buf[HEADER_OFFSET..])?;
        // self.write(
        //     self.base_address() + DATA_OFFSET as u32,
        //     data: &buf[DATA_OFFSET..DATA_OFFSET + size],
        // )?;
        // self.write(self.size_address(), &buf)?;
        Ok(())
    }

    /// expects data to be available immediately?
    pub fn receive(&mut self) -> Result<Event, Error> {
        // let header = self.poll()?;
        unimplemented!();
    }

    // fn try_receive(&self) -> Result<Option<Event>, Error> {
    // }

    // pub fn poll(&mut self) -> Result<Header, Error> {
    //     let mut data = [0u8; 8];
    //     self.relay.read(self.header_address(), &mut data)?;
    //     Ok(Header::decode(&data)?)
    // }

    pub fn acknowledge(&mut self) -> Result<(), Error> {
        let data = [0u8; 8];
        self.link_io.write(self.header_address(), &data)?;
        Ok(())
    }
}

pub trait Buffer {
    fn buf(&self) -> &[u8];
    fn mut_buf(&mut self) -> &mut [u8];
}

pub trait Read {
    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error>;
}

pub trait Write {
    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error>;
}
