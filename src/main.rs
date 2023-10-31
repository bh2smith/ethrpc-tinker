use ethrpc::{eth, types::*};
use futures::future::join_all;
use std::error::Error;
use tracing::Level;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

/// Coppied from: https://github.com/nlordell/ethrpc-rs/blob/main/examples/http/src/main.rs
async fn repo_example() -> Result<(), Box<dyn Error>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    let (block_number, block) = tokio::try_join!(
        client.call(eth::BlockNumber, Empty),
        client.call(
            eth::GetBlockByNumber,
            (BlockTag::Latest.into(), Hydrated::No)
        ),
    )?;

    assert_eq!(block_number, block.unwrap().number);
    Ok(())
}

async fn next_level_example(num_requests: u64) -> Result<(), Box<dyn Error>> {
    let n = num_requests;
    tracing::info!("Preparing {} GetBlock Requests", n);
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    let mut futures = vec![];
    for i in 1..num_requests + 1 {
        futures.push(client.call(eth::GetBlockByNumber, (i.into(), Hydrated::No)));
    }
    tracing::info!("Loaded {} GetBlock Requests", n);
    let x = join_all(futures).await.into_iter().map(|r| r.ok());
    tracing::info!("Resolved {} GetBlock Requests", x.len());
    Ok(())
}

// async fn contract_calls() -> Result<(), Box<dyn Error>> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // let _ = repo_example().await;
    let _ = next_level_example(2000).await;
    Ok(())
}
