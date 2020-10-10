use crate::host::models::*;
use crate::runner::{DeviceLink, Error};

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
            .unwrap()
            .json::<ProbeInfo>()
            .unwrap();

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
            base_address: probe.base_address.unwrap(),
        })
    }
}

impl DeviceLink for Hosted {
    fn base_address(&self) -> u32 {
        self.base_address
    }

    fn reset(&mut self) -> Result<(), Error> {
        let body = Reset {};

        let _ = self
            .client
            .post(&format!("{}/reset", self.base_url))
            .json::<Reset>(&body)
            .send()
            .unwrap()
            .json::<Reset>()
            .unwrap();

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
            .unwrap()
            .json::<ReadResult>()
            .unwrap();

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
            .unwrap()
            .json::<WriteResult>()
            .unwrap();

        Ok(res.len)
    }
}
