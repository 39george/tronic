use std::collections::HashMap;

use time::OffsetDateTime;

use super::account::AccountType;
use crate::domain::{address::TronAddress, permission::Permission, trx::Trx};

use super::HexMessage;
use super::Message;

pub mod methods;

#[derive(Debug, Clone, PartialEq)]
pub enum ContractType {
    AccountCreateContract(AccountCreateContract),
    TransferContract(TransferContract),
    TransferAssetContract(TransferAssetContract),
    VoteAssetContract,
    VoteWitnessContract(VoteWitnessContract),
    WitnessCreateContract(WitnessCreateContract),
    AssetIssueContract(AssetIssueContract),
    WitnessUpdateContract(WitnessUpdateContract),
    ParticipateAssetIssueContract(ParticipateAssetIssueContract),
    AccountUpdateContract(AccountUpdateContract),
    FreezeBalanceContract(FreezeBalanceContract),
    UnfreezeBalanceContract(UnfreezeBalanceContract),
    WithdrawBalanceContract(WithdrawBalanceContract),
    UnfreezeAssetContract(UnfreezeAssetContract),
    UpdateAssetContract(UpdateAssetContract),
    ProposalCreateContract(ProposalCreateContract),
    ProposalApproveContract(ProposalApproveContract),
    ProposalDeleteContract(ProposalDeleteContract),
    SetAccountIdContract(SetAccountIdContract),
    CustomContract,
    CreateSmartContract(CreateSmartContract),
    TriggerSmartContract(TriggerSmartContract),
    GetContract,
    UpdateSettingContract(UpdateSettingContract),
    ExchangeCreateContract(ExchangeCreateContract),
    ExchangeInjectContract(ExchangeInjectContract),
    ExchangeWithdrawContract(ExchangeWithdrawContract),
    ExchangeTransactionContract(ExchangeTransactionContract),
    UpdateEnergyLimitContract(UpdateEnergyLimitContract),
    AccountPermissionUpdateContract(AccountPermissionUpdateContract),
    ClearAbiContract(ClearAbiContract),
    UpdateBrokerageContract(UpdateBrokerageContract),
    ShieldedTransferContract(ShieldedTransferContract),
    MarketSellAssetContract(MarketSellAssetContract),
    MarketCancelOrderContract(MarketCancelOrderContract),
    FreezeBalanceV2Contract(FreezeBalanceV2Contract),
    UnfreezeBalanceV2Contract(UnfreezeBalanceV2Contract),
    WithdrawExpireUnfreezeContract(WithdrawExpireUnfreezeContract),
    DelegateResourceContract(DelegateResourceContract),
    UnDelegateResourceContract(UnDelegateResourceContract),
    CancelAllUnfreezeV2Contract(CancelAllUnfreezeV2Contract),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub contract_type: ContractType,
    pub provider: Vec<u8>,
    pub contract_name: Message,
    pub permission_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerSmartContract {
    pub owner_address: TronAddress,
    pub contract_address: TronAddress,
    pub call_value: Trx,
    pub data: HexMessage,
    pub call_token_value: Trx,
    pub token_id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrozenSupply {
    pub frozen_amount: Trx,
    pub frozen_days: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssetIssueContract {
    pub id: String,
    pub owner_address: TronAddress,
    pub name: Message,
    pub abbr: Message,
    pub total_supply: i64,
    pub frozen_supply: Vec<FrozenSupply>,
    pub trx_num: Trx,
    pub precision: i32,
    pub num: i32,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
    /// useless
    pub order: i64,
    pub vote_score: i32,
    pub description: Message,
    pub url: Message,
    pub free_asset_net_limit: i64,
    pub public_free_asset_net_limit: i64,
    pub public_free_asset_net_usage: i64,
    pub public_latest_free_net_time: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferAssetContract {
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    pub asset_name: Message,
    pub owner_address: TronAddress,
    pub to_address: TronAddress,
    pub amount: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnfreezeAssetContract {
    pub owner_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAssetContract {
    pub owner_address: TronAddress,
    pub description: Message,
    pub url: Message,
    pub new_limit: i64,        // Not TRX
    pub new_public_limit: i64, // Not TRX
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParticipateAssetIssueContract {
    pub owner_address: TronAddress,
    pub to_address: TronAddress,
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    pub asset_name: Message,
    /// the amount of drops
    pub amount: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountCreateContract {
    pub owner_address: TronAddress,
    pub account_address: TronAddress,
    pub account_type: AccountType,
}

// Update account name. Account name is not unique now.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateContract {
    pub account_name: Message,
    pub owner_address: TronAddress,
}

// Set account id if the account has no id. Account id is unique and case insensitive.
#[derive(Debug, Clone, PartialEq)]
pub struct SetAccountIdContract {
    pub account_id: Message,
    pub owner_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountPermissionUpdateContract {
    pub owner_address: TronAddress,
    /// Empty is invalidate
    pub owner: Option<Permission>,
    /// Can be empty
    pub witness: Option<Permission>,
    /// Empty is invalidate
    pub actives: Vec<Permission>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WitnessCreateContract {
    pub owner_address: TronAddress,
    pub url: Message,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WitnessUpdateContract {
    pub owner_address: TronAddress,
    pub update_url: Message,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vote {
    pub vote_address: TronAddress,
    pub vote_count: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteWitnessContract {
    pub owner_address: TronAddress,
    pub votes: Vec<Vote>,
    pub support: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FreezeBalanceContract {
    pub owner_address: TronAddress,
    pub frozen_balance: Trx,
    pub frozen_duration: time::Duration,
    pub resource: ResourceCode,
    pub receiver_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnfreezeBalanceContract {
    pub owner_address: TronAddress,
    pub resource: ResourceCode,
    pub receiver_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawBalanceContract {
    pub owner_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferContract {
    pub owner_address: TronAddress,
    pub to_address: TronAddress,
    pub amount: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionBalanceTrace {
    pub transaction_identifier: HexMessage,
    pub operation: Vec<Operation>,
    pub r#type: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operation_identifier: i64,
    pub address: TronAddress,
    pub amount: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockBalanceTrace {
    pub block_identifier: BlockIdentifier,
    pub timestamp: OffsetDateTime,
    ///   BlockIdentifier parent_block_identifier = 4;
    pub transaction_balance_trace: Vec<TransactionBalanceTrace>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockIdentifier {
    pub hash: HexMessage,
    pub number: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccountTrace {
    pub balance: Trx,
    pub placeholder: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountIdentifier {
    pub address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountBalanceRequest {
    pub account_identifier: AccountIdentifier,
    pub block_identifier: BlockIdentifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountBalanceResponse {
    pub balance: Trx,
    pub block_identifier: BlockIdentifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FreezeBalanceV2Contract {
    pub owner_address: TronAddress,
    pub frozen_balance: Trx,
    pub resource: ResourceCode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnfreezeBalanceV2Contract {
    pub owner_address: TronAddress,
    pub unfreeze_balance: Trx,
    pub resource: ResourceCode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawExpireUnfreezeContract {
    pub owner_address: TronAddress,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ResourceCode {
    #[default]
    Bandwidth = 0,
    Energy = 1,
    #[doc = "Deprecated in FreezeV2. Use `tron_power` field instead."]
    TronPower = 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DelegateResourceContract {
    pub owner_address: TronAddress,
    pub resource: ResourceCode,
    pub balance: Trx,
    pub receiver_address: TronAddress,
    pub lock: bool,
    pub lock_period: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnDelegateResourceContract {
    pub owner_address: TronAddress,
    pub resource: ResourceCode,
    pub balance: Trx,
    pub receiver_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CancelAllUnfreezeV2Contract {
    pub owner_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProposalApproveContract {
    pub owner_address: TronAddress,
    pub proposal_id: i64,
    /// add or remove approval
    pub is_add_approval: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProposalCreateContract {
    pub owner_address: TronAddress,
    pub parameters: HashMap<i64, i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProposalDeleteContract {
    pub owner_address: TronAddress,
    pub proposal_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyStorageBytesContract {
    pub owner_address: TronAddress,
    /// storage bytes for buy
    pub bytes: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyStorageContract {
    pub owner_address: TronAddress,
    /// trx quantity for buy storage (sun)
    pub quant: Trx,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SellStorageContract {
    pub owner_address: TronAddress,
    pub storage_bytes: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateBrokerageContract {
    pub owner_address: TronAddress,
    /// 1 mean 1%
    pub brokerage: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeCreateContract {
    pub owner_address: TronAddress,
    pub first_token_id: Message,
    pub first_token_balance: i64,
    pub second_token_id: Message,
    pub second_token_balance: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeInjectContract {
    pub owner_address: TronAddress,
    pub exchange_id: i64,
    pub token_id: Message,
    pub quant: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeWithdrawContract {
    pub owner_address: TronAddress,
    pub exchange_id: i64,
    pub token_id: Message,
    pub quant: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeTransactionContract {
    pub owner_address: TronAddress,
    pub exchange_id: i64,
    pub token_id: Message,
    pub quant: i64,
    pub expected: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketSellAssetContract {
    pub owner_address: TronAddress,
    pub sell_token_id: Message,
    pub sell_token_quantity: i64,
    pub buy_token_id: Message,
    /// min to receive
    pub buy_token_quantity: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketCancelOrderContract {
    pub owner_address: TronAddress,
    pub order_id: HexMessage,
}

// TODO: continue
#[derive(Debug, Clone, PartialEq)]
pub struct SmartContract {
    pub origin_address: TronAddress,
    pub contract_address: TronAddress,
    pub abi: Abi,
    pub bytecode: Vec<u8>,
    pub call_value: Trx,
    pub consume_user_resource_percent: i64,
    pub name: String,
    pub origin_energy_limit: i64,
    pub code_hash: HexMessage,
    pub trx_hash: HexMessage,
    pub version: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abi {
    pub entrys: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub anonymous: bool,
    pub constant: bool,
    pub name: String,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
    pub entry_type: EntryType,
    pub payable: bool,
    pub state_mutability: StateMutabilityType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub indexed: bool,
    pub name: String,
    /// SolidityType type = 3;
    pub param_type: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntryType {
    UnknownEntryType = 0,
    Constructor = 1,
    Function = 2,
    Event = 3,
    Fallback = 4,
    Receive = 5,
    Error = 6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateMutabilityType {
    UnknownMutabilityType = 0,
    Pure = 1,
    View = 2,
    Nonpayable = 3,
    Payable = 4,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContractState {
    pub energy_usage: i64,
    pub energy_factor: i64,
    pub update_cycle: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSmartContract {
    pub owner_address: TronAddress,
    pub new_contract: SmartContract,
    pub call_token_value: Trx,
    pub token_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClearAbiContract {
    pub owner_address: TronAddress,
    pub contract_address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSettingContract {
    pub owner_address: TronAddress,
    pub contract_address: TronAddress,
    pub consume_user_resource_percent: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateEnergyLimitContract {
    pub owner_address: TronAddress,
    pub contract_address: TronAddress,
    pub origin_energy_limit: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpendDescription {
    pub value_commitment: Vec<u8>,
    /// merkle root
    pub anchor: Vec<u8>,
    /// used for check double spend
    pub nullifier: Vec<u8>,
    /// used for check spend authority signature
    pub rk: Vec<u8>,
    pub zkproof: Vec<u8>,
    pub spend_authority_signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReceiveDescription {
    pub value_commitment: Vec<u8>,
    pub note_commitment: Vec<u8>,
    /// for Encryption
    pub epk: Vec<u8>,
    /// Encryption for incoming, decrypt it with ivk
    pub c_enc: Vec<u8>,
    /// Encryption for audit, decrypt it with ovk
    pub c_out: Vec<u8>,
    pub zkproof: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShieldedTransferContract {
    /// transparent address
    pub transparent_from_address: TronAddress,
    pub from_amount: i64,
    pub spend_description: Vec<SpendDescription>,
    pub receive_description: Vec<ReceiveDescription>,
    pub binding_signature: Vec<u8>,
    /// transparent address
    pub transparent_to_address: TronAddress,
    /// the amount to transparent to_address
    pub to_amount: i64,
}
