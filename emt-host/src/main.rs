use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
struct Runtime {
    probe_attached: bool,
    id: String,
    version: String,
    num_tests: u32,
}

#[get("/runtime")]
async fn runtime() -> HttpResponse {
    HttpResponse::Ok().json(Runtime {
        probe_attached: false,
        id: "todo".to_owned(),
        version: "todo".to_owned(),
        num_tests: 0,
    })
}

#[derive(Deserialize, Debug)]
struct Run {
    test_id: u32,
}

#[post("/run")]
async fn run(params: web::Json<Run>) -> HttpResponse {
    println!("{:?}", params);
    HttpResponse::Ok().json(Test {
        token: Uuid::new_v4(),
    })
}

#[derive(Serialize, Debug)]
struct Test {
    token: Uuid,
}

#[get("/poll/{token}")]
async fn poll(web::Path(token): web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(Test { token })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let domain = "localhost";
    let port = 1234_u16;
    let addr = format!("{}:{}", domain, port);
    // todo: attach probe and share it across handlers
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .service(runtime)
            .service(run)
            .service(poll)
    })
    .bind(addr)?
    .run()
    .await
}