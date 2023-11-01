use ethrpc::{eth, types::*};
use std::error::Error;

/// Coppied from: https://github.com/nlordell/ethrpc-rs/blob/main/examples/http/src/main.rs
pub async fn run_example() -> Result<(), Box<dyn Error>> {
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
