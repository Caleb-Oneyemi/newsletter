use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::{io::Error, net::TcpListener};

use crate::routes::health_check;

pub fn run(listener: TcpListener, conn_pool: Pool<Postgres>) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .app_data(conn_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
