use crate::host::models::*;
use crate::runner::{DeviceLink, Error};

pub struct Hosted {
    base_address: u32,
}

impl Hosted {
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let resp = reqwest::blocking::get(&format!("{}/probe", base_url))
            .unwrap()
            .json::<ProbeInfo>()
            .unwrap();
        println!("{:?}", resp);
        unimplemented!();
    }
}

impl DeviceLink for Hosted {
    fn base_address(&self) -> u32 {
        self.base_address
    }

    fn reset(&mut self) -> Result<(), Error> {
        // call POST url/reset {}
        unimplemented!();
    }

    fn read(&mut self, address: u32, data: &mut [u8]) -> Result<usize, Error> {
        // call POST url/read {"address": <>, "data_size": <>}
        unimplemented!();
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<usize, Error> {
        // call POST url/write {"address": <>, "data": []}
        unimplemented!();
    }
}
