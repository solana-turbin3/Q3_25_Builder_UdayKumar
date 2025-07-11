# Solana Turbine

**Builder:** Uday Kumar  
**Project:** Solana Turbine  
**Quarter:** 2025 Q3  

## 🚀 Overview

This repository contains the assignments and learnings taught inside Solana Turbine Q3 classes.

## 📁 Project Structure

```
Turbine/
├── pre-requistes/          # Pre requistes tasks to enroll in this program.
│   ├── rust-preq/         # Rust-based Solana operations
│   └── ts-preq/           # TypeScript-based Solana operations
├── week_1/                # NFT Development
│   └── minting_nft_using_umi/  # NFT minting with Umi framework
└── week_2/                # DeFi Applications
    ├── anchor-amm/        # Automated Market Maker (AMM)
    └── vault/             # Vault smart contract
```

## 🛠️ Prerequisites

### Rust Prerequisites (`pre-requistes/rust-preq/`)
- **Solana SDK**: Core Solana development tools
- **Solana Client**: Client-side operations
- **Solana Program**: Smart contract development
- **BS58**: Base58 encoding utilities

**Key Features:**
- Basic Solana account operations
- Transaction handling
- Program interaction

### TypeScript Prerequisites (`pre-requistes/ts-preq/`)
- **Anchor Framework**: Solana development framework
- **Solana Web3.js**: JavaScript SDK for Solana
- **BS58**: Base58 encoding
- **Prompt-sync**: Interactive CLI tools

**Available Scripts:**
- `keygen`: Generate Solana keypairs
- `airdrop`: Request SOL airdrops
- `transfer`: Transfer SOL between accounts
- `enroll`: Enroll in programs

## 🎨 Week 1: NFT Development

### NFT Minting with Umi Framework (`week_1/minting_nft_using_umi/`)

**Technologies Used:**
- **Umi Framework**: Modern Solana development framework
- **Metaplex**: NFT metadata standards
- **Irys Uploader**: Decentralized file storage
- **TypeScript**: Type-safe development

**Features:**
- NFT metadata creation
- Image upload to decentralized storage
- NFT minting with custom attributes
- Metaplex token metadata integration

## 🏦 Week 2: DeFi Applications

### Automated Market Maker (AMM) (`week_2/anchor-amm/`)

**Program ID:** `DKsA1wFJ2UeNfKgcgAMqErfR3b6JpXGBBXfWaLLqP16w`

**Features:**
- Liquidity pool management
- Token swapping functionality
- Price calculation algorithms
- Yield farming mechanics

**Technologies:**
- **Anchor Framework**: Solana program development
- **TypeScript**: Client-side integration
- **Mocha**: Testing framework

### Vault Smart Contract (`week_2/vault/`)

**Program ID:** `4vrVwqf5txbavZ5viVRbTsPS8rTB986v3PWBtwbnLrXE`

**Features:**
- Asset management and custody
- Multi-signature functionality
- Withdrawal controls
- Security mechanisms

**Technologies:**
- **Anchor Framework**: Solana program development
- **TypeScript**: Client-side integration
- **Mocha**: Testing framework

## 🚀 Getting Started

### Prerequisites Installation

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Solana CLI:**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.15.2/install)"
   ```

3. **Install Node.js and Yarn:**
   ```bash
   npm install -g yarn
   ```

### Setup Instructions

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd Turbine
   ```

2. **Install TypeScript prerequisites:**
   ```bash
   cd pre-requistes/ts-preq
   yarn install
   ```

3. **Install Rust prerequisites:**
   ```bash
   cd ../rust-preq
   cargo build
   ```

4. **Setup Week 1 - NFT Project:**
   ```bash
   cd ../../week_1/minting_nft_using_umi
   yarn install
   ```

5. **Setup Week 2 - DeFi Projects:**
   ```bash
   cd ../../week_2/anchor-amm
   yarn install
   
   cd ../vault
   yarn install
   ```

## 🧪 Testing

### Running Tests

**Anchor AMM Tests:**
```bash
cd week_2/anchor-amm
yarn test
```

**Vault Tests:**
```bash
cd week_2/vault
yarn test
```

### Local Development

1. **Start local validator:**
   ```bash
   solana-test-validator
   ```

2. **Build and deploy programs:**
   ```bash
   anchor build
   anchor deploy
   ```

## 📚 Learning Path

### Week 1: NFT Fundamentals
- Understanding NFT standards (Metaplex)
- Metadata creation and storage
- Umi framework basics
- Decentralized file uploads

### Week 2: DeFi Applications
- Automated Market Maker concepts
- Liquidity pool mechanics
- Vault security patterns
- Multi-signature implementations

## 🔧 Development Tools

- **Anchor Framework**: Solana program development
- **Umi Framework**: Modern Solana development
- **Metaplex**: NFT standards and tools
- **TypeScript**: Type-safe development
- **Rust**: Smart contract development
- **Solana CLI**: Blockchain interaction

## 📝 License

This project is licensed under the MIT License.

## 👨‍💻 Builder Information

**Name:** Uday Kumar  
**Project:** Solana Turbine  
**Quarter:** 2025 Q3  

---
