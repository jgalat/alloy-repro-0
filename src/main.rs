use alloy::{
    network::TransactionBuilder,
    primitives::utils::parse_ether,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let ws = WsConnect::new("ws://localhost:8545");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_ws(ws)
        .await?;

    let wallet = provider.get_accounts().await?[0];

    let tx = TransactionRequest::default()
        .with_from(wallet)
        .with_to(wallet)
        .with_value(parse_ether("1").unwrap())
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    let pending_tx = provider.send_transaction(tx).await?;

    let hash = pending_tx
        .with_required_confirmations(1)
        .with_timeout(Some(tokio::time::Duration::from_secs(5)))
        .watch()
        .await?;

    println!("tx hash {}", hash);

    let receipt = provider.get_transaction_receipt(hash).await?.unwrap();

    println!(
        "tx status {} @ block {}",
        receipt.status(),
        receipt.block_number.unwrap()
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    Ok(())
}
