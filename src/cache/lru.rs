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
use crate::cache::Cache;
use lru::LruCache;
use std::hash::Hash;

impl<K, V> Cache<K, V> for LruCache<K, V>
where
    K: Hash + Eq,
{
    fn contains(&self, key: &K) -> bool {
        LruCache::contains(self, key)
    }

    fn read<'a>(&'a mut self, key: &K) -> Option<&'a V> {
        LruCache::get(self, key)
    }

    fn write(&mut self, key: K, value: V) {
        LruCache::put(self, key, value);
    }
}
