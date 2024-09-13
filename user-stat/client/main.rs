use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Request;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

use user_stat::pb::{user_stats_client::UserStatsClient, GreetRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    info!("running user stats client");

    let endpoint = Endpoint::from_static("http://[::1]:8989");

    let mut client = UserStatsClient::connect(endpoint).await?;

    call_greet(&mut client).await?;

    Ok(())
}

async fn call_greet(client: &mut UserStatsClient<Channel>) -> anyhow::Result<()> {
    let request = Request::new(GreetRequest {
        msg: "welcome to here".to_string(),
    });

    let resp = client.greet(request).await?;
    info!("greet resp: {:?}", resp);
    Ok(())
}
