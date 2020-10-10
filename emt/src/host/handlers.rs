use std::sync::Mutex;

use actix_web::{get, post, web, HttpResponse};

use crate::host::models::*;
use crate::link::{Link, Probe};

#[get("/probe")]
async fn get_probe(probe: web::Data<Mutex<Probe>>) -> HttpResponse {
    println!("probe");
    let probe = probe.lock().unwrap();
    HttpResponse::Ok().json(ProbeResponse {
        probe_attached: true,
        base_address: Some(probe.base_address()),
    })
}

#[post("/reset")]
async fn post_reset(
    probe: web::Data<Mutex<Probe>>,
    params: web::Json<ResetParams>,
) -> HttpResponse {
    println!("reset: {:?}", params);
    probe.lock().unwrap().reset().unwrap();
    HttpResponse::Ok().json(ResetResponse {})
}

#[post("/read")]
async fn post_read(probe: web::Data<Mutex<Probe>>, params: web::Json<ReadParams>) -> HttpResponse {
    println!("read: {:?}", params);
    let mut probe = probe.lock().unwrap();
    let mut data = vec![0; params.len as usize];
    probe.read(params.address, &mut data).unwrap();
    HttpResponse::Ok().json(ReadResponse {
        address: params.address,
        data,
    })
}

#[post("/write")]
async fn post_write(
    probe: web::Data<Mutex<Probe>>,
    params: web::Json<WriteParams>,
) -> HttpResponse {
    println!("write: {:?}", params);
    let mut probe = probe.lock().unwrap();
    probe.write(params.address, &params.data).unwrap();
    HttpResponse::Ok().json(WriteResponse {
        address: params.address,
        len: params.data.len(),
    })
}
