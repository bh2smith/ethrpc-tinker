use ethrpc::{eth, types::*};
use std::error::Error;

pub async fn run_example() -> Result<(), Box<dyn Error>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    let block = client
        .call(
            eth::GetBlockByNumber,
            (
                BlockSpec::Number(U256::from_str_radix("15000000", 10).unwrap()),
                Hydrated::Yes,
            ),
        )
        .await;

    assert!(block.is_err());
    // Prints: ERROR Json(JsonError(Error("data did not match any variant of untagged enum BlockTransactions", line: 0, column: 0)))
    println!("ERROR {:?}", block.unwrap_err());
    Ok(())
}
