use ethrpc::{eth, types::*};
use futures::future::{join, join_all};
use solabi::{decode::Decode, encode::Encode, selector, Address, FunctionEncoder};
use std::collections::HashMap;

const NAME: FunctionEncoder<(), (String,)> = FunctionEncoder::new(selector!("name()"));
const SYMBOL: FunctionEncoder<(), (String,)> = FunctionEncoder::new(selector!("symbol()"));
const TOKEN_URI: FunctionEncoder<(U256,), (String,)> =
    FunctionEncoder::new(selector!("tokenURI(uint256)"));

fn name_call(address: Address) -> TransactionCall {
    TransactionCall {
        to: Some(address),
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
        Err(_) => {
            // tracing::warn!("got {:?}", err.to_string());
            None
        }
    }
}

pub async fn get_name_and_symbol(
    addresses: Vec<Address>,
) -> HashMap<Address, (Option<String>, Option<String>)> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    tracing::debug!("Preparing {} Contract Details Requests", addresses.len());
    let name_futures = addresses
        .iter()
        .cloned()
        .map(|addr| client.call(eth::Call, (name_call(addr), BlockId::default())));

    let symbol_futures = addresses
        .iter()
        .cloned()
        .map(|addr| client.call(eth::Call, (symbol_call(addr), BlockId::default())));

    let (names, symbols) = join(join_all(name_futures), join_all(symbol_futures)).await;
    tracing::debug!("Complete {} Contract Details Requests", addresses.len());
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct NftId {
    pub address: Address,
    pub id: U256,
}

pub async fn get_uris(token_ids: Vec<NftId>) -> HashMap<NftId, Option<String>> {
    let client = ethrpc::http::Client::from_env().buffered(Default::default());
    tracing::debug!("Preparing {} tokenUri Requests", token_ids.len());
    let futures = token_ids
        .iter()
        .cloned()
        .map(|token| client.call(eth::Call, (uri_call(token), BlockId::default())));

    let uris = join_all(futures).await;
    tracing::debug!("Complete {} tokenUri Requests", token_ids.len());
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
