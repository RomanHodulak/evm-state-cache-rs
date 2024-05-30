use crate::cache::Cache;
use crate::evm_state::{Account, Address};

#[derive(Debug)]
pub enum EvictionPolicy {
    LeastRecentlyUsed,
    LeastFrequentlyUsed,
}

impl From<EvictionPolicy> for moka::policy::EvictionPolicy {
    fn from(value: EvictionPolicy) -> Self {
        match value {
            EvictionPolicy::LeastRecentlyUsed => Self::lru(),
            EvictionPolicy::LeastFrequentlyUsed => Self::tiny_lfu(),
        }
    }
}

#[derive(Debug)]
pub struct CacheBuilder {
    capacity: usize,
    policy: EvictionPolicy,
}

impl CacheBuilder {
    pub fn new() -> Self {
        Self {
            capacity: 10,
            policy: EvictionPolicy::LeastRecentlyUsed,
        }
    }

    pub fn with_eviction_policy(mut self, policy: EvictionPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn build(self) -> impl Cache<Address, Account> {
        moka::sync::CacheBuilder::new(self.capacity as u64)
            .eviction_policy(self.policy.into())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_builder_creates_cache_with_desired_capacity_that_evicts_lru() {
        let mut cache = CacheBuilder::new()
            .with_capacity(1)
            .with_eviction_policy(EvictionPolicy::LeastRecentlyUsed)
            .build();
        let first_address = [0u8; 20];
        let first_account = Account::new(0, U256::zero(), U256::zero(), U256::zero());
        let second_address = [1u8; 20];
        let second_account = Account::new(1, U256::zero(), U256::zero(), U256::zero());

        cache.write(first_address, first_account);
        cache.write(second_address, second_account);

        for _ in 0..100 {
            cache.read(&first_address);
            cache.read(&second_address);
        }

        assert!(
            cache.read(&second_address).is_some(),
            "Cache does not contain most recently used entry"
        );
        assert!(
            cache.read(&first_address).is_none(),
            "Cache contains evicted entry"
        );
    }

    #[test]
    fn test_builder_creates_cache_with_desired_capacity_that_evicts_lfu() {
        let mut cache = CacheBuilder::new()
            .with_capacity(1)
            .with_eviction_policy(EvictionPolicy::LeastFrequentlyUsed)
            .build();
        let first_address = [0u8; 20];
        let first_account = Account::new(0, U256::zero(), U256::zero(), U256::zero());
        let second_address = [1u8; 20];
        let second_account = Account::new(1, U256::zero(), U256::zero(), U256::zero());

        cache.write(first_address, first_account.clone());
        cache.write(first_address, first_account);
        cache.write(second_address, second_account);

        for _ in 0..100 {
            cache.read(&first_address);
            cache.read(&second_address);
        }

        assert!(
            cache.read(&second_address).is_none(),
            "Cache contains evicted entry"
        );
        assert!(
            cache.read(&first_address).is_some(),
            "Cache does not contain most frequently used entry"
        );
    }
}
