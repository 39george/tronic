use std::collections::HashMap;

use derivative::Derivative;
use time::OffsetDateTime;

use crate::domain::contract::ResourceCode;
use crate::domain::{address::TronAddress, trx::Trx};

use super::Message;
use super::permission::Permission;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AccountType {
    #[default]
    Normal = 0,
    AssetIssue = 1,
    Contract = 2,
}

#[derive(Debug, Derivative, Clone, PartialEq)]
#[derivative(Default)]
pub struct Frozen {
    /// the frozen trx balance
    pub frozen_balance: Trx,
    /// the expire time
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub expire_time: OffsetDateTime,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vote {
    /// the super rep address
    pub vote_address: TronAddress,
    /// the vote num to this super rep.
    pub vote_count: i64,
}

#[derive(Debug, Derivative, Clone, PartialEq)]
#[derivative(Default)]
pub struct AccountResource {
    /// energy resource, get from frozen
    pub energy_usage: i64,
    /// the frozen balance for energy
    pub frozen_balance_for_energy: Option<Frozen>,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub latest_consume_time_for_energy: OffsetDateTime,
    /// Frozen balance provided by other accounts to this account
    pub acquired_delegated_frozen_balance_for_energy: i64,
    /// Frozen balances provided to other accounts
    pub delegated_frozen_balance_for_energy: i64,
    /// storage resource, get from market
    pub storage_limit: i64,
    pub storage_usage: i64,
    pub latest_exchange_storage_time: i64,
    pub energy_window_size: i64,
    pub delegated_frozen_v2_balance_for_energy: i64,
    pub acquired_delegated_frozen_v2_balance_for_energy: i64,
    pub energy_window_optimized: bool,
}

pub struct AccountResourceUsage {
    /// Used free bandwidth
    pub free_net_used: i64,
    /// Free bandwidth limit (daily)
    pub free_net_limit: i64,
    /// Used total bandwidth
    pub net_used: i64,
    /// Total bandwidth limit
    pub net_limit: i64,
    /// Bandwidth used per token (key: token ID)
    pub asset_net_used: HashMap<String, i64>,
    /// Bandwidth limit per token
    pub asset_net_limit: HashMap<String, i64>,
    /// Network-wide bandwidth pool
    pub total_net_limit: i64,
    /// Staking-based bandwidth allocation ratio
    pub total_net_weight: i64,
    pub total_tron_power_weight: i64,
    pub tron_power_used: i64,
    pub tron_power_limit: i64,
    /// Used for smart contract execution
    pub energy_used: i64,
    /// Max allocatable energy
    pub energy_limit: i64,
    /// Network-wide energy pool
    pub total_energy_limit: i64,
    /// Staking-based energy allocation ratio
    pub total_energy_weight: i64,
    #[doc = "Deprecated"]
    pub storage_used: i64,
    #[doc = "Deprecated"]
    pub storage_limit: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FreezeV2 {
    pub freeze_type: ResourceCode,
    pub amount: Trx,
}

#[derive(Debug, Derivative, Clone, Copy, PartialEq)]
#[derivative(Default)]
pub struct UnFreezeV2 {
    pub unfreeze_type: i32,
    pub unfreeze_amount: Trx,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub unfreeze_expire_time: OffsetDateTime,
}

#[derive(Debug, Derivative, Clone, PartialEq)]
#[derivative(Default)]
pub struct Account {
    /// account nick name
    pub account_name: Message,
    pub account_type: AccountType,
    /// the create address
    pub address: TronAddress,
    /// the trx balance
    pub balance: Trx,
    /// the votes
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// the other asset owned by this account
    pub asset: HashMap<String, i64>,
    /// the other asset owned by this account，key is assetId
    pub asset_v2: HashMap<String, i64>,
    /// the frozen balance for bandwidth
    pub frozen: Vec<Frozen>,
    /// bandwidth, get from frozen
    pub net_usage: i64,
    /// Frozen balance provided by other accounts to this account
    pub acquired_delegated_frozen_balance_for_bandwidth: Trx,
    /// Freeze and provide balances to other accounts
    pub delegated_frozen_balance_for_bandwidth: Trx,
    pub old_tron_power: i64,
    pub tron_power: Option<Frozen>,
    pub asset_optimized: bool,
    /// this account create time
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub create_time: OffsetDateTime,
    /// this last operation time, including transfer, voting and so on. //FIXME fix grammar
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub latest_opration_time: OffsetDateTime,
    /// witness block producing allowance
    pub allowance: i64,
    /// last withdraw time
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub latest_withdraw_time: OffsetDateTime,
    /// not used so far
    pub code: Vec<u8>,
    pub is_witness: bool,
    pub is_committee: bool,
    /// frozen asset(for asset issuer)
    pub frozen_supply: Vec<Frozen>,
    /// asset_issued_name
    pub asset_issued_name: Vec<u8>,
    pub asset_issued_id: Vec<u8>,
    pub latest_asset_operation_time: HashMap<String, i64>,
    pub latest_asset_operation_time_v2: HashMap<String, i64>,
    pub free_net_usage: i64,
    pub free_asset_net_usage: HashMap<String, i64>,
    pub free_asset_net_usage_v2: HashMap<String, i64>,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub latest_consume_time: OffsetDateTime,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub latest_consume_free_time: OffsetDateTime,
    /// the identity of this account, case insensitive
    pub account_id: Vec<u8>,
    pub net_window_size: i64,
    pub net_window_optimized: bool,
    pub account_resource: AccountResource,
    pub code_hash: Vec<u8>,
    pub owner_permission: Permission,
    pub witness_permission: Option<Permission>,
    pub active_permission: Vec<Permission>,
    pub frozen_v2: Vec<FreezeV2>,
    pub unfrozen_v2: Vec<UnFreezeV2>,
    pub delegated_frozen_v2_balance_for_bandwidth: Trx,
    pub acquired_delegated_frozen_v2_balance_for_bandwidth: Trx,
}
