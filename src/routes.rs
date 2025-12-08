use axum::{Json, http::StatusCode};
use crate::models::{AnalyzeTxRequest, AnalyzeTxResponse};
use crate::services::{blockchain, ai};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn analyze_tx(
    Json(payload): Json<AnalyzeTxRequest>,
) -> Result<Json<AnalyzeTxResponse>, (StatusCode, String)> {
    // 1. Fetch raw tx details from blockchain (stub for now)
    let tx_details = blockchain::fetch_transaction(&payload.network, &payload.tx_hash)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to fetch tx details: {}", e),
            )
        })?;

    // 2. Call AI analyzer with structured tx summary
    let analysis = ai::analyze_transaction(&payload.network, &payload.tx_hash, &tx_details)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("AI analysis failed: {}", e),
            )
        })?;

    Ok(Json(analysis))
}

