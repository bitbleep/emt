use crate::runtime::{Error, Meta};
use crate::{TestContext, TestResult};

#[derive(Debug, Copy, Clone)]
pub enum Event<'a> {
    None,
    MetaRequest,
    Meta(Meta<'a>),
    Test(u32),
    Context(TestContext<'a>),
    Output(&'a str),
    Result(TestResult),
}

impl<'a> Event<'a> {
    pub const fn id(&self) -> u32 {
        match *self {
            Event::None => 0,
            Event::MetaRequest => 1,
            Event::Meta(_) => 2,
            Event::Test(_) => 3,
            Event::Context(_) => 4,
            Event::Output(_) => 5,
            Event::Result(_) => 6,
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
            Event::Context(context) => {
                let mut len = encode_str(context.name, into)?;
                len += encode_str(context.description, &mut into[len..])?;
                len += encode_bool(context.requires_human_interaction, &mut into[len..])?;
                len += encode_bool(context.should_panic, &mut into[len..])?;
                len += encode_u32(context.timeout_ms, &mut into[len..])?;
                Ok(len)
            }
            Event::Output(message) => {
                let len = encode_str(message, into)?;
                Ok(len)
            }
            Event::Result(result) => {
                let len = encode_u32(result.into(), into)?;
                Ok(len)
            }
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
            4 => {
                let mut len = 0_usize;
                let (name, name_len) = decode_str(from)?;
                len += name_len;
                let (description, description_len) = decode_str(&from[len..])?;
                len += description_len;
                let requires_human_interaction = decode_bool(&from[len..])?;
                len += 1;
                let should_panic = decode_bool(&from[len..])?;
                len += 1;
                let timeout_ms = decode_u32(&from[len..])?;
                Event::Context(TestContext {
                    name,
                    description,
                    requires_human_interaction,
                    should_panic,
                    timeout_ms,
                })
            }
            5 => {
                let (message, _) = decode_str(from)?;
                Event::Output(message)
            }
            6 => Event::Result(TestResult::from(decode_u32(from)?)),
            _ => return Err(Error::IllegalEventId),
        })
    }
}

pub fn encode_u32(value: u32, into: &mut [u8]) -> Result<usize, Error> {
    if into.len() < 4 {
        return Err(Error::BufferOverflow);
    }
    into[0] = (value & 0xff) as u8;
    into[1] = (value >> 8 & 0xff) as u8;
    into[2] = (value >> 16 & 0xff) as u8;
    into[3] = (value >> 24 & 0xff) as u8;
    Ok(4)
}

pub fn decode_u32(from: &[u8]) -> Result<u32, Error> {
    if from.len() < 4 {
        return Err(Error::BufferOverflow);
    }
    Ok(from[0] as u32 | (from[1] as u32) << 8 | (from[2] as u32) << 16 | (from[3] as u32) << 24)
}

pub fn encode_str(value: &str, into: &mut [u8]) -> Result<usize, Error> {
    let len = value.as_bytes().len();
    if into.len() < len + 1 {
        return Err(Error::BufferOverflow);
    }
    into[..len].copy_from_slice(value.as_bytes());
    into[len] = 0;
    Ok(len + 1)
}

pub fn decode_str<'a>(from: &'a [u8]) -> Result<(&'a str, usize), Error> {
    let mut len = 0_usize;
    for index in 0..from.len() {
        if from[index] == 0 {
            len = index + 1;
            break;
        }
    }
    if len <= 1 {
        return Err(Error::IllegalString);
    }
    match core::str::from_utf8(&from[..len - 1]) {
        Ok(value) => Ok((value, len)),
        Err(_) => Err(Error::IllegalString),
    }
}

fn encode_bool(value: bool, into: &mut [u8]) -> Result<usize, Error> {
    if into.len() < 1 {
        return Err(Error::BufferOverflow);
    }
    into[0] = match value {
        true => 1,
        false => 0,
    };
    Ok(1)
}

fn decode_bool(from: &[u8]) -> Result<bool, Error> {
    if from.len() < 1 {
        return Err(Error::BufferOverflow);
    }
    match from[0] {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(Error::IllegalBool),
    }
}
