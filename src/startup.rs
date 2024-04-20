use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use std::{io::Error, net::TcpListener};

use crate::handlers::{health_check, subscribe};

pub fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server, Error> {
    let db_pool = Data::new(conn_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
