use alloy_consensus::TxEnvelope;
use alloy_merkle_client::{MerkleTxAuction, MERKLE_SEARCHERS_URL};
use alloy_rpc_types_eth::Transaction;
use eyre::Result;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Change this to "trace" to see websocket messages
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from("info")).init();

    // Connect to merkle.io searchers API
    let (ws_stream, _) = connect_async(MERKLE_SEARCHERS_URL).await.expect("Failed to connect");
    let (_, mut read) = ws_stream.split();
    info!("Subscribed to merkle.io searcher API");

    // This loop will print all pending transactions received from merkle.io
    while let Some(event) = read.next().await {
        let auction: MerkleTxAuction = serde_json::from_slice(&event?.into_data())?;
        info!("Received auction: {:?}", auction);

        let tx: Transaction<TxEnvelope> = auction.transaction.into();
        info!("Transaction: {:?}", tx);
    }

    Ok(())
}
