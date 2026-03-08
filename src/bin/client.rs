#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    subxt_baseline::client::run_client().await
}