use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use structopt::StructOpt;

use core::link::Probe;

mod handlers;

#[derive(StructOpt, Debug)]
#[structopt(name = "emt-host")]
pub struct HostOptions {
    #[structopt(short = "d", long = "domain", default_value = "localhost")]
    pub domain: String,

    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = HostOptions::from_args();
    let base_url = format!("{}:{}", opt.domain, opt.port);
    let probe = Probe::new().expect("failed to attach probe");
    let shared_probe = web::Data::new(Mutex::new(probe));

    HttpServer::new(move || {
        App::new()
            .app_data(shared_probe.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(handlers::get_probe)
            .service(handlers::post_reset)
            .service(handlers::post_read)
            .service(handlers::post_write)
    })
    .bind(base_url)?
    .run()
    .await
}
