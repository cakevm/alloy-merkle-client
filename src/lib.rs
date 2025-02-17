use alloy_consensus::{SignableTransaction, TxEip1559, TxEip2930, TxEip4844, TxEip4844Variant, TxEip7702, TxEnvelope, TxLegacy, TxType};
use alloy_primitives::Bytes;
use alloy_primitives::{Address, ChainId, PrimitiveSignature, TxHash, TxKind, U256};
use alloy_rpc_types_eth::{AccessList, Transaction};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

pub const MERKLE_SEARCHERS_URL: &str = "wss://mempool.merkle.io/stream/auctions";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MerkleTx {
    pub chain_id: u64,
    pub data: Bytes,
    pub from: Address,
    pub function_selector: Bytes,
    pub gas: u64,
    pub gas_fee_cap: U256,
    pub gas_price: U256,
    pub gas_tip_cap: U256,
    pub hash: TxHash,
    pub logs: Vec<Value>,
    pub nonce: u64,
    pub to: Option<Address>,
    #[serde(rename = "type")]
    pub tx_type: TxType,
    pub value: U256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MerkleTxAuction {
    pub chain_id: ChainId,
    pub id: String,
    pub fee_recipient: Address,
    pub transaction: MerkleTx,
    pub closes_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<MerkleTx> for Transaction<TxEnvelope> {
    fn from(tx: MerkleTx) -> Self {
        let tx_envelop = match tx.tx_type {
            TxType::Legacy => TxEnvelope::Legacy(
                TxLegacy {
                    chain_id: Some(tx.chain_id),
                    nonce: tx.nonce,
                    gas_price: tx.gas_price.to::<u128>(),
                    gas_limit: tx.gas,
                    to: TxKind::from(tx.to),
                    value: tx.value,
                    input: tx.data,
                }
                .into_signed(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false)),
            ),
            TxType::Eip2930 => TxEnvelope::Eip2930(
                TxEip2930 {
                    chain_id: tx.chain_id,
                    nonce: tx.nonce,
                    gas_price: tx.gas_price.to::<u128>(),
                    gas_limit: tx.gas,
                    to: TxKind::from(tx.to),
                    value: tx.value,
                    input: tx.data,
                    access_list: AccessList::default(),
                }
                .into_signed(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false)),
            ),
            TxType::Eip1559 => TxEnvelope::Eip1559(
                TxEip1559 {
                    chain_id: tx.chain_id,
                    nonce: tx.nonce,
                    max_priority_fee_per_gas: tx.gas_fee_cap.to::<u128>(),
                    max_fee_per_gas: tx.gas_tip_cap.to::<u128>(),
                    gas_limit: tx.gas,
                    to: TxKind::from(tx.to),
                    value: tx.value,
                    access_list: Default::default(),
                    input: tx.data,
                }
                .into_signed(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false)),
            ),
            TxType::Eip4844 => TxEnvelope::Eip4844(
                TxEip4844Variant::TxEip4844(TxEip4844 {
                    chain_id: tx.chain_id,
                    nonce: tx.nonce,
                    gas_limit: tx.gas,
                    max_fee_per_gas: tx.gas_fee_cap.to::<u128>(),
                    max_priority_fee_per_gas: tx.gas_tip_cap.to::<u128>(),
                    to: tx.to.unwrap_or_default(),
                    value: tx.value,
                    input: tx.data,
                    access_list: AccessList::default(),
                    blob_versioned_hashes: vec![],
                    max_fee_per_blob_gas: 0,
                })
                .into_signed(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false)),
            ),
            TxType::Eip7702 => TxEnvelope::Eip7702(
                TxEip7702 {
                    chain_id: tx.chain_id,
                    nonce: tx.nonce,
                    gas_limit: tx.gas,
                    max_fee_per_gas: tx.gas_fee_cap.to::<u128>(),
                    max_priority_fee_per_gas: tx.gas_tip_cap.to::<u128>(),
                    to: tx.to.unwrap_or_default(),
                    value: tx.value,
                    input: tx.data,
                    access_list: AccessList::default(),
                    authorization_list: vec![],
                }
                .into_signed(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false)),
            ),
        };
        Transaction {
            inner: tx_envelop,
            block_hash: None,
            block_number: None,
            transaction_index: None,
            effective_gas_price: None,
            from: tx.from,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::address;
    use std::str::FromStr;

    #[test]
    fn test_deserialize_type_0() {
        let tx_raw = r#"{
            "chain_id": 1,
            "closes_at": "2025-02-17T16:56:51.139393387Z",
            "created_at": "2025-02-17T16:56:49.639393387Z",
            "fee_recipient": "0x1E8e81dC3B221885b386e3d1c9efe93fc2863B24",
            "id": "96d87b2c-363d-4d2e-9c84-0bf676332419",
            "transaction":
            {
                "chain_id": 1,
                "data": "4e71d92d",
                "from": "0xDFB1C86E93C0e07F747cF969eEE4cED350aC2cff",
                "function_selector": "0x4e71d92d",
                "gas": 127360,
                "gas_fee_cap": "",
                "gas_price": "",
                "gas_tip_cap": "",
                "hash": "0xe1cd7d0a9a62f98fa779b68c6bf73adf9f68fb48723d1bfcc88d99248796bc12",
                "logs":
                [],
                "nonce": 21,
                "to": "0x9a15bB3a8FEc8d0d810691BAFE36f6e5d42360F7",
                "type": 0,
                "value": "0"
            }
        }"#;

        let auction: MerkleTxAuction = serde_json::from_str(tx_raw).unwrap();
        assert_eq!(auction.id, "96d87b2c-363d-4d2e-9c84-0bf676332419");
        assert_eq!(auction.chain_id, 1);
        assert_eq!(auction.transaction.from, address!("0xDFB1C86E93C0e07F747cF969eEE4cED350aC2cff"));
    }

    #[test]
    fn test_deserialize_type_2() {
        let tx_raw = r#"{
            "chain_id": 1,
            "closes_at": "2025-02-17T15:19:26.012375336Z",
            "created_at": "2025-02-17T15:19:24.512375336Z",
            "fee_recipient": "0x1E8e81dC3B221885b386e3d1c9efe93fc2863B24",
            "id": "8afec385-576b-428b-80b0-1f8147093877",
            "transaction": {
                "chain_id": 1,
                "data": "4e71d92d",
                "from": "0xD99e9d68e940B385FBDb3B63213763A218A9E2CF",
                "function_selector": "0x4e71d92d",
                "gas": 133451,
                "gas_fee_cap": "",
                "gas_price": "",
                "gas_tip_cap": "",
                "hash": "0x2cc884af9ec0804ecaf0a44be62929478a2fb18f64c2a7de47dfce4ea64893f3",
                "logs": [],
                "nonce": 60,
                "to": "0x9a15bB3a8FEc8d0d810691BAFE36f6e5d42360F7",
                "type": 2,
                "value": "0"
            }
        }"#;

        let auction: MerkleTxAuction = serde_json::from_str(tx_raw).unwrap();
        assert_eq!(auction.id, "8afec385-576b-428b-80b0-1f8147093877");
        assert_eq!(auction.chain_id, 1);
        assert_eq!(auction.fee_recipient, address!("0x1E8e81dC3B221885b386e3d1c9efe93fc2863B24"));
    }

    #[test]
    fn test_tx_from_merkle_tx_legacy() {
        let merkle_tx = MerkleTx {
            chain_id: 1,
            data: Bytes::from("4e71d92d"),
            from: address!("0xD99e9d68e940B385FBDb3B63213763A218A9E2CF"),
            function_selector: Bytes::from("0x4e71d92d"),
            gas: 133451,
            gas_fee_cap: U256::ZERO,
            gas_price: U256::ZERO,
            gas_tip_cap: U256::ZERO,
            hash: TxHash::from_str("0x2cc884af9ec0804ecaf0a44be62929478a2fb18f64c2a7de47dfce4ea64893f3").unwrap(),
            logs: vec![],
            nonce: 60,
            to: Some(address!("0x9a15bB3a8FEc8d0d810691BAFE36f6e5d42360F7")),
            tx_type: TxType::Legacy,
            value: U256::ZERO,
        };

        let tx: Transaction<TxEnvelope> = merkle_tx.into();
        assert_eq!(tx.from, address!("0xD99e9d68e940B385FBDb3B63213763A218A9E2CF"));
    }
}
