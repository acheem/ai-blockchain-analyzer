use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Unsupported network: {0}")]
    UnsupportedNetwork(String),
    #[error("RPC error: {0}")]
    #[allow(dead_code)]
    RpcError(String),
}

pub async fn fetch_transaction(
    network: &str,
    tx_hash: &str,
) -> Result<Value, BlockchainError> {
    // TODO: Replace with real RPC call using ethers-rs or web3
    // For now, return a mocked tx JSON
    if network != "ethereum-mainnet" {
        return Err(BlockchainError::UnsupportedNetwork(network.to_string()));
    }

    let mock = serde_json::json!({
        "hash": tx_hash,
        "from": "0x1234...abcd",
        "to": "0xabcd...1234",
        "value": "1.5 ETH",
        "gas_used": 21000,
        "status": "success",
        "logs": [
            {
                "address": "0xUniswapV3Pool...",
                "topics": ["Swap", "..."],
                "data": "..."
            }
        ]
    });

    Ok(mock)
}

