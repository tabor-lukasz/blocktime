use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

mod eth_client;

#[tokio::main]
async fn main() {
    let client = Arc::new(eth_client::EthClient::new().await);
    
    let route = warp::path!("currentBlockTime").and_then( move || {
        handle_connection(client.clone())
    });
    let routes = route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 12345)).await;
}

async fn handle_connection(client: Arc<eth_client::EthClient>) -> std::result::Result<impl Reply, Rejection> {
    match client.get_eth_ts().await {
            Some(v) => Ok(v),
            None => Ok("Unknown".to_string())
        }
}
