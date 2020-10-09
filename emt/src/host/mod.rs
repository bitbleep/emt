use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::link::Probe;
use crate::runner::DeviceLink;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProbeInfo {
    probe_attached: bool,
    base_address: Option<u32>,
}

#[get("/probe")]
async fn handle_probe(probe: web::Data<Mutex<Probe>>) -> HttpResponse {
    println!("probe");
    let probe = probe.lock().unwrap();
    HttpResponse::Ok().json(ProbeInfo {
        probe_attached: true,
        base_address: Some(probe.base_address()),
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reset {}

#[post("/reset")]
async fn handle_reset(probe: web::Data<Mutex<Probe>>, params: web::Json<Reset>) -> HttpResponse {
    println!("reset: {:?}", params);
    probe.lock().unwrap().reset().unwrap();
    HttpResponse::Ok().json(Reset {})
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadParams {
    address: u32,
    len: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResult {
    address: u32,
    data: Vec<u8>,
}

#[post("/read")]
async fn handle_read(
    probe: web::Data<Mutex<Probe>>,
    params: web::Json<ReadParams>,
) -> HttpResponse {
    println!("read: {:?}", params);
    let mut probe = probe.lock().unwrap();
    let mut data = vec![0; params.len as usize];
    probe.read(params.address, &mut data).unwrap();
    HttpResponse::Ok().json(ReadResult {
        address: params.address,
        data,
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteParams {
    address: u32,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteResult {
    address: u32,
    len: u32,
}

#[post("/write")]
async fn handle_write(
    probe: web::Data<Mutex<Probe>>,
    params: web::Json<WriteParams>,
) -> HttpResponse {
    println!("write: {:?}", params);
    let mut probe = probe.lock().unwrap();
    probe.write(params.address, &params.data).unwrap();
    HttpResponse::Ok().json(WriteResult {
        address: params.address,
        len: params.data.len() as u32,
    })
}
