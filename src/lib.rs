pub use crate::constants::*;
pub use crate::sandbox::*;
pub use crate::utils::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
// use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

mod constants;
mod sandbox;
mod utils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// #[serde(crate = "near_sdk::serde")]
pub struct Contract {
    pub owner_id: AccountId,
    pub cluster_per_owner: LookupMap<AccountId, UnorderedSet<ClusterId>>,
    pub projects: UnorderedMap<ProjectId, Project>,
    pub users: UnorderedMap<AccountId, ProjectUser>,
    pub recommendations: UnorderedMap<AccountId, Recommendation>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            cluster_per_owner: LookupMap::new(StorageKey::ClusterPerOwner),
            projects: UnorderedMap::new(StorageKey::Project),
            users: UnorderedMap::new(StorageKey::User),
            recommendations: UnorderedMap::new(StorageKey::Recommendations),
        }
    }
}
