use newsletter::startup_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    startup_server()?.await
}
