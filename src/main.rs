use newsletter::config::get_config;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("starting server ....");

    let config = get_config().expect("failed to read config.yaml");
    let db_url = config.db.get_connection_string();
    let conn_pool = PgPool::connect(&db_url)
        .await
        .expect("failed to connect to db");

    println!("db connection successful ....");

    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr).expect("failed to bind port");

    run(listener, conn_pool)?.await
}
