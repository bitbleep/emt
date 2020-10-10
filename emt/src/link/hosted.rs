use crate::host::models::*;
use crate::link::{Error, Link};

pub struct Hosted {
    client: reqwest::blocking::Client,
    base_url: String,
    base_address: u32,
}

impl Hosted {
    pub fn new(domain: &str, port: u16) -> Result<Self, Error> {
        let client = reqwest::blocking::Client::new();
        let base_url = format!("http://{}:{}", domain, port);

        let probe = client
            .get(&format!("{}/probe", base_url))
            .send()
            .map_err(|_| Error::Io)?
            .json::<ProbeResponse>()
            .map_err(|_| Error::Decoding)?;

        let base_address = match probe.base_address {
            Some(addr) => addr,
            None => return Err(Error::NoRuntimeFound),
        };

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
            base_address: base_address,
        })
    }
}

impl Link for Hosted {
    fn base_address(&self) -> u32 {
        self.base_address
    }

    fn reset(&mut self) -> Result<(), Error> {
        let body = ResetParams {};

        let _ = self
            .client
            .post(&format!("{}/reset", self.base_url))
            .json::<ResetParams>(&body)
            .send()
            .map_err(|_| Error::Io)?
            .json::<ResetResponse>()
            .map_err(|_| Error::Decoding)?;

        Ok(())
    }

    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error> {
        let body = ReadParams {
            address,
            len: data.len(),
        };

        let res = self
            .client
            .post(&format!("{}/read", self.base_url))
            .json::<ReadParams>(&body)
            .send()
            .map_err(|_| Error::Io)?
            .json::<ReadResponse>()
            .map_err(|_| Error::Decoding)?;

        data.copy_from_slice(&res.data);
        Ok(res.data.len())
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error> {
        let body = WriteParams {
            address,
            data: data.to_vec(),
        };

        let res = self
            .client
            .post(&format!("{}/write", self.base_url))
            .json::<WriteParams>(&body)
            .send()
            .map_err(|_| Error::Io)?
            .json::<WriteResponse>()
            .map_err(|_| Error::Decoding)?;

        Ok(res.len)
    }
}
