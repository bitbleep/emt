use crate::test::Context;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    BufferOverflow,
    IllegalString,
    IllegalStatus { actual: Status, expected: Status },
}

#[derive(Debug, Copy, Clone)]
pub struct Meta<'a> {
    pub id: &'a str,
    pub version: &'a str,
    pub num_tests: u32,
}

// todo: this should be a safer setup of TryFrom/Into etc.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Status {
    NotReady = 0,
    Idle,
    Send,
    Receive,
}

#[derive(Debug, Copy, Clone)]
pub enum Event<'a> {
    None,
    MetaRequest,
    Meta(Meta<'a>),
    Test(u32),
    Context,
    Output,
    Result,
}

impl<'a> Event<'a> {
    pub const fn id(&self) -> u32 {
        match *self {
            Event::None => 0,
            Event::MetaRequest => 1,
            Event::Meta(_) => 2,
            Event::Test(_) => 3,
            Event::Context => 4,
            Event::Output => 5,
            Event::Result => 6,
        }
    }

    pub fn encode(&self, into: &mut [u8]) -> Result<usize, Error> {
        match *self {
            Event::None => Ok(0),
            Event::MetaRequest => Ok(0),
            Event::Meta(meta) => {
                let mut len = encode_str(meta.id, into)?;
                len += encode_str(meta.version, &mut into[len..])?;
                len += encode_u32(meta.num_tests, &mut into[len..])?;
                Ok(len)
            }
            Event::Test(id) => Ok(encode_u32(id, into)?),
            Event::Context => unimplemented!(),
            Event::Output => unimplemented!(),
            Event::Result => unimplemented!(),
        }
    }

    pub fn decode(event_id: u32, from: &'a [u8]) -> Result<Event, Error> {
        Ok(match event_id {
            0 => Event::None,
            1 => Event::MetaRequest,
            2 => {
                let mut len = 0_usize;
                let (id, id_len) = decode_str(from)?;
                len += id_len;
                let (version, version_len) = decode_str(&from[len..])?;
                len += version_len;
                let num_tests = decode_u32(&from[len..])?;
                Event::Meta(Meta {
                    id,
                    version,
                    num_tests,
                })
            }
            3 => {
                let id = decode_u32(from)?;
                Event::Test(id)
            }
            _ => unimplemented!(),
        })
    }
}

pub trait Runtime {
    fn status(&self) -> Status;
    fn set_status(&mut self, status: Status);
    fn request(&mut self, event: Event) -> Result<Event, Error>;
    fn respond(&mut self, event: Event) -> Result<(), Error>;
    fn read(&mut self) -> Result<Event, Error>;
}

fn encode_u32(value: u32, into: &mut [u8]) -> Result<usize, Error> {
    if into.len() < 4 {
        return Err(Error::BufferOverflow);
    }
    into[0] = (value & 0xff) as u8;
    into[1] = (value >> 8 & 0xff) as u8;
    into[2] = (value >> 16 & 0xff) as u8;
    into[3] = (value >> 24 & 0xff) as u8;
    Ok(4)
}

fn decode_u32(from: &[u8]) -> Result<u32, Error> {
    if from.len() < 4 {
        return Err(Error::BufferOverflow);
    }
    Ok(from[0] as u32 | (from[1] as u32) << 8 | (from[2] as u32) << 16 | (from[3] as u32) << 24)
}

fn encode_str(value: &str, into: &mut [u8]) -> Result<usize, Error> {
    let len = value.as_bytes().len();
    if into.len() < len + 1 {
        return Err(Error::BufferOverflow);
    }
    into[..len].copy_from_slice(value.as_bytes());
    into[len] = 0;
    Ok(len + 1)
}

fn decode_str<'a>(from: &'a [u8]) -> Result<(&'a str, usize), Error> {
    let mut len = 0_usize;
    for index in 0..from.len() {
        if from[index] == 0 {
            len = index + 1;
            break;
        }
    }
    if len == 0 {
        return Err(Error::IllegalString);
    }
    match core::str::from_utf8(&from[..len]) {
        Ok(value) => Ok((value, len)),
        Err(_) => Err(Error::IllegalString),
    }
}
