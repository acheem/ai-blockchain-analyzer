# Data Flow Documentation

This document describes the detailed data flow through the AI Blockchain Analyzer system.

## 📊 High-Level Data Flow

```
┌──────────┐
│  Client  │
│          │
└────┬─────┘
     │
     │ 1. HTTP POST /analyze_tx
     │    {"network": "ethereum-mainnet", "tx_hash": "0x123..."}
     │
     ▼
┌────────────────────────────────────────────────┐
│           Axum Web Server                      │
│                                                │
│  ┌──────────────────────────────────────┐     │
│  │      Route Handler Layer             │     │
│  │   (routes::analyze_tx)               │     │
│  │                                      │     │
│  │  • Deserializes JSON request         │     │
│  │  • Validates input                   │     │
│  └──────────┬───────────────────────────┘     │
│             │                                  │
│             │ 2. Extract network & tx_hash     │
│             │                                  │
└─────────────┼──────────────────────────────────┘
              │
              ▼
┌────────────────────────────────────────────────┐
│      Blockchain Service Layer                  │
│   (services/blockchain.rs)                     │
│                                                │
│  ┌──────────────────────────────────────┐     │
│  │  fetch_transaction()                 │     │
│  │                                      │     │
│  │  • Validates network support         │     │
│  │  • Makes RPC call to blockchain      │     │
│  │    node (or mock data for now)       │     │
│  │  • Parses transaction details        │     │
│  │  • Returns structured JSON           │     │
│  └──────────┬───────────────────────────┘     │
│             │                                  │
│             │ 3. Transaction Details (JSON)    │
│             │    {                             │
│             │      "hash": "0x123...",         │
│             │      "from": "0xabc...",         │
│             │      "to": "0xdef...",           │
│             │      "value": "1.5 ETH",         │
│             │      "logs": [...]               │
│             │    }                             │
└─────────────┼──────────────────────────────────┘
              │
              ▼
┌────────────────────────────────────────────────┐
│         AI Analysis Service Layer              │
│      (services/ai.rs)                          │
│                                                │
│  ┌──────────────────────────────────────┐     │
│  │  analyze_transaction()               │     │
│  │                                      │     │
│  │  4a. Parse transaction logs          │     │
│  │  4b. Detect patterns (DEX, NFT, etc) │     │
│  │  4c. Call LLM API (future)           │     │
│  │  4d. Calculate risk score            │     │
│  │  4e. Generate explanation            │     │
│  │                                      │     │
│  │  Returns: AnalyzeTxResponse          │     │
│  └──────────┬───────────────────────────┘     │
│             │                                  │
│             │ 5. Analysis Result               │
│             │    {                             │
│             │      "tx_type": "DEX_SWAP",      │
│             │      "protocol": "Uniswap",      │
│             │      "risk_score": 0.2,          │
│             │      ...                         │
│             │    }                             │
└─────────────┼──────────────────────────────────┘
              │
              ▼
┌────────────────────────────────────────────────┐
│      Route Handler (Response)                  │
│                                                │
│  • Serializes AnalyzeTxResponse to JSON        │
│  • Sets HTTP status code (200 OK)             │
│  • Returns response to client                  │
└─────────────┬──────────────────────────────────┘
              │
              │ 6. HTTP 200 OK + JSON Response
              │
              ▼
┌──────────────────────────────────────────┐
│         Client Receives Result           │
│                                          │
│  {                                       │
│    "tx_hash": "0x123...",                │
│    "network": "ethereum-mainnet",        │
│    "tx_type": "DEX_SWAP",                │
│    "protocol": "Uniswap",                │
│    "risk_score": 0.2,                    │
│    "risk_reasons": [...],                │
│    "natural_language_explanation": "..." │
│  }                                       │
└──────────────────────────────────────────┘
```

## 🔍 Detailed Component Data Flow

### 1. Request Processing (routes.rs)

**Input**:

```rust
AnalyzeTxRequest {
    network: String,      // e.g., "ethereum-mainnet"
    tx_hash: String,      // e.g., "0x123abc..."
}
```

**Process**:

1. Axum extracts JSON from HTTP body
2. Deserializes into `AnalyzeTxRequest` struct
3. Validates required fields
4. Passes to blockchain service

**Error Handling**:

- Invalid JSON → 400 Bad Request
- Missing fields → 400 Bad Request

---

### 2. Blockchain Data Fetching (services/blockchain.rs)

**Input**:

- `network: &str` - Network identifier
- `tx_hash: &str` - Transaction hash

**Process**:

```rust
pub async fn fetch_transaction(
    network: &str,
    tx_hash: &str,
) -> Result<Value, BlockchainError>
```

**Steps**:

1. Validate network is supported
2. Make async RPC call to blockchain node:
   ```
   POST https://mainnet.infura.io/v3/YOUR_KEY
   {
     "jsonrpc": "2.0",
     "method": "eth_getTransactionByHash",
     "params": ["0x123..."],
     "id": 1
   }
   ```
3. Parse RPC response
4. Extract relevant transaction fields
5. Return structured JSON

**Output**:

```json
{
  "hash": "0x123...",
  "from": "0xabc...",
  "to": "0xdef...",
  "value": "1500000000000000000",
  "gas_used": 21000,
  "status": "success",
  "logs": [
    {
      "address": "0xUniswapV3Pool...",
      "topics": ["0x..."],
      "data": "0x..."
    }
  ]
}
```

**Error Handling**:

- Unsupported network → `BlockchainError::UnsupportedNetwork`
- RPC failure → `BlockchainError::RpcError`

---

### 3. AI Analysis (services/ai.rs)

**Input**:

