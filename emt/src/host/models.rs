use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeResponse {
    pub probe_attached: bool,
    pub base_address: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadParams {
    pub address: u32,
    pub len: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResponse {
    pub address: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteParams {
    pub address: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteResponse {
    pub address: u32,
    pub len: usize,
}
