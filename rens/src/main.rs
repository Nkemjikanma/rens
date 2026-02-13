use alloy::network::Ethereum;
use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use rens::resolver::{resolve_address, resolve_name};

const RPC_URL: &str = "https://reth-ethereum.ithaca.xyz/rpc";

#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect(RPC_URL)
        .await
        .expect("Something went wrong");

    let address = resolve_name("vitalik.eth", &provider)
        .await
        .expect("Name resolutions went wrong!");

    // let address_test = resolve_name("", &provider).await.expect("Error");

    let ens_name = resolve_address(
        address!("0xD1a4180f7F92a7b39b1eECC7D61E573E965A5cFc"),
        &provider,
    )
    .await
    .expect("Error");

    println!("{}", address);
    // println!("{}", address_test);
    println!("{}", ens_name);
}
