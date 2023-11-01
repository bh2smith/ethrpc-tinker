use ethrpc::{eth, types::*};
use futures::future::join_all;
use solabi::{decode::Decode, encode::Encode, selector, Address, FunctionEncoder};
use std::collections::HashMap;

const NAME: FunctionEncoder<(), (String,)> = FunctionEncoder::new(selector!("name()"));
const SYMBOL: FunctionEncoder<(), (String,)> = FunctionEncoder::new(selector!("symbol()"));
const TOKEN_URI: FunctionEncoder<(U256,), (String,)> =
    FunctionEncoder::new(selector!("tokenURI(uint256 tokenId)"));

/// First (weak attempt)
fn name_call(address: Address) -> TransactionCall {
    // Should be able to use solabi to construct `get_name_call` from function signature:
    // function name() public view virtual override returns (string memory)
    TransactionCall {
        to: Some(address),
        // This is only the selector (since name() has no parameters)
        input: Some(NAME.encode_params(&())),
        ..Default::default()
    }
}

/// Using solabi docs example:
fn symbol_call(address: Address) -> TransactionCall {
    TransactionCall {
        to: Some(address),
        input: Some(SYMBOL.encode_params(&())),
        ..Default::default()
    }
}

fn uri_call(token: NftId) -> TransactionCall {
    TransactionCall {
        to: Some(token.address),
        input: Some(TOKEN_URI.encode_params(&(token.id,))),
        ..Default::default()
    }
}

fn decode_function_result_string<T>(
    res: &Result<Vec<u8>, ethrpc::http::Error>,
    encoder: FunctionEncoder<T, (String,)>,
) -> Option<String>
where
    T: Encode + Decode,
{
    match res {
        Ok(bytes) => Some(encoder.decode_returns(bytes).expect("always blue").0),
        Err(err) => {
            tracing::warn!("got {:?}", err.to_string());
            None
        }
    }
}

pub async fn get_name_and_symbol(
    addresses: Vec<Address>,
) -> HashMap<Address, (Option<String>, Option<String>)> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    tracing::info!("Preparing {} GetName Requests", addresses.len());
    let name_futures = addresses
        .iter()
        .cloned()
        .map(|addr| client.call(eth::Call, (name_call(addr), BlockId::default())));

    let symbol_futures = addresses
        .iter()
        .cloned()
        .map(|addr| client.call(eth::Call, (symbol_call(addr), BlockId::default())));

    let names = join_all(name_futures).await;
    let symbols = join_all(symbol_futures).await;

    addresses
        .into_iter()
        .zip(names.iter().zip(symbols))
        .map(|(address, (name_result, symbol_result))| {
            (
                address,
                (
                    decode_function_result_string(name_result, NAME),
                    decode_function_result_string(&symbol_result, SYMBOL),
                ),
            )
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NftId {
    pub address: Address,
    pub id: U256,
}

pub async fn get_uris(token_ids: Vec<NftId>) -> HashMap<NftId, Option<String>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    tracing::info!("Preparing {} tokenUri Requests", token_ids.len());
    let name_futures = token_ids
        .iter()
        .cloned()
        .map(|token| client.call(eth::Call, (uri_call(token), BlockId::default())));

    let uris = join_all(name_futures).await;

    token_ids
        .into_iter()
        .zip(uris)
        .map(|(address, uri_result)| {
            (
                address,
                decode_function_result_string(&uri_result, TOKEN_URI),
            )
        })
        .collect()
}
