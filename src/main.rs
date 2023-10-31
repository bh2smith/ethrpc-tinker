use ethrpc::{eth, types::*};
use futures::future::{join_all, try_join};
use futures::try_join;
use std::error::Error;

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

async fn next_level_example() -> Result<(), Box<dyn Error>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    let mut futures = vec![];
    for i in 1..200 {
        futures.push(client.call(eth::GetBlockByNumber, (i.into(), Hydrated::No)));
    }

    // let x = tokio::join_all!(futures)?;
    // let x = try_join!(futures.iter())?;
    // println!("{x}", x);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = repo_example().await;
    Ok(())
}
