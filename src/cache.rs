mod lru;

use std::borrow::Borrow;

pub trait Cache<K, V> {
    fn read<'a, Q: Borrow<K>>(&'a mut self, key: Q) -> Option<&'a V>;
    fn write(&mut self, key: K, value: V) -> Option<V>;
}
