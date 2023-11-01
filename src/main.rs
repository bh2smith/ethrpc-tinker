mod examples;
mod util;
use examples::_03_solabi::NftId;
use solabi::Address;
use std::{error::Error, str::FromStr};
use tracing::Level;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let _ = examples::_01_repo_example::run_example().await;
    let _ = examples::_02_next_level::run_example(500).await;
    let _ = examples::_03_solabi::get_name_and_symbol(
        util::addresses_from_file("./addresses.txt").unwrap(),
    )
    .await;
    let _ = examples::_03_solabi::get_uris(
        (1u32..100)
            .map(|i| NftId {
                address: Address::from_str("0xD06966A7860131A8F858573BD76D08C27E7286BA").unwrap(),
                id: i.into(),
            })
            .collect(),
    )
    .await;
    Ok(())
}
