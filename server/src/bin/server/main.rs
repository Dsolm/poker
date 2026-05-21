use libcasino::config::Config;
use libcasino::domain::blackjack::service::Service;
use libcasino::inbound::http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let repo = libcasino::outbound::memory::PlayerGameMap::default();
    let blackjack_service = Service::new(repo);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };

    let http_server = HttpServer::new(blackjack_service, server_config).await?;
    http_server.run().await
}
