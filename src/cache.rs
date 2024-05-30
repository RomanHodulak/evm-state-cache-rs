//! A module dedicated to generic [`Cache`] trait and its implementations provided by this crate.
mod concurrent;

/// A trait for objects that implement fast key-value storage.
///
/// Implementor may choose to support certain eviction policy. The cache is defined by three methods
///
/// * The `read` method tries to load a value that is associated with given `key`. Successfully
/// loading the value from cache is referred to as a "hit" and correspondingly as "miss" to the
/// opposite case. The returned value is *cloned*
/// * The `write` method writes `value` into cache storage and associates it with given `key`.
/// In case there was not any value already associated with such `key`, then it is replaced.
/// If not, it is entered as a new key-value pair. Subsequently, if maximum capacity is reached
/// a certain different key-value pair is evicted from the cache. Which particular pair gets
/// evicted is based on a policy of the implementor.
/// * The `contains` method checks if there is a cache hit for given key and has a default
/// implementation that uses the `read` method. The implementor may choose to implement this
/// method differently if there is a more efficient way to do it or if calling the `read` method
/// messes with the eviction policy.
pub trait Cache<K, V> {
    fn contains(&self, key: &K) -> bool {
        self.read(key).is_some()
    }

    fn read(&self, key: &K) -> Option<V>;
    fn write(&self, key: K, value: V);
}
