# Anchor Vault

A simple and secure SOL vault program built with the Anchor framework on Solana. This program allows users to initialize a personal vault, deposit SOL, withdraw SOL, and close their vault when needed.

## Features

- **Initialize**: Create a personal vault account with PDA-based architecture
- **Deposit**: Transfer SOL into your vault
- **Withdraw**: Retrieve SOL from your vault
- **Close**: Close your vault and reclaim all remaining SOL and rent

## Program Architecture

The program uses two Program Derived Addresses (PDAs):
- **Vault State PDA**: Stores the bump seeds for both PDAs (derived from `["state", user.publicKey]`)
- **Vault PDA**: Holds the deposited SOL (derived from `["vault", vaultState.publicKey]`)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.89.0)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (version 0.32.1)
- [Node.js](https://nodejs.org/) and [Yarn](https://yarnpkg.com/)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd anchor_vault
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Testing

Run the test suite:
```bash
anchor test
```

The tests cover:
- Vault initialization
- SOL deposits
- SOL withdrawals
- Vault closure

## Usage

### Initialize a Vault

```typescript
await program.methods
  .initialize()
  .accounts({
    user: userPublicKey,
    vaultState: vaultStatePda,
    vault: vaultPda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Deposit SOL

```typescript
const depositAmount = 1 * LAMPORTS_PER_SOL; // 1 SOL

await program.methods
  .deposit(new BN(depositAmount))
  .accounts({
    user: userPublicKey,
    vault: vaultPda,
    vaultState: vaultStatePda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Withdraw SOL

```typescript
const withdrawAmount = 0.5 * LAMPORTS_PER_SOL; // 0.5 SOL

await program.methods
  .withdraw(new BN(withdrawAmount))
  .accounts({
    user: userPublicKey,
    vault: vaultPda,
    vaultState: vaultStatePda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Close Vault

```typescript
await program.methods
  .close()
  .accounts({
    user: userPublicKey,
    vault: vaultPda,
    vaultState: vaultStatePda,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## Project Structure

```
anchor_vault/
├── programs/
│   └── anchor_vault/
│       └── src/
│           └── lib.rs          # Main program logic
├── tests/
│   └── anchor_vault.ts         # Test suite
├── migrations/
│   └── deploy.ts               # Deployment script
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Rust workspace configuration
└── package.json                # Node.js dependencies
```

## Program ID

```
6RpYNZhk25mktpRowY71JzGsyQtRZTbxPN4n2FE1ga8w
```

## Security Considerations

- The vault uses PDA-based ownership, ensuring only the user who initialized the vault can interact with it
- Rent-exempt balance is automatically funded during initialization
- All operations require the user's signature
- The close instruction transfers all remaining lamports back to the user

## Development

### Lint Code

```bash
yarn lint
```

### Fix Formatting

```bash
yarn lint:fix
```

## License

ISC

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
