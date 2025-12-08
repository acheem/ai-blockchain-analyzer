# Architecture Documentation

This document provides detailed architectural information about the AI Blockchain Analyzer system.

## 🏛️ System Architecture

### Architectural Style

**Layered Architecture** with clear separation of concerns:

```
┌───────────────────────────────────────────────────────┐
│              Presentation Layer                       │
│         (HTTP API - Axum Web Server)                  │
│  • Request validation                                 │
│  • Response serialization                             │
│  • Error handling                                     │
└────────────────────┬──────────────────────────────────┘
                     │
┌────────────────────▼──────────────────────────────────┐
│              Application Layer                        │
│            (Route Handlers - routes.rs)               │
│  • Business logic orchestration                       │
│  • Service coordination                               │
│  • Transaction management                             │
└────────────────────┬──────────────────────────────────┘
                     │
┌────────────────────▼──────────────────────────────────┐
│              Service Layer                            │
│         (Business Logic Services)                     │
│  ┌────────────────────┐  ┌─────────────────────┐     │
│  │ Blockchain Service │  │   AI Service        │     │
│  │ • RPC calls        │  │ • Pattern detection │     │
│  │ • Data parsing     │  │ • LLM integration   │     │
│  │ • Network handling │  │ • Risk assessment   │     │
│  └────────────────────┘  └─────────────────────┘     │
└────────────────────┬──────────────────────────────────┘
                     │
┌────────────────────▼──────────────────────────────────┐
│              External Layer                           │
│         (Third-party integrations)                    │
│  • Blockchain RPC nodes (Infura, Alchemy)             │
│  • LLM APIs (OpenAI, Anthropic)                       │
│  • Other data providers                               │
└───────────────────────────────────────────────────────┘
```

---

## 🧩 Component Architecture

### Core Components

#### 1. Web Server (main.rs)

```rust
┌─────────────────────────────────────────┐
│           Tokio Runtime                 │
│  ┌───────────────────────────────┐     │
│  │    Axum Server                │     │
│  │  • Port: 8080                 │     │
│  │  • Async handlers             │     │
│  │  • JSON middleware            │     │
│  │  • Tracing/logging            │     │
│  └───────────────────────────────┘     │
└─────────────────────────────────────────┘
```

**Responsibilities**:

- Initialize tracing subscriber
- Configure Axum router
- Bind to network address
- Handle graceful shutdown

**Key Technologies**:

- `tokio`: Async runtime
- `axum`: Web framework
- `tracing`: Structured logging
- `tower`: Middleware

---

#### 2. Routes Layer (routes.rs)

```rust
┌──────────────────────────────────────────┐
│         Route Handlers                   │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  GET /health                   │     │
│  │  → health()                    │     │
│  │  Returns: "OK"                 │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  POST /analyze_tx              │     │
│  │  → analyze_tx(Json)            │     │
│  │  Returns: Json<AnalyzeTxRes>   │     │
│  └────────────────────────────────┘     │
└──────────────────────────────────────────┘
```

**Responsibilities**:

- Extract and validate request data
- Coordinate service calls
- Handle errors and return appropriate HTTP status
- Serialize responses

**Error Handling Strategy**:

```rust
Result<Json<T>, (StatusCode, String)>
```

---

#### 3. Models Layer (models.rs)

```rust
┌──────────────────────────────────────────┐
│          Data Transfer Objects           │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  AnalyzeTxRequest              │     │
│  │  • network: String             │     │
│  │  • tx_hash: String             │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  AnalyzeTxResponse             │     │
│  │  • tx_hash: String             │     │
│  │  • network: String             │     │
│  │  • tx_type: String             │     │
│  │  • protocol: Option<String>    │     │
│  │  • risk_score: f32             │     │
│  │  • risk_reasons: Vec<String>   │     │
│  │  • natural_language_...        │     │
│  └────────────────────────────────┘     │
└──────────────────────────────────────────┘
```

**Design Principles**:

- Immutable data structures
- Serde serialization/deserialization
- Type safety
- Clear ownership semantics

---

#### 4. Blockchain Service (services/blockchain.rs)

```rust
┌──────────────────────────────────────────────┐
│       Blockchain Integration Layer           │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │  fetch_transaction()               │     │
│  │                                    │     │
│  │  Input:                            │     │
│  │    • network: &str                 │     │
│  │    • tx_hash: &str                 │     │
│  │                                    │     │
│  │  Process:                          │     │
│  │    1. Validate network             │     │
│  │    2. Build RPC request            │     │
│  │    3. Make async HTTP call         │     │
│  │    4. Parse response               │     │
│  │    5. Extract transaction data     │     │
│  │                                    │     │
│  │  Output:                           │     │
│  │    Result<Value, BlockchainError>  │     │
│  └────────────────────────────────────┘     │
│                                              │
│  Error Types:                                │
│  • UnsupportedNetwork(String)               │
│  • RpcError(String)                         │
└──────────────────────────────────────────────┘
```

