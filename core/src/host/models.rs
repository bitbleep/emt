use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeInfo {
    pub probe_attached: bool,
    pub base_address: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reset {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadParams {
    pub address: u32,
    pub len: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResult {
    pub address: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteParams {
    pub address: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteResult {
    pub address: u32,
    pub len: usize,
}
