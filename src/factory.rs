//! A module that provides creation responsible interfaces.
use crate::cache::Cache;
use crate::evm_state::{Account, Address};
use std::fmt::Debug;
use std::marker::PhantomData;

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
#[derive(Debug, Default)]
pub struct CacheBuilder<State> {
    _phantom: PhantomData<State>,
    capacity: Option<usize>,
    policy: Option<EvictionPolicy>,
}

impl CacheBuilder<()> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<State: Debug + Default> CacheBuilder<State> {
    /// Sets the eviction (and admission) policy of the cache.
    pub fn with_eviction_policy(
        mut self,
        policy: EvictionPolicy,
    ) -> CacheBuilder<WithPolicy<State>> {
        self.policy.replace(policy);

        CacheBuilder::<WithPolicy<State>> {
            _phantom: PhantomData,
            capacity: self.capacity,
            policy: self.policy,
        }
    }

    /// Sets the maximum `capacity` of entries that the cache holds.
    pub fn with_capacity(mut self, capacity: usize) -> CacheBuilder<WithCapacity<State>> {
        self.capacity.replace(capacity);

        CacheBuilder::<WithCapacity<State>> {
            _phantom: PhantomData,
            capacity: self.capacity,
            policy: self.policy,
        }
    }
}

#[derive(Debug, Default)]
pub struct WithCapacity<T: Debug + Default>(PhantomData<T>);

#[derive(Debug, Default)]
pub struct WithPolicy<T: Debug + Default>(PhantomData<T>);

pub trait HasPolicy {}
pub trait HasCapacity {}

impl HasPolicy for WithPolicy<()> {}
impl HasCapacity for WithCapacity<()> {}
impl<State: Debug + Default + HasPolicy> HasCapacity for WithCapacity<State> {}
impl<State: Debug + Default + HasPolicy> HasPolicy for WithCapacity<State> {}
impl<State: Debug + Default + HasCapacity> HasPolicy for WithPolicy<State> {}
impl<State: Debug + Default + HasCapacity> HasCapacity for WithPolicy<State> {}
impl<State: Debug + Default + HasPolicy> HasPolicy for CacheBuilder<State> {}
impl<State: Debug + Default + HasCapacity> HasCapacity for CacheBuilder<State> {}

impl<State: Debug + Default + HasCapacity + HasPolicy> CacheBuilder<State> {
    /// Builds a [`Cache`] implementation according to parameters set on the builder.
    pub fn build(self) -> impl Cache<Address, Account> {
        moka::sync::CacheBuilder::new(self.capacity.expect("Parameters are filled-in") as u64)
            .eviction_policy(self.policy.expect("Parameters are filled-in").into())
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
            .with_eviction_policy(EvictionPolicy::LeastFrequentlyUsed)
            .with_capacity(1)
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
