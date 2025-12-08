use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AnalyzeTxRequest {
    pub network: String,
    pub tx_hash: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeTxResponse {
    pub tx_hash: String,
    pub network: String,
    pub tx_type: String,
    pub protocol: Option<String>,
    pub risk_score: f32,
    pub risk_reasons: Vec<String>,
    pub natural_language_explanation: String,
}

