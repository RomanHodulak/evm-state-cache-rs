//! Thread-safe [`Cache`] implementation with concurrent access and multiple eviction policies.
//!
//! # Example
//! ```
//! use moka::sync::Cache as Moka;
//! use evm_state_cache::Cache;
//! let mut cache: Moka<usize, &str> = Moka::new(20);
//!
//! cache.write(1, "phylax");
//! cache.write(2, "centurion");
//!
//! let actual = cache.read(&1).expect("Key 1 was just written");
//! assert_eq!(actual, "phylax");
//! ```
use crate::cache::Cache;
use moka::sync::Cache as Moka;
use std::hash::Hash;

impl<K, V> Cache<K, V> for Moka<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn read(&self, key: &K) -> Option<V> {
        Moka::get(self, key)
    }

    fn write(&self, key: K, value: V) {
        Moka::insert(self, key, value);
    }
}
