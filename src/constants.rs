use crate::*;
use near_sdk::{Gas};

pub const DEFAULT_GAS_FEE: Gas = Gas(20_000_000_000_000);
pub const NEAR_DECIMAL: Balance = 1_000_000_000_000_000_000_000_000;
pub const INVESTOR_PROTECT_PERCENT: u128 = 70; // Amount of pledge locked for investor
pub const OFFER_FEES_PERCENT: u128 = 1; // Amount of fee percenr for offer
pub type ClusterId = String;
pub type ProjectId = String;
pub type OfferId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    ClusterPerOwner,
    Cluster,
    ClusterMetadata,
    ClusterPerOwnerInner { id: AccountId },
    Project,
    User,
}