**Future Enhancements**:

- Connection pooling
- Retry logic with exponential backoff
- Multiple RPC endpoint fallback
- WebSocket support for subscriptions

---

#### 5. AI Service (services/ai.rs)

```rust
┌──────────────────────────────────────────────┐
│         AI Analysis Engine                   │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │  analyze_transaction()             │     │
│  │                                    │     │
│  │  Input:                            │     │
│  │    • network: &str                 │     │
│  │    • tx_hash: &str                 │     │
│  │    • tx_details: &Value            │     │
│  │                                    │     │
│  │  Analysis Pipeline:                │     │
│  │    1. Pattern Detection            │     │
│  │       ├─ DEX identification        │     │
│  │       ├─ NFT detection             │     │
│  │       ├─ DeFi protocol mapping     │     │
│  │       └─ Contract analysis         │     │
│  │                                    │     │
│  │    2. Risk Assessment              │     │
│  │       ├─ Contract age check        │     │
│  │       ├─ Transaction history       │     │
│  │       ├─ Value analysis            │     │
│  │       └─ Gas pattern analysis      │     │
│  │                                    │     │
│  │    3. LLM Enhancement (future)     │     │
│  │       ├─ Prepare prompt            │     │
│  │       ├─ Call LLM API              │     │
│  │       ├─ Parse LLM response        │     │
│  │       └─ Generate explanation      │     │
│  │                                    │     │
│  │  Output:                           │     │
│  │    Result<AnalyzeTxResponse, ...>  │     │
│  └────────────────────────────────────┘     │
└──────────────────────────────────────────────┘
```

**Analysis Strategies**:

1. **Rule-Based Detection** (Current):

   - Pattern matching on logs
   - Contract address lookup
   - Heuristic risk scoring

2. **ML-Based Detection** (Future):

   - Trained models for transaction classification
   - Anomaly detection
   - Predictive risk modeling

3. **LLM-Enhanced Analysis** (Future):
   - Natural language understanding
   - Context-aware explanations
   - Complex pattern recognition

---

## 🔄 Request Flow Architecture

### Synchronous Flow

```
┌─────────┐
│ Client  │
└────┬────┘
     │ HTTP POST
     ▼
┌─────────────┐
│ Axum Server │
└──────┬──────┘
       │ Extract JSON
       ▼
┌────────────────┐
│ Route Handler  │
└──────┬─────────┘
       │ Call services
       ▼
┌───────────────────────────┐
│ Blockchain Service        │
│ • Async RPC call          │
│ • .await                  │
└──────┬────────────────────┘
       │ Transaction data
       ▼
┌───────────────────────────┐
│ AI Service                │
│ • Analyze patterns        │
│ • Calculate risk          │
│ • (Call LLM .await)       │
└──────┬────────────────────┘
       │ Analysis result
       ▼
┌────────────────┐
│ Route Handler  │
│ • Serialize    │
└──────┬─────────┘
       │ JSON response
       ▼
┌─────────────┐
│ Axum Server │
└──────┬──────┘
       │ HTTP 200
       ▼
┌─────────┐
│ Client  │
└─────────┘
```

### Async Architecture Benefits

1. **Non-blocking I/O**:

   ```rust
   // Multiple requests can be processed concurrently
   tokio::spawn(async move {
       handle_request(req).await
   });
   ```

2. **Resource Efficiency**:

   - Single-threaded async runtime
   - Thousands of concurrent connections
   - Low memory footprint

3. **Scalability**:
   - Horizontal scaling (multiple instances)
   - Load balancing ready
   - Cloud-native design

---

## 🔌 Integration Architecture

### External Service Integration

```
┌────────────────────────────────────────────┐
│         AI Blockchain Analyzer             │
└─────────────┬──────────────────────────────┘
              │
    ┌─────────┼─────────┐
    │         │         │
    ▼         ▼         ▼
┌────────┐ ┌─────┐ ┌──────────┐
│Infura  │ │Alchemy│ │OpenAI  │
│RPC     │ │RPC   │ │LLM API │
└────────┘ └─────┘ └──────────┘
```

### API Integration Patterns

#### 1. RPC Client Pattern

```rust
pub struct RpcClient {
    endpoint: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub async fn call(&self, method: &str, params: Vec<Value>)
        -> Result<Value, RpcError>
    {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?;

        response.json().await
    }
}
```

#### 2. LLM Client Pattern

```rust
pub struct LlmClient {
    api_key: String,
    base_url: String,
    model: String,
}

impl LlmClient {
    pub async fn complete(&self, prompt: &str)
        -> Result<String, LlmError>
    {
        let request = json!({
            "model": self.model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "max_tokens": 500
        });

        let response = reqwest::Client::new()
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        // Parse and return content
        let json: Value = response.json().await?;
        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}
```

---

## 🛡️ Security Architecture

### Security Layers

