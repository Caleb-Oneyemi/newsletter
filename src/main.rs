use newsletter::config::get_config;
use newsletter::startup::run;
use newsletter::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_tracing_subscriber("newsletter".into(), "info".into(), std::io::stdout);

    init_tracing_subscriber(subscriber);

    let config = get_config().expect("failed to read config.yaml");
    let conn_pool = PgPool::connect_lazy(&config.db.get_connection_string().expose_secret())
        .expect("failed to connect to db");

    println!("db connection successful ....");

    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(addr.clone()).expect("failed to bind port");

    println!("listening on {} ....", addr);

    run(listener, conn_pool)?.await
}
