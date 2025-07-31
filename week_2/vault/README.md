# Solana Vault Program

A secure Solana program built with Anchor framework that implements a vault system for depositing and withdrawing SOL tokens. This project demonstrates advanced Solana development concepts including Program Derived Addresses (PDAs), Cross-Program Invocations (CPIs), and account management.

## 🏗️ Project Structure

```
vault/
├── Anchor.toml              # Anchor framework configuration
├── Cargo.toml               # Rust workspace configuration
├── package.json             # Node.js dependencies and scripts
├── tsconfig.json           # TypeScript configuration
├── programs/
│   └── vault/
│       ├── Cargo.toml      # Program-specific dependencies
│       ├── src/
│       │   └── lib.rs      # Main program logic
│       └── Xargo.toml      # Cross-compilation configuration
├── tests/
│   └── vault.ts            # Test suite
├── migrations/
│   └── deploy.ts           # Deployment script
├── app/                    # Frontend application (empty)
└── target/                 # Build artifacts
```

## 🚀 Features

### Core Functionality
- **Vault Initialization**: Create a new vault for a user with proper account setup
- **SOL Deposits**: Securely deposit SOL into the vault
- **SOL Withdrawals**: Withdraw deposited SOL (up to deposit limit)
- **Account Management**: Proper PDA (Program Derived Address) management
- **Security**: Built-in validation and error handling

### Technical Features
- **PDA (Program Derived Address)**: Uses deterministic address generation for vault accounts
- **CPI (Cross-Program Invocation)**: Integrates with Solana's System Program for transfers
- **Account Validation**: Comprehensive account validation and security checks
- **Error Handling**: Custom error types for better user experience

## 📋 Prerequisites

Before running this project, ensure you have the following installed:

- **Rust** (latest stable version)
- **Node.js** (v16 or higher)
- **Yarn** package manager
- **Solana CLI** tools
- **Anchor CLI**

### Installation Commands

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install Node.js dependencies
yarn install
```

## 🛠️ Setup Instructions

### 1. Clone and Navigate
```bash
git clone <repository-url>
cd vault
```

### 2. Install Dependencies
```bash
yarn install
```

### 3. Build the Program
```bash
anchor build
```

### 4. Configure Solana Wallet
```bash
# Generate a new wallet (if you don't have one)
solana-keygen new

# Set your wallet as the default
solana config set --keypair ~/.config/solana/id.json
```

### 5. Configure Network
The project is configured to use Solana devnet by default. You can change this in `Anchor.toml`:

```toml
[provider]
cluster = "devnet"  # Change to "localnet" for local testing
wallet = "~/.config/solana/id.json"
```

## 🧪 Testing

### Run Tests
```bash
# Run all tests
anchor test

# Or use the npm script
yarn test
```

### Test Coverage
The test suite includes:
- Vault initialization
- Account creation validation
- Error handling scenarios

## 📚 Program Architecture

### Account Structure

#### VaultState Account
```rust
pub struct VaultState {
    pub vault_bump: u8,        // PDA bump for vault account
    pub state_bump: u8,        // PDA bump for state account
    pub owner: Pubkey,         // Owner of the vault
    pub total_deposits: u64    // Total SOL deposited
}
```

#### PDA Seeds
- **Vault State**: `[b"state", user.key().as_ref()]`
- **Vault Account**: `[b"vault", vault_state.key().as_ref()]`

### Instructions

#### 1. Initialize
Creates a new vault for a user:
- Creates a `VaultState` account
- Creates a `Vault` account (PDA)
- Sets up initial state with zero deposits

#### 2. Deposit
Allows users to deposit SOL:
- Validates user has sufficient balance
- Transfers SOL from user to vault
- Updates total deposits counter

#### 3. Withdraw
Allows users to withdraw their deposited SOL:
- Validates withdrawal amount doesn't exceed deposits
- Transfers SOL from vault to user
- Updates total deposits counter

### Security Features

#### Error Handling
```rust
pub enum CustomError {
    #[msg("You are trying to withdraw more than you deposited.")]
    InsufficientDeposits,
    #[msg("You do not have enough SOL to make this deposit.")]
    InsufficientUserBalance
}
```

#### Account Validation
- Ensures proper account ownership
- Validates PDA derivation
- Checks account mutability requirements

## 🔧 Configuration

### Anchor.toml
```toml
[programs.localnet]
vault = "4vrVwqf5txbavZ5viVRbTsPS8rTB986v3PWBtwbnLrXE"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

### Key Configuration Options
- **Program ID**: `4vrVwqf5txbavZ5viVRbTsPS8rTB986v3PWBtwbnLrXE`
- **Network**: Configured for devnet (can be changed to localnet)
- **Test Timeout**: 1,000,000ms for comprehensive testing

## 🚀 Deployment

### Local Development
```bash
# Start local validator
solana-test-validator

# Deploy to localnet
anchor deploy
```

### Devnet Deployment
```bash
# Switch to devnet
solana config set --url devnet

# Deploy to devnet
anchor deploy
```

### Mainnet Deployment
```bash
# Switch to mainnet
solana config set --url mainnet-beta

# Deploy to mainnet (ensure sufficient SOL for deployment)
anchor deploy
```

## 📖 Usage Examples

### Client-Side Integration

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/anchor_vault";

// Initialize provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.vault as Program<Vault>;

// Initialize vault
const vaultState = anchor.web3.Keypair.generate();
await program.methods
  .initialize()
  .accounts({
    vaultState: vaultState.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId
  })
  .signers([vaultState])
  .rpc();

// Deposit SOL
await program.methods
  .deposit(new anchor.BN(1000000000)) // 1 SOL
  .accounts({
    user: provider.wallet.publicKey,
    vault: vaultPDA,
    vaultState: vaultState.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId
  })
  .rpc();
```

## 🔍 Troubleshooting

### Common Issues

1. **Build Errors**
   ```bash
   # Clean and rebuild
   anchor clean
   anchor build
   ```

2. **Test Failures**
   ```bash
   # Ensure local validator is running
   solana-test-validator
   
   # Run tests with verbose output
   anchor test --verbose
   ```

3. **Deployment Issues**
   ```bash
   # Check SOL balance
   solana balance
   
   # Airdrop SOL (devnet only)
   solana airdrop 2
   ```