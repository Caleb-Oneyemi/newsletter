use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::{io::Error, net::TcpListener};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("server is live")
}

pub fn startup_server(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
