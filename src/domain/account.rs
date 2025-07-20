use std::collections::HashMap;

use derivative::Derivative;
use time::OffsetDateTime;

use crate::domain::{address::TronAddress, trx::Trx};

#[derive(Default, Clone, PartialEq)]
pub struct Key {
    pub address: TronAddress,
    pub weight: i64,
}

#[derive(Default, Clone, PartialEq)]
pub enum PermissionType {
    #[default]
    Owner = 0,
    Witness = 1,
    Active = 2,
}

#[derive(Default, Clone, PartialEq)]
pub struct Permission {
    pub permission_type: PermissionType,
    /// Owner id=0, Witness id=1, Active id start by 2
    pub id: i32,
    pub permission_name: String,
    pub threshold: i64,
    pub parent_id: i32,
    /// 1 bit 1 contract
    pub operations: Vec<u8>,
    pub keys: Vec<Key>,
}

#[derive(Derivative, Clone, PartialEq)]
#[derivative(Default)]
pub struct Frozen {
    /// the frozen trx balance
    pub frozen_balance: Trx,
    /// the expire time
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub expire_time: OffsetDateTime,
}

#[derive(Default, Clone, PartialEq)]
pub struct Vote {
    /// the super rep address
    pub vote_address: TronAddress,
    /// the vote num to this super rep.
    pub vote_count: i64,
}

#[derive(Default, Clone, PartialEq)]
pub struct AccountResource {
    /// energy resource, get from frozen
    pub energy_usage: i64,
    /// the frozen balance for energy
    pub frozen_balance_for_energy: Option<Frozen>,
    pub latest_consume_time_for_energy: i64,
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

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FreezeV2 {
    pub freeze_type: i32,
    pub amount: Trx,
}

#[derive(Derivative, Clone, Copy, PartialEq)]
#[derivative(Default)]
pub struct UnFreezeV2 {
    pub unfreeze_type: i32,
    pub unfreeze_amount: Trx,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    pub unfreeze_expire_time: OffsetDateTime,
}

#[derive(Default, Clone, PartialEq)]
pub struct Account {
    /// account nick name
    pub account_name: ::prost::alloc::vec::Vec<u8>,
    pub r#type: i32,
    /// the create address
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// the trx balance
    pub balance: i64,
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
    pub acquired_delegated_frozen_balance_for_bandwidth: i64,
    /// Freeze and provide balances to other accounts
    pub delegated_frozen_balance_for_bandwidth: i64,
    pub old_tron_power: i64,
    pub tron_power: Option<Frozen>,
    pub asset_optimized: bool,
    /// this account create time
    pub create_time: i64,
    /// this last operation time, including transfer, voting and so on. //FIXME fix grammar
    pub latest_opration_time: i64,
    /// witness block producing allowance
    pub allowance: i64,
    /// last withdraw time
    pub latest_withdraw_time: i64,
    /// not used so far
    pub code: ::prost::alloc::vec::Vec<u8>,
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
    pub latest_consume_time: i64,
    pub latest_consume_free_time: i64,
    /// the identity of this account, case insensitive
    pub account_id: Vec<u8>,
    pub net_window_size: i64,
    pub net_window_optimized: bool,
    pub account_resource: Option<AccountResource>,
    pub code_hash: Vec<u8>,
    pub owner_permission: Option<Permission>,
    pub witness_permission: Option<Permission>,
    pub active_permission: Vec<Permission>,
    pub frozen_v2: Vec<FreezeV2>,
    pub unfrozen_v2: Vec<UnFreezeV2>,
    pub delegated_frozen_v2_balance_for_bandwidth: i64,
    pub acquired_delegated_frozen_v2_balance_for_bandwidth: i64,
}
