use actix_web::{dev::Server, web, App, HttpServer};
use std::{io::Error, net::TcpListener};

use crate::routes::health_check;

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
