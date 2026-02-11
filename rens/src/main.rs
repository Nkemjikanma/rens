use alloy::network::Ethereum;
use alloy::providers::ProviderBuilder;
use rens::resolver::resolve_name;

const RPC_URL: &str = "https://reth-ethereum.ithaca.xyz/rpc";

#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new()
        .network::<Ethereum>()
        .connect(RPC_URL)
        .await
        .expect("Something went wrong");

    let address = resolve_name("vitalik.eth", &provider).await;

    println!("{}", address)
}
