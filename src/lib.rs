#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

//! EVM state cache crate provides read/write access to EVM state with a fast, concurrent cache layer.
//!
//! The library uses in-memory concurrent cache implementations on top of hash maps.
//! They support full concurrency of retrievals and a high expected concurrency for updates.
//! They utilize a lock-free concurrent hash table as the central key-value storage.
//!
//! Accessing the EVM state uses a trait for freedom and flexibility in its implementation.
//! Implementations provided by this crate include:
//! * In-memory single-threaded ideal for testing.
//! * In-memory concurrent multithreaded ideal for benchmarking.
//! * Rust EVM database compatible.
//!
//! # Example
//!
//! ```
//! use revm::InMemoryDB;
//! use evm_state_cache::{CacheBuilder, CachedEvmStateRepository, EvictionPolicy, EvmStateRepository, RevmStateRepository};
//!
//! // Create cache with provided options
//! let cache = CacheBuilder::new()
//!      // Provide capacity based on available memory and usage
//!     .with_capacity(10)
//!     // Pick eviction policy most efficient for your application use-case
//!     .with_eviction_policy(EvictionPolicy::LeastRecentlyUsed)
//!     // Build cache instance
//!     .build();
//!
//! // Create repository with read/write access to a desired database
//! let repository = RevmStateRepository::new(InMemoryDB::default());
//!
//! // Combine the repository and cache into the cached repository
//! let repository = CachedEvmStateRepository::new(repository, cache);
//!
//! // Create an Ethereum address
//! let address = [0u8; 20];
//!
//! // Load account by given address
//! let account = repository.get(&address);
//!
//! // Enjoy loading from a fast, concurrent cache in subsequent calls for the cached address
//! ```

mod cache;
mod evm_state;
mod factory;

pub use cache::*;
pub use evm_state::*;
pub use factory::*;
