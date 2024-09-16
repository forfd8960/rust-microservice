use crm_send::NotificationService;
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "[::1]:9090".parse().unwrap();
    info!("notification service listening on: {}", addr);

    let service = NotificationService::new().into_server();

    info!("starting notification service...");
    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
