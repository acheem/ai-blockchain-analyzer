use serde_json::Value;
use crate::models::AnalyzeTxResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("LLM call failed: {0}")]
    #[allow(dead_code)]
    LlmCallFailed(String),
}

pub async fn analyze_transaction(
    network: &str,
    tx_hash: &str,
    tx_details: &Value,
) -> Result<AnalyzeTxResponse, AiError> {
    // TODO v2: call real LLM (OpenAI / Anthropic) with tx_details
    // For now, we do a dumb rule-based placeholder that pretends to be AI.

    // Simple heuristic example
    let tx_type = if tx_details["logs"].as_array()
        .unwrap_or(&vec![])
        .iter()
        .any(|log| log["address"].as_str().unwrap_or("").contains("Uniswap"))
    {
        "DEX_SWAP".to_string()
    } else {
        "TRANSFER".to_string()
    };

    let protocol = if tx_type == "DEX_SWAP" {
        Some("Uniswap (detected heuristically)".to_string())
    } else {
        None
    };

    let risk_score = 0.2; // placeholder; later computed by LLM or ML

    let risk_reasons = vec![
        "Heuristic analysis only; no AI risk model yet".to_string()
    ];

    let natural_language_explanation = format!(
        "This is a placeholder analysis for transaction {} on {}.\n\
         In the next version, an AI model will interpret on-chain data, \
         classify the transaction type, and assess risk using LLM reasoning.",
        tx_hash, network
    );

    Ok(AnalyzeTxResponse {
        tx_hash: tx_hash.to_string(),
        network: network.to_string(),
        tx_type,
        protocol,
        risk_score,
        risk_reasons,
        natural_language_explanation,
    })
}

