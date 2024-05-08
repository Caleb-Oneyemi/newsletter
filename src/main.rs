use newsletter::config::get_config;
use newsletter::providers::EmailClient;
use newsletter::startup::run;
use newsletter::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_tracing_subscriber("newsletter".into(), "info".into(), std::io::stdout);

    init_tracing_subscriber(subscriber);

    let config = get_config().expect("failed to read config file");
    let conn_pool = PgPool::connect_lazy(&config.db.get_connection_string().expose_secret())
        .expect("failed to connect to db");

    println!("db connection successful ....");

    let sender = config
        .email
        .get_sender()
        .expect("invalid sender email address");

    let email_client = EmailClient::new(config.email.base_url.clone(), sender);
    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(addr.clone()).expect("failed to bind port");

    println!("listening on {} ....", addr);

    run(listener, conn_pool, email_client)?.await
}
