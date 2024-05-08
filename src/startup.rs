use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use std::{io::Error, net::TcpListener};
use tracing_actix_web::TracingLogger;

use crate::{
    handlers::{health_check, subscribe},
    providers::EmailClient,
};

pub fn run(
    listener: TcpListener,
    conn_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, Error> {
    let db_pool = Data::new(conn_pool);
    let email_client = Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