```
┌──────────────────────────────────────┐
│    Network Security                  │
│  • HTTPS/TLS encryption              │
│  • Firewall rules                    │
│  • DDoS protection                   │
└────────────┬─────────────────────────┘
             │
┌────────────▼─────────────────────────┐
│    Application Security              │
│  • Input validation                  │
│  • Request sanitization              │
│  • Error message filtering           │
└────────────┬─────────────────────────┘
             │
┌────────────▼─────────────────────────┐
│    Authentication (future)           │
│  • API key validation                │
│  • JWT tokens                        │
│  • Rate limiting per user            │
└────────────┬─────────────────────────┘
             │
┌────────────▼─────────────────────────┐
│    Data Security                     │
│  • No persistent storage             │
│  • Encrypted API keys                │
│  • Secure env variables              │
└──────────────────────────────────────┘
```

### Input Validation Strategy

```rust
fn validate_request(req: &AnalyzeTxRequest) -> Result<(), ValidationError> {
    // Validate network
    const SUPPORTED_NETWORKS: &[&str] = &[
        "ethereum-mainnet",
        "polygon-mainnet",
        "arbitrum-one",
    ];

    if !SUPPORTED_NETWORKS.contains(&req.network.as_str()) {
        return Err(ValidationError::InvalidNetwork);
    }

    // Validate transaction hash format
    if !req.tx_hash.starts_with("0x") || req.tx_hash.len() != 66 {
        return Err(ValidationError::InvalidTxHash);
    }

    Ok(())
}
```

---

## 📊 Scalability Architecture

### Horizontal Scaling

```
          ┌─────────────┐
          │ Load Balancer│
          └──────┬───────┘
                 │
    ┌────────────┼────────────┐
    │            │            │
    ▼            ▼            ▼
┌────────┐  ┌────────┐  ┌────────┐
│Instance│  │Instance│  │Instance│
│   1    │  │   2    │  │   3    │
└────────┘  └────────┘  └────────┘
```

**Deployment Strategies**:

1. Containerization (Docker)
2. Kubernetes orchestration
3. Auto-scaling based on load
4. Health check endpoints

### Caching Strategy (Future)

```
┌─────────────────────────────────┐
│        Redis Cache              │
│                                 │
│  Key Pattern:                   │
│    tx:{network}:{hash} → Data   │
│                                 │
│  TTL: 1 hour                    │
│  LRU eviction policy            │
└─────────────────────────────────┘
```

---

## 🔍 Observability Architecture

### Logging Strategy

```rust
// Structured logging with tracing
tracing::info!(
    tx_hash = %request.tx_hash,
    network = %request.network,
    "Processing transaction analysis request"
);

tracing::error!(
    error = %e,
    tx_hash = %request.tx_hash,
    "Failed to fetch transaction"
);
```

### Metrics Collection (Future)

```rust
// Prometheus metrics
counter!("http_requests_total",
    "method" => "POST",
    "endpoint" => "/analyze_tx"
);

histogram!("request_duration_seconds", duration);

gauge!("active_connections", active_count);
```

### Tracing Integration

```
┌──────────────────────────────────┐
│   Distributed Tracing            │
│   (Jaeger/Zipkin)                │
│                                  │
│   Request ID: abc-123            │
│   ├─ HTTP Handler: 50ms          │
│   ├─ Blockchain RPC: 200ms       │
│   └─ AI Analysis: 300ms          │
│                                  │
│   Total: 550ms                   │
└──────────────────────────────────┘
```

---

## 🧪 Testing Architecture

### Test Pyramid

```
        ┌────────────┐
        │   E2E      │  Integration tests
        └────────────┘
       ┌──────────────┐
       │ Integration  │  Service layer tests
       └──────────────┘
     ┌──────────────────┐
     │   Unit Tests     │  Function-level tests
     └──────────────────┘
```

**Test Coverage Goals**:

- Unit tests: 80%+
- Integration tests: Key workflows
- E2E tests: Critical paths

---

## 🚀 Deployment Architecture

### Docker Container

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/ai-blockchain-analyzer /usr/local/bin/
EXPOSE 8080
CMD ["ai-blockchain-analyzer"]
```

### Cloud Deployment Options

1. **AWS**:

   - ECS/EKS for container orchestration
   - Application Load Balancer
   - CloudWatch for logging

2. **Google Cloud**:

   - Cloud Run (serverless containers)
   - Cloud Load Balancing
   - Cloud Logging

3. **Azure**:
   - Azure Container Instances
   - Application Gateway
   - Azure Monitor

---

## 📈 Performance Targets

| Metric                 | Target     | Notes            |
| ---------------------- | ---------- | ---------------- |
| Response Time (p50)    | < 500ms    | Without LLM call |
| Response Time (p99)    | < 2s       | With LLM call    |
| Throughput             | 1000 req/s | Per instance     |
| Concurrent Connections | 10,000+    | Using Tokio      |
| Memory Usage           | < 100MB    | Base footprint   |
| CPU Usage              | < 50%      | At 80% load      |

---

**Last Updated**: December 8, 2025
