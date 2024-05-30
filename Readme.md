# EVM state cache

This crate provides read/write access to EVM state with a fast, concurrent cache layer.

The library uses in-memory concurrent cache implementations on top of hash maps.
They support full concurrency of retrievals and a high expected concurrency for updates.
They utilize a lock-free concurrent hash table as the central key-value storage.

Accessing the EVM state uses a trait for freedom and flexibility in its implementation.
Implementations provided by this crate include:

* In-memory single-threaded ideal for testing.
* In-memory concurrent multithreaded ideal for benchmarking.
* Rust EVM database compatible.

# Usage

> [!NOTE]  
> The crate is currently not published to crates.io, therefore these instructions are currently invalid.

To add `evm-state-cache` you can run the `cargo add` command as following:

```
# To use the Rust EVM integration:
cargo add evm-state-cache --features revm
```

# Example

```rust
use revm::InMemoryDB;
use evm_state_cache::{CacheBuilder, CachedEvmStateRepository, EvictionPolicy, EvmStateRepository, RevmStateRepository};

// Create cache with provided options
let cache = CacheBuilder::new()
// Provide capacity based on available memory and usage
.with_capacity(10)
// Pick eviction policy most efficient for your application use-case
.with_eviction_policy(EvictionPolicy::LeastRecentlyUsed)
// Build cache instance
.build();

// Create repository with read/write access to a desired database
let repository = RevmStateRepository::new(InMemoryDB::default ());

// Combine the repository and cache into the cached repository
let mut repository = CachedEvmStateRepository::new(repository, cache);

// Create an Ethereum address
let address = [0u8; 20];

// Load account by given address
let account = repository.get( & address);

// Enjoy loading from a fast, concurrent cache in every subsequent calls for the cached address
```

# Minimum supported Rust versions

The crate's minimum supported Rust versions (MSRV) are the followings:

| Feature          |           MSRV            |
|:-----------------|:-------------------------:|
| default features | Rust 1.65.0 (Nov 3, 2022) |
| `revm`           | Rust 1.65.0 (Nov 3, 2022) |

# Library concepts

## EVM state

In the context of Ethereum, the state is an enormous data structure called
a [modified Merkle Patricia Trie](https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/),
which keeps all [accounts](https://ethereum.org/en/developers/docs/accounts/) linked by hashes and reducible to a single
root hash stored on the blockchain.

### EVM state trie

There is one global state trie, and it is updated every time a client processes a block. In it, a `path` is
always: `keccak256(ethereumAddress)` and a `value` is always: `rlp(ethereumAccount)`. More specifically an
ethereum `account` is a 4 item array of `[nonce,balance,storageRoot,codeHash]`. At this point, it's worth noting that
this `storageRoot` is the root of another patricia trie: the storage trie

## Cache

Cache holds data in-memory for fast retrieval, limited to a certain maximum number of entries. When the maximum amount
of entries is reached, a number of elements must be evicted from the cache. To determine which elements are deemed to be
evicted is based on eviction policy.

The cache’s interface has nothing to do with EVM state and should be designed to only satisfy it’s own responsibility
mentioned in the previous paragraph.
