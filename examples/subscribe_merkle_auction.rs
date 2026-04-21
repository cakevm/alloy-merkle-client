use alloy_consensus::TxEnvelope;
use alloy_merkle_client::{MERKLE_SEARCHERS_URL, MerkleTxAuction};
use alloy_rpc_types_eth::Transaction;
use eyre::Result;
use futures_util::StreamExt;
use std::time::Duration;
use tokio_tungstenite::connect_async;
use tracing::{error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    // Change this to "trace" to see websocket messages
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from("info")).init();

    loop {
        // Connect to merkle.io searchers API
        let (ws_stream, _) = match connect_async(MERKLE_SEARCHERS_URL).await {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to connect: {e}");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };
        let (_, mut read) = ws_stream.split();
        info!("Subscribed to merkle.io searcher API");

        // This loop will print all pending transactions received from merkle.io
        while let Some(event) = read.next().await {
            let data = match event {
                Ok(msg) => msg.into_data(),
                Err(e) => {
                    warn!("WebSocket error: {e}");
                    break;
                }
            };

            let auction: MerkleTxAuction = match serde_json::from_slice(&data) {
                Ok(auction) => auction,
                Err(e) => {
                    warn!("Failed to deserialize auction: {e}");
                    continue;
                }
            };
            info!("Received auction: {:?}", auction);

            let tx: Transaction<TxEnvelope> = auction.transaction.into();
            info!("Transaction: {:?}", tx);
        }

        warn!("Connection closed, reconnecting in 5s...");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
