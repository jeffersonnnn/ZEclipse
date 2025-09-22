# AGENTS.md - ZEclipse Development Guide

## Build & Test Commands

**Rust (Solana Programs):**
```bash
cargo check --package zeclipse --features no-entrypoint
cargo test --package zeclipse --lib --features no-entrypoint
cargo build-bpf  # Build for Solana BPF target
```

**TypeScript (SDK & Apps):**
```bash
cd app
npm install && npm run build
npm test  # Run Jest tests
npm run dev  # Run CLI with ts-node
```

**Single Test Execution:**
- Rust: `cargo test --package zeclipse --lib <test_name> --features no-entrypoint -- --nocapture`
- TypeScript: `npm test -- <test_pattern>` (from `app/` directory)

## Architecture Overview

**ZEclipse** is a Solana privacy system using Zero-Knowledge Proofs (ZKPs) and multi-hop transaction routing. Core modules:
- **programs/zeclipse**: Anchor-based Solana program with instructions (initialize, execute_hop, batch_hop, finalize, refund, reveal_fake, config_update)
- **programs/zeclipse-anchor**: Anchor account structures
- **app**: TypeScript SDK with connectors, temporal obfuscation, proof generators
- **poseidon_standalone**: Custom Poseidon hash implementation (ZKP-friendly)
- **poseidon_validator**: Validation and testing for Poseidon

**Key Dependencies**: Anchor 0.28.0, solana-program 1.18.26, solana-poseidon 2.2.14, @solana/web3.js 1.73.0

## Code Style & Conventions

**Rust (programs/):**
- Module organization: instructions/, state/, utils/, verification/
- Error handling: Custom `BlackoutError` enum with #[error_code] macro
- Comments: Doc comments (///) for public APIs, module-level //! for overviews
- Naming: snake_case functions, PascalCase types/structs
- Types: Strong typing with anchor_lang prelude; use bytemuck for serialization

**TypeScript (app/):**
- Use strict mode; target ES2020
- Interfaces for public contracts; PascalCase for classes/interfaces
- Error handling: Use try-catch; return Result-like objects with `success` field
- Comments: JSDoc (/** */) for exported functions
- Async/await for all async operations; Promises over callbacks
- Logging: Use console methods; structured log objects where possible

**General:**
- 2-space indentation (TypeScript), 4-space (Rust)
- No trailing semicolons (TypeScript files use ASI)
- Imports: Group by external → internal, alphabetically within groups
- ZKP-related code requires careful attention to cryptographic soundness (marked CRITICAL in docs)

## Important Notes

⚠️ **Security Warnings** (from DOCUMENTATION.md):
- Current ZKP implementations (HyperPlonk, range proofs, Merkle verification) are **NOT production-ready** and contain placeholders
- Poseidon hashing is critical for ZK soundness; SHA-256 cannot replace it
- System is in BETA; security audit still in progress

**Build quirks**: Use `--features no-entrypoint` for Anchor macro compatibility with custom crypto libs.
