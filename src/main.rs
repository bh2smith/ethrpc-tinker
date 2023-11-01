mod examples;
use std::error::Error;
use tracing::Level;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let _ = examples::_01_repo_example::run_example().await;
    let _ = examples::_02_next_level::run_example(500).await;
    Ok(())
}
