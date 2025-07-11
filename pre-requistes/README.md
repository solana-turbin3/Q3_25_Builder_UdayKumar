# Solana Turbine Program - Prerequisites Assignment

This project demonstrates basic Solana development capabilities through two implementations: Rust and TypeScript.

### Rust Implementation (`rust-preq/`)

The Rust implementation can:

- **Generate Solana keypairs** - Creates new wallet keypairs for testing
- **Convert wallet formats** - Transforms between Base58 and JSON wallet formats
- **Request airdrops** - Gets test SOL from Solana devnet
- **Transfer SOL** - Sends entire wallet balance to specified addresses
- **Submit program enrollment** - Interacts with the Turbine program for enrollment

### TypeScript Implementation (`ts-preq/`)

The TypeScript implementation can:

- **Generate Solana keypairs** - Creates new wallet keypairs for testing
- **Convert wallet formats** - Transforms between Base58 and JSON wallet formats
- **Request airdrops** - Gets test SOL from Solana devnet
- **Transfer SOL** - Sends entire wallet balance to specified addresses
- **Submit program enrollment** - Interacts with the Turbine program for enrollment

## Network Configuration

Both implementations connect to Solana devnet using:
- RPC URL: `https://api.devnet.solana.com`
- Program ID: `TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM`
- Collection: `5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2`
- MPL Core Program: `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d`
