use axum::{
    routing::get,
    Router,
};
use ethers::providers::{Http, Middleware, Provider};

async fn get_block() -> eyre::Result<String> {
    let rpc_url = "https://eth.llamarpc.com";
    let provider = Provider::try_from(rpc_url)?;
    let block_number = provider.get_block_number().await?;

    Ok(block_number.as_u64().to_string())
}

#[tokio::main]
async fn main() {

    let block_string = get_block().await.expect("Failed");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { block_string }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}