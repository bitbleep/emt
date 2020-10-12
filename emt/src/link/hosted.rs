use std::path::{Path, PathBuf};

use crate::host::models::*;
use crate::link::{Error, Link};

pub struct Hosted {
    client: reqwest::blocking::Client,
    base_url: String,
    base_address: u32,
}

impl Hosted {
    pub fn new(domain: &str, port: u16, binary_path: Option<PathBuf>) -> Result<Self, Error> {
        let mut client = reqwest::blocking::Client::new();
        let base_url = format!("http://{}:{}", domain, port);

        let probe = client
            .get(&format!("{}/probe", base_url))
            .send()
            .map_err(|_| Error::Io)?
            .json::<ProbeResponse>()
            .map_err(|_| Error::Decoding)?;

        if let Some(binary_path) = binary_path {
            print!("flashing elf binary.. ");
            flash(&mut client, &base_url, &binary_path)?;
            println!("ok");
        }

        reset(&mut client, &base_url)?;

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
        reset(&mut self.client, &self.base_url)
    }

    fn flash(&mut self, path: &Path) -> Result<(), Error> {
        flash(&mut self.client, &self.base_url, path)
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

fn reset(client: &mut reqwest::blocking::Client, base_url: &str) -> Result<(), Error> {
    let body = ResetParams {};

    let _ = client
        .post(&format!("{}/reset", base_url))
        .json::<ResetParams>(&body)
        .send()
        .map_err(|_| Error::Io)?
        .json::<ResetResponse>()
        .map_err(|_| Error::Decoding)?;

    Ok(())
}

fn flash(
    client: &mut reqwest::blocking::Client,
    base_url: &str,
    binary_path: &Path,
) -> Result<(), Error> {
    let body = BinaryParams {
        data: std::fs::read(binary_path).map_err(|_| Error::Io)?,
    };

    let _ = client
        .post(&format!("{}/binary", base_url))
        .json::<BinaryParams>(&body)
        .send()
        .map_err(|_| Error::Io)?
        .json::<BinaryResponse>()
        .map_err(|_| Error::Decoding)?;

    Ok(())
}
