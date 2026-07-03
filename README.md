# Zero-Contracts

Zero-Contracts is a set of Soroban smart contracts for the Stellar network that enables **zero-knowledge (ZK) privacy-preserving transactions** with built-in auditability and compliance features.

## Overview

This project implements two core smart contracts working together with a Stellar Privacy Pool (SPP) to enable shielded transactions while maintaining regulatory compliance:

1. **Audit Registry** - Stores encrypted audit notes and user viewing keys
2. **Orchestrator** - Routes ZK shielded transactions through a privacy pool and mirrors audit metadata

## Zero-Knowledge Features

Zero-Contracts is designed to integrate with zero-knowledge proof systems on Stellar:

- **Shielded Transactions**: Works with SPP (Stellar Privacy Pool) to enable private asset transfers
- **ZK Proof Verification**: Accepts and routes zero-knowledge proofs for transaction validity
- **Commitments & Nullifiers**: Manages cryptographic commitments (shielded balances) and nullifiers (spent note tracking)
- **Auditability with Privacy**: Stores encrypted audit metadata that can be accessed by authorized auditors without compromising transaction privacy
- **Viewing Keys**: Users can register incoming-viewing public keys (IVPK) to grant auditors access to their encrypted notes

## Architecture

### Audit Registry Contract (`contracts/audit-registry/`)
- **Key Features**:
  - Register and retrieve user incoming-viewing public keys (IVPK)
  - Store encrypted audit notes against ZK commitments
  - Uses persistent storage for viewing keys (never lost to expiry)
  - Uses temporary storage with TTL extensions for audit payloads (~30 days)
  - Encrypted note structure includes ephemeral public key, nonce, and ciphertext

### Orchestrator Contract (`contracts/orchestrator/`)
- **Key Features**:
  - Atomic transaction routing between SPP privacy pool and audit registry
  - Ensures audit compliance state always matches pool commitment set
  - Validates payload/commitment length consistency
  - Requires depositor authentication
  - Routes ZK proofs, public amounts, commitments, and nullifiers to the privacy pool
  - Stores corresponding encrypted audit payloads for each new commitment

## Installation & Build

### Prerequisites
- Rust toolchain
- Soroban SDK 22.0.0
- Stellar CLI (optional, for deployment)

### Build Contracts

```bash
# Build all contracts in release mode
cargo build --release

# Build with debug logs
cargo build --profile release-with-logs
```

## Usage

### Development
The contracts can be tested using Soroban's test utilities. Both contracts include unit tests.

```bash
# Run tests
cargo test
```

## Project Structure

```
Zero-Contracts/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock
├── contracts/
│   ├── audit-registry/     # Audit Registry contract
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── orchestrator/       # Orchestrator contract
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
└── target/                 # Build artifacts
```

## License

[Add your license information here]
