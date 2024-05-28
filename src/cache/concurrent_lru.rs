//! [`Cache`] implementation with Least-Recently-Used eviction policy, optimized for concurrent access.
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
use crate::cache::Cache;
use concurrent_lru::sharded::LruCache;
use std::hash::Hash;

impl<K, V> Cache<K, V> for LruCache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn read<'a>(&'a mut self, key: &K) -> Option<&'a V> {
        Some(LruCache::get(self, key.clone())?.value())
    }

    fn write(&mut self, key: K, value: V) {
        LruCache::get_or_init(self, key, 1, |key| value);
    }
}
