use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .await?;

    let mut stream = provider.subscribe_full_pending_txs().await?;

    while let Some(tx) = stream.next().await {
        assert!(tx.block_number.is_none()); // block number is not yet decided
        println!("From: {:?}\tTo: {:?}\tValue: {:?}\tBytes: {:?}", 
            tx.from, 
            tx.to.unwrap_or_default(), // different types for `from` and `to` 
            tx.value,
            tx.input,
        );
    }

    Ok(())
}
