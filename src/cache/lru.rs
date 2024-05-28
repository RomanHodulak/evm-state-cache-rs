//! [`Cache`] implementation with Least-Recently-Used eviction policy.
//!
//! # Example
//! ```
//! # use std::num::NonZeroUsize;
//! # use lru::LruCache;
//! let mut cache: LruCache<usize, &str> = LruCache::new(NonZeroUsize(20));
//!
//! cache.write(1, "phylax");
//! cache.write(2, "centurion");
//!
//! let actual = cache.read(1).expect("Key 1 was just written");
//! assert_eq!(actual, "phylax");
//! ```
use std::borrow::Borrow;
use lru::LruCache;
use crate::cache::Cache;

impl<K, V> Cache<K, V> for LruCache<K, V> {
    fn read<'a, Q: Borrow<K>>(&'a mut self, key: Q) -> Option<&'a V> {
        LruCache::get(self, key)
    }

    fn write(&mut self, key: K, value: V) -> Option<V> {
        LruCache::put(key, value)
    }
}
