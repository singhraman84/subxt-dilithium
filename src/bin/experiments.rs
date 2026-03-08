#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    subxt_baseline::experiments::run_experiment().await
}