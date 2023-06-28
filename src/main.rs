use axum::{
    routing::get,
    Router,
};
use ethers::providers::{Middleware, Provider};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

async fn update_block(block_string: Arc<RwLock<String>>) -> eyre::Result<()> {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Provider::try_from(rpc_url)?;

    loop {
        let block_number = provider.get_block_number().await?;

        {
            let mut block_str_guard = block_string.write().await;
            *block_str_guard = block_number.as_u64().to_string();
        }

        sleep(Duration::from_secs(13)).await;
    }
}

#[tokio::main]
async fn main() {

    let block_string = Arc::new(RwLock::new(String::new()));

    tokio::spawn(update_block(block_string.clone()));

    let app = Router::new().route("/", get(move || {
        let block_str_clone = block_string.clone();
        async move {
            let block_str_guard = block_str_clone.read().await;
            block_str_guard.clone()
        }
    }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}