use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
use user_stat::UserStatsService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "[::1]:8989".parse().unwrap();
    info!("user-stats service listening on: {}", addr);

    let service = UserStatsService::new().await.into_server();

    info!("starting service...");
    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
