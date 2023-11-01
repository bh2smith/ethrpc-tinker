use ethrpc::{eth, types::*};
use futures::future::join_all;
use hex_literal::hex;
use solabi::{
    decode::{self},
    Address,
};
use std::{borrow::Cow, error::Error};

fn get_name_call(address: Address) -> TransactionCall {
    // Should be able to use solabi to construct `get_name_call` from function signature:
    // function name() public view virtual override returns (string memory)
    TransactionCall {
        to: Some(address),
        input: Some(hex!("06fdde03").to_vec()),
        ..Default::default()
    }
}

pub async fn get_names(addresses: Vec<Address>) -> Result<(), Box<dyn Error>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    tracing::info!("Preparing {} GetName Requests", addresses.len());
    let futures = addresses
        .into_iter()
        .map(|addr| client.call(eth::Call, (get_name_call(addr), BlockId::default())));

    let x: Vec<_> = join_all(futures)
        .await
        .into_iter()
        .map(|r| match r {
            Ok(bytes) => {
                // TODO use solabi::Decode::decode here!
                // this seems like a hack.
                decode::decode::<Cow<str>>(&bytes.as_slice()[32..])
                    .unwrap()
                    .to_string()
            }
            Err(err) => err.to_string(),
        })
        .collect();
    tracing::info!("Resolved {} GetName Requests", x.len());
    println!("Names: {:?}", x);
    Ok(())
}