- `network: &str` - Network identifier
- `tx_hash: &str` - Transaction hash
- `tx_details: &Value` - Raw transaction data from blockchain

**Process**:

```rust
pub async fn analyze_transaction(
    network: &str,
    tx_hash: &str,
    tx_details: &Value,
) -> Result<AnalyzeTxResponse, AiError>
```

**Steps**:

#### 3.1 Pattern Detection

```rust
// Check transaction logs for protocol signatures
let logs = tx_details["logs"].as_array();

// Detect DEX activity
if logs.contains("Uniswap") || logs.contains("0x...[Swap signature]") {
    tx_type = "DEX_SWAP";
}

// Detect NFT activity
if logs.contains("Transfer(address,address,uint256)") {
    tx_type = "NFT_TRANSFER";
}
```

#### 3.2 Protocol Identification

```rust
// Map contract addresses to known protocols
match contract_address {
    "0x1f9840..." => protocol = Some("Uniswap"),
    "0x7a250..." => protocol = Some("Compound"),
    _ => protocol = None,
}
```

#### 3.3 Risk Assessment

```rust
let mut risk_score = 0.0;
let mut risk_reasons = Vec::new();

// Check various risk factors
if is_new_contract(to_address) {
    risk_score += 0.3;
    risk_reasons.push("New contract (< 30 days old)");
}

if has_failed_transactions(from_address) {
    risk_score += 0.2;
    risk_reasons.push("Sender has failed transactions");
}

if is_high_value(value) {
    risk_score += 0.1;
    risk_reasons.push("High value transaction");
}
```

#### 3.4 LLM Call (Future Implementation)

```rust
// Prepare prompt for LLM
let prompt = format!(
    "Analyze this Ethereum transaction:\n\
     From: {}\n\
     To: {}\n\
     Value: {}\n\
     Logs: {:?}\n\
     \n\
     Provide a natural language explanation of what this transaction does.",
    tx_details["from"],
    tx_details["to"],
    tx_details["value"],
    tx_details["logs"]
);

// Call OpenAI/Anthropic API
let llm_response = call_llm_api(prompt).await?;

// Parse LLM response
let explanation = llm_response.content;
```

#### 3.5 Generate Response

```rust
AnalyzeTxResponse {
    tx_hash: tx_hash.to_string(),
    network: network.to_string(),
    tx_type,
    protocol,
    risk_score,
    risk_reasons,
    natural_language_explanation,
}
```

**Output**:

```rust
AnalyzeTxResponse {
    tx_hash: "0x123...",
    network: "ethereum-mainnet",
    tx_type: "DEX_SWAP",
    protocol: Some("Uniswap"),
    risk_score: 0.2,
    risk_reasons: vec!["Low risk factors detected"],
    natural_language_explanation: "This transaction swaps 1.5 ETH for USDC..."
}
```

**Error Handling**:

- LLM API failure → `AiError::LlmCallFailed`
- Invalid transaction data → Parse error

---

### 4. Response Serialization

**Input**: `AnalyzeTxResponse` struct

**Process**:

1. Axum serializes struct to JSON using Serde
2. Sets `Content-Type: application/json` header
3. Sets HTTP status code 200
4. Sends response to client

**Output**:

```http
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 456

{
  "tx_hash": "0x123...",
  "network": "ethereum-mainnet",
  "tx_type": "DEX_SWAP",
  "protocol": "Uniswap",
  "risk_score": 0.2,
  "risk_reasons": ["Low risk factors detected"],
  "natural_language_explanation": "This transaction..."
}
```

---

## 🔄 Error Flow

```
Client Request
     │
     ▼
Route Handler
     │
     ├──> Input Validation Error
     │         │
     │         └──> 400 Bad Request + Error Message
     │
     ▼
Blockchain Service
     │
     ├──> Unsupported Network
     │         │
     │         └──> 400 Bad Request + "Unsupported network: {network}"
     │
     ├──> RPC Error
     │         │
     │         └──> 400 Bad Request + "Failed to fetch tx details: {error}"
     │
     ▼
AI Service
     │
     ├──> LLM API Error
     │         │
     │         └──> 500 Internal Server Error + "AI analysis failed: {error}"
     │
     ▼
Success Response (200 OK)
```

---

## 📈 Performance Considerations

### Async Processing

- All I/O operations are async (tokio runtime)
- Non-blocking RPC calls
- Concurrent request handling

### Caching Strategy (Future)

```rust
// Cache transaction data to avoid repeated RPC calls
let cache_key = format!("{}:{}", network, tx_hash);
if let Some(cached) = cache.get(&cache_key) {
    return Ok(cached);
}
```

### Rate Limiting (Future)

```rust
// Limit requests per IP
if rate_limiter.check(client_ip).is_err() {
    return Err((StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"));
}
```

---

## 🔐 Data Security

### Sensitive Data Handling

- No transaction data is stored (stateless)
- API keys in environment variables only
- HTTPS for all external calls
- Input sanitization on all endpoints

### Data Validation

```rust
// Validate Ethereum address format
fn is_valid_eth_address(addr: &str) -> bool {
    addr.starts_with("0x") && addr.len() == 42
}

// Validate transaction hash format
fn is_valid_tx_hash(hash: &str) -> bool {
    hash.starts_with("0x") && hash.len() == 66
}
```

---

## 📊 Monitoring & Logging

### Request Logging

```rust
tracing::info!(
    "Analyzing transaction {} on {}",
    tx_hash,
    network
);
```

### Performance Metrics

```rust
let start = Instant::now();
let result = analyze_transaction(...).await;
let duration = start.elapsed();

tracing::info!(
    "Analysis completed in {:?}ms",
    duration.as_millis()
);
```

---

**Last Updated**: December 8, 2025
