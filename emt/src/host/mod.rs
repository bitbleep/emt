use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use crate::link::Probe;

mod handlers;
pub mod models;

pub async fn start(base_url: &str, probe: Probe) -> std::io::Result<()> {
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
