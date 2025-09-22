//! # ZEclipse - Privacy-Preserving Payment System for Solana
//! 
//! ZEclipse is a privacy-focused payment system built on Solana that enables
//! confidential transactions while maintaining the security and performance of
//! the Solana blockchain.
//!
//! ## Architecture Overview
//!
//! The system is composed of several key components:
//! - **Core Program**: Handles the main logic for private transactions
//! - **Poseidon Hashing**: Provides ZK-friendly cryptographic primitives
//! - **State Management**: Manages the on-chain state of transactions and accounts
//! - **Instruction Processing**: Handles the various transaction types
//!
//! ## Security Model
//!
//! ZEclipse uses zero-knowledge proofs to ensure transaction privacy while
//! maintaining the security guarantees of the Solana blockchain.
//!
//! ## Modules
//! - `instructions`: Implementation of the program's instructions
//! - `state`: Data structures for on-chain state
//! - `utils`: Utility functions and helpers
//! - `poseidon_validator`: Poseidon hashing implementation
//! - `errors`: Error types and handling
//!
//! ## Getting Started
//!
//! ### Prerequisites
//! - Rust 1.65.0 or later
//! - Solana CLI tools
//! - Anchor framework
//!
//! ### Building
//! ```bash
//! cargo build-bpf
//! ```
//!
//! ### Testing
//! ```bash
//! cargo test-bpf
//! ```
//!
//! For more details, see the [documentation](https://zeclipse.dev/docs).

// Standard imports
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

// Re-export commonly used types for easier access
pub use anchor_lang::{
    solana_program,
    system_program,
    prelude::*,
};

// Re-export program_error for consistency
pub use ProgramError;

// Core modules
pub mod errors;
pub mod state;
pub mod utils;
pub mod poseidon_validator;
pub mod poseidon_constants;
pub mod instructions;

// Re-export important types
pub use state::{
    TransferConfig,
    TransferState,
    TransferStateStatus,
    ZEclipseError,
};

// Re-export instructions for easier access
pub use instructions::{
    initialize,
    execute_hop,
    batch_hop,
    finalize,
    config_update,
    refund,
    reveal_fake,
};

// Re-export utils for external use
pub use utils::{
    TransferContext,
    Loggable,
};

// Program ID
declare_id!("B1ack111111111111111111111111111111111111111");

// Core Solana imports for compatibility and easier maintenance
pub mod solana_imports;

// Formal verification modules, only available with the "verification" feature
#[cfg(feature = "verification")]
pub mod verification;
