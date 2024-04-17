use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::io::Error;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("server is live")
}

pub fn startup_server() -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind("127.0.0.1:7777")?
        .run();

    Ok(server)
}
