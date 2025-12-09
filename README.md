# AI Blockchain Analyzer

A high-performance REST API service built with Rust that analyzes blockchain transactions using AI/LLM to provide intelligent insights about transaction types, protocols, risk assessment, and natural language explanations.

## 🚀 Features

- **Transaction Analysis**: Analyze blockchain transactions with AI-powered insights
- **Multi-Network Support**: Designed to support multiple blockchain networks (currently Ethereum mainnet)
- **Risk Assessment**: Automated risk scoring and risk factor identification
- **Protocol Detection**: Identifies DeFi protocols and transaction patterns
- **Natural Language Explanations**: Human-readable transaction summaries
- **High Performance**: Built with Rust and async runtime (Tokio) for maximum efficiency

## 📋 Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      Client/User                             │
│                    (HTTP Requests)                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    Axum Web Server                           │
│                   (Port 8080)                                │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    Routes Layer                              │
│  ┌─────────────────┐     ┌───────────────────────┐          │
│  │  /health        │     │  /analyze_tx          │          │
│  │  (GET)          │     │  (POST)               │          │
│  └─────────────────┘     └───────────────────────┘          │
└────────────────────────┬──────────────┬──────────────────────┘
                         │              │
                ┌────────▼──────┐  ┌────▼──────────┐
                │   Blockchain  │  │   AI Service  │
                │   Service     │  │               │
                └────────┬──────┘  └────┬──────────┘
                         │              │
                         ▼              ▼
            ┌───────────────────────────────────┐
            │       External Services           │
            │  • Blockchain RPC Nodes           │
            │  • LLM APIs (OpenAI/Anthropic)    │
            └───────────────────────────────────┘
```

## 🔄 Data Flow

```
1. Client Request
   ↓
   POST /analyze_tx
   {
     "network": "ethereum-mainnet",
     "tx_hash": "0x123abc..."
   }
   ↓
2. Route Handler (routes.rs)
   ↓
3. Blockchain Service (services/blockchain.rs)
   • Fetches transaction details from RPC node
   • Returns structured transaction data
   ↓
4. AI Service (services/ai.rs)
   • Analyzes transaction data
   • Calls LLM for interpretation
   • Generates risk score
   • Creates natural language explanation
   ↓
5. Response to Client
   {
     "tx_hash": "0x123abc...",
     "network": "ethereum-mainnet",
     "tx_type": "DEX_SWAP",
     "protocol": "Uniswap",
     "risk_score": 0.2,
     "risk_reasons": ["Low risk factors detected"],
     "natural_language_explanation": "This transaction..."
   }
```

## 🏗️ Project Structure

```
ai-blockchain-analyzer/
├── src/
│   ├── main.rs              # Application entry point & server setup
│   ├── routes.rs            # HTTP route handlers
│   ├── models.rs            # Data models (Request/Response types)
│   └── services/
│       ├── mod.rs           # Service module exports
│       ├── blockchain.rs    # Blockchain RPC interaction logic
│       └── ai.rs            # AI/LLM analysis logic
├── Cargo.toml               # Project dependencies & metadata
├── README.md                # This file
├── ARCHITECTURE.md          # Detailed architecture documentation
├── DATAFLOW.md              # Data flow diagrams & explanations
└── GIT_WORKFLOW.md          # Git workflow & branching strategy
```

## 🛠️ Technology Stack

- **Language**: Rust (2024 edition)
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Serialization**: Serde + serde_json
- **HTTP Client**: Reqwest (with rustls-tls)
- **Error Handling**: thiserror
- **Logging**: tracing + tracing-subscriber
- **Configuration**: dotenvy (environment variables)

## 📦 Installation

### Prerequisites

- Rust 1.75+ (2024 edition support)
- Cargo

### Setup

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd ai-blockchain-analyzer
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Set up environment variables** (optional):
   ```bash
   cp .env.example .env
   # Edit .env with your API keys and configuration
   ```

## 🚀 Running the Application

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
cargo build --release
./target/release/ai-blockchain-analyzer
```

The server will start on `http://0.0.0.0:8080`

## 📡 API Endpoints

### Health Check

**GET** `/health`

Returns server status.

**Response**:

```
OK
```

### Analyze Transaction

**POST** `/analyze_tx`

Analyzes a blockchain transaction using AI.

**Request Body**:

```json
{
  "network": "ethereum-mainnet",
  "tx_hash": "0x1234567890abcdef..."
}
```

**Response** (200 OK):

```json
{
  "tx_hash": "0x1234567890abcdef...",
  "network": "ethereum-mainnet",
  "tx_type": "DEX_SWAP",
  "protocol": "Uniswap",
  "risk_score": 0.2,
  "risk_reasons": [
    "Standard DEX swap pattern detected",
    "No suspicious contract interactions"
  ],
  "natural_language_explanation": "This transaction is a token swap on Uniswap V3..."
}
```

**Error Responses**:

- `400 Bad Request`: Invalid network or transaction hash
- `500 Internal Server Error`: AI analysis failed

## 🧪 Testing

### Using curl

```bash
# Health check
curl http://localhost:8080/health

# Analyze transaction
curl -X POST http://localhost:8080/analyze_tx \
  -H "Content-Type: application/json" \
  -d '{
    "network": "ethereum-mainnet",
    "tx_hash": "0xabc123def456..."
  }'
```

### Using HTTP files (REST Client)

Create a file `test.http`:

```http
### Health Check
GET http://localhost:8080/health

### Analyze Transaction
POST http://localhost:8080/analyze_tx
Content-Type: application/json

{
  "network": "ethereum-mainnet",
  "tx_hash": "0x1234567890abcdef"
}
```

## 🗺️ Roadmap

### Phase 1 (Current - MVP)

- [x] Basic REST API structure
- [x] Mock blockchain data fetching
- [x] Rule-based transaction analysis
- [x] Health check endpoint

### Phase 2 (Next)

- [ ] Real blockchain RPC integration (ethers-rs)
- [ ] LLM integration (OpenAI/Anthropic)
- [ ] Advanced risk assessment models
- [ ] Transaction pattern recognition

### Phase 3 (Future)

- [ ] Multi-chain support (Polygon, BSC, Arbitrum)
- [ ] WebSocket support for real-time analysis
- [ ] Transaction monitoring & alerts
- [ ] Historical analysis & trends
- [ ] ML model training on transaction patterns

## 🔒 Security Considerations

- API keys should be stored in environment variables
- Input validation on all endpoints
- Rate limiting (to be implemented)
- Secure RPC endpoints (HTTPS only)
- Error messages don't leak sensitive information

## 🤝 Contributing

Please read [GIT_WORKFLOW.md](GIT_WORKFLOW.md) for our Git workflow and branching strategy.

### Quick Start

1. Fork the repository
2. Create a feature branch from `dev`: `git checkout -b feature/your-feature`
3. Make your changes and commit following [Conventional Commits](https://www.conventionalcommits.org/)
4. Write tests for your changes
5. Push to your fork: `git push origin feature/your-feature`
6. Create a Pull Request to the `dev` branch

### Development Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License.

## 👥 Authors

- ar cheemala - Initial work

## 🙏 Acknowledgments

- Axum web framework team
- Tokio async runtime
- Rust community

## 📞 Support

For issues and questions:

- Open an issue on GitHub
- Contact: abbi.cheemala@gmail.com

---

**Built with ❤️ and Rust**

