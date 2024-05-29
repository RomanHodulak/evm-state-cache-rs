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
