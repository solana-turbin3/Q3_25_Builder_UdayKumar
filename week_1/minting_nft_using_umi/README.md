# NFT Minting with Umi Framework

**Project:** Solana NFT Minting using Umi Framework  
**Week:** 1  
**Course:** Solana Turbine Q3 2025  

A complete NFT minting project demonstrating how to create, upload, and mint NFTs on Solana using the modern Umi framework and Metaplex standards.

## 🎨 Overview

This project showcases the complete NFT creation pipeline:
1. **Image Upload**: Upload NFT images to decentralized storage (Irys)
2. **Metadata Creation**: Generate NFT metadata following Metaplex standards
3. **NFT Minting**: Mint NFTs on Solana devnet using the Umi framework

## 🏗️ Project Structure

```
minting_nft_using_umi/
├── src/
│   ├── nft_image.ts      # Image upload to Irys
│   ├── nft_metadata.ts   # Metadata creation and upload
│   └── nft_mint.ts       # NFT minting on Solana
├── image2.png            # Sample NFT image
├── turbine-wallet.json   # Wallet configuration
├── package.json          # Dependencies
└── tsconfig.json         # TypeScript configuration
```

## 🛠️ Technologies Used

- **Umi Framework**: Modern Solana development framework
- **Metaplex**: NFT metadata standards and tools
- **Irys Uploader**: Decentralized file storage
- **TypeScript**: Type-safe development
- **Solana Devnet**: Test network for development

## 🚀 Getting Started

### Prerequisites

1. **Node.js and Yarn**: Make sure you have Node.js and Yarn installed
2. **Solana CLI**: Install Solana CLI tools
3. **Wallet**: You need a Solana wallet with some devnet SOL

### Installation

1. **Install dependencies:**
   ```bash
   yarn install
   ```

2. **Configure your wallet:**
   - Replace the contents of `turbine-wallet.json` with your wallet keypair
   - Ensure you have devnet SOL in your wallet

### Running the Project

The project consists of three main scripts that should be run in sequence:

#### 1. Upload Image (`nft_image.ts`)

Uploads the NFT image to Irys decentralized storage:

```bash
npx ts-node src/nft_image.ts
```

**Output:** Image URI that will be used in metadata

#### 2. Create Metadata (`nft_metadata.ts`)

Creates and uploads NFT metadata following Metaplex standards:

```bash
npx ts-node src/nft_metadata.ts
```

**Output:** Metadata URI that will be used for minting

#### 3. Mint NFT (`nft_mint.ts`)

Mints the NFT on Solana devnet:

```bash
npx ts-node src/nft_mint.ts
```

**Output:** Transaction signature and mint address

## 🔧 Configuration

### Wallet Setup

1. **Generate a new keypair** (if needed):
   ```bash
   solana-keygen new --outfile turbine-wallet.json
   ```

2. **Get devnet SOL**:
   ```bash
   solana airdrop 2 --url devnet
   ```

### Network Configuration

- **RPC Endpoint**: `https://api.devnet.solana.com`
- **Irys Endpoint**: `https://devnet.irys.xyz/`
- **Network**: Solana Devnet

## 📊 NFT Details

- **Name**
- **Symbol**
- **Description**
- **Royalty**
- **Attributes**
- **Image Format**

## 🔍 Verification

After minting, you can verify your NFT:

1. **Check transaction**: Use the provided Solana Explorer link
2. **View NFT**: Search for your mint address on Solana Explorer
3. **Metadata**: Verify metadata URI resolves correctly

## 📚 Learning Resources

- [Umi Framework Documentation](https://docs.metaplex.com/umi/)
- [Metaplex Token Metadata](https://docs.metaplex.com/programs/token-metadata/)
- [Irys Uploader](https://docs.irys.xyz/)
- [Solana Devnet](https://docs.solana.com/clusters/devnet)

## 🤝 Contributing

This is a learning project for the Solana Turbine course. Feel free to:
- Improve error handling
- Add more NFT attributes
- Implement batch minting
- Add unit tests

