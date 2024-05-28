//mod concurrent_lru;
mod lru;

pub trait Cache<K, V> {
    fn contains(&self, key: &K) -> bool;
    fn read<'a>(&'a mut self, key: &K) -> Option<&'a V>;
    fn write(&mut self, key: K, value: V);
}
