//! A module that provides creation responsible interfaces.
use crate::cache::Cache;
use crate::evm_state::{Account, Address};

/// The eviction (and admission) policy of a cache.
///
/// When the cache is full, the eviction/ admission policy is used to determine which items
/// should be admitted to the cache and which cached items should be evicted. The choice of
/// a policy will directly affect the performance (hit rate) of the cache.
#[derive(Debug)]
pub enum EvictionPolicy {
    /// A policy that evicts entries that were used most recently.
    LeastRecentlyUsed,
    /// A policy that evicts entries that are being used the most.
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

/// Responsible for constructing [`Cache`] while providing various configuration parameters.
#[derive(Debug)]
pub struct CacheBuilder {
    capacity: usize,
    policy: EvictionPolicy,
}

impl Default for CacheBuilder {
    fn default() -> Self {
        Self {
            capacity: 10,
            policy: EvictionPolicy::LeastRecentlyUsed,
        }
    }
}

impl CacheBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the eviction (and admission) policy of the cache.
    pub fn with_eviction_policy(mut self, policy: EvictionPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Sets the maximum `capacity` of entries that the cache holds.
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    /// Builds a [`Cache`] implementation according to parameters set on the builder.
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
        let cache = CacheBuilder::new()
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
        let cache = CacheBuilder::new()
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
