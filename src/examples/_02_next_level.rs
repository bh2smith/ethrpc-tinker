use ethrpc::{eth, types::*};
use futures::future::join_all;
use std::error::Error;

pub async fn run_example(num_requests: u64) -> Result<(), Box<dyn Error>> {
    let n = num_requests;
    tracing::debug!("Preparing {} GetBlock Requests", n);
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    let mut futures = vec![];
    for i in 1..num_requests + 1 {
        futures.push(client.call(eth::GetBlockByNumber, (i.into(), Hydrated::No)));
    }
    let x = join_all(futures).await.into_iter().map(|r| r.ok());
    tracing::debug!("Resolved {} GetBlock Requests", x.len());
    Ok(())
}
