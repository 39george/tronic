use k256::ecdsa::{RecoveryId, Signature};
pub use protocol::*;
use time::OffsetDateTime;

use crate::{
    domain::{
        self, Hash32, RecoverableSignature, RefBlockBytes, RefBlockHash,
        address::TronAddress, permission::Ops,
    },
    impl_enum_conversions,
    utility::TronOffsetDateTime,
};

// #[path = "google.api.rs"]
// mod google_api;
mod protocol;

pub mod contracts_conversions;

// pub const TRON_PROTOCOL_FILE_DESCRIPTOR_SET: &[u8] =
//     tonic::include_file_descriptor_set!("tron_protocol_descriptor");

// ────────────────────────────── Transaction ─────────────────────────────── //

#[derive(Debug, thiserror::Error)]
pub enum ProtoConvError {
    #[error("missing field {0}")]
    Missing(&'static str),
    #[error("invalid ref_block_bytes length: got {got}, expected 2")]
    BadRefBlockBytes { got: usize },
    #[error("invalid ref_block_hash length: got {got}, expected 8")]
    BadRefBlockHash { got: usize },
    #[error("bad timestamp: {0}")]
    IncorrectTimestamp(#[from] time::error::ComponentRange),
}

impl From<Hash32> for BytesMessage {
    fn from(value: Hash32) -> Self {
        BytesMessage {
            value: value.into(),
        }
    }
}

impl_enum_conversions! {
  transaction::result::ContractResult => domain::transaction::ContractResult {
        Default,
        Success,
        Revert,
        BadJumpDestination,
        OutOfMemory,
        PrecompiledContract,
        StackTooSmall,
        StackTooLarge,
        IllegalOperation,
        StackOverflow,
        OutOfEnergy,
        OutOfTime,
        JvmStackOverFlow,
        Unknown,
        TransferFailed,
        InvalidCode,
    }
}

impl From<AccountId> for domain::transaction::AccountId {
    fn from(value: AccountId) -> Self {
        domain::transaction::AccountId {
            name: value.name.into(),
            address: value.address.as_slice().try_into().unwrap_or_default(),
        }
    }
}

impl From<domain::transaction::AccountId> for AccountId {
    fn from(value: domain::transaction::AccountId) -> Self {
        AccountId {
            name: value.name.into(),
            address: value.address.as_bytes().to_vec(),
        }
    }
}

impl From<Authority> for domain::transaction::Authority {
    fn from(value: Authority) -> Self {
        domain::transaction::Authority {
            account: value.account.unwrap_or_default().into(),
            permission_name: value.permission_name.into(),
        }
    }
}

impl From<domain::transaction::Authority> for Authority {
    fn from(value: domain::transaction::Authority) -> Self {
        Authority {
            account: Some(value.account.into()),
            permission_name: value.permission_name.into(),
        }
    }
}

impl From<MarketOrderDetail> for domain::transaction::MarketOrderDetail {
    fn from(value: MarketOrderDetail) -> Self {
        domain::transaction::MarketOrderDetail {
            maker_order_id: value.maker_order_id.try_into().unwrap_or_default(),
            taker_order_id: value.taker_order_id.try_into().unwrap_or_default(),
            fill_sell_quantity: value.fill_sell_quantity,
            fill_buy_quantity: value.fill_buy_quantity,
        }
    }
}

impl From<domain::transaction::MarketOrderDetail> for MarketOrderDetail {
    fn from(value: domain::transaction::MarketOrderDetail) -> Self {
        MarketOrderDetail {
            maker_order_id: value.maker_order_id.into(),
            taker_order_id: value.taker_order_id.into(),
            fill_sell_quantity: value.fill_sell_quantity,
            fill_buy_quantity: value.fill_buy_quantity,
        }
    }
}

impl From<transaction::Result> for domain::transaction::TransactionResult {
    fn from(r: transaction::Result) -> Self {
        domain::transaction::TransactionResult {
            fee: r.fee,
            ret: r.ret().into(),
            contract_ret: r.contract_ret().into(),
            asset_issue_id: r.asset_issue_id,
            withdraw_amount: r.withdraw_amount,
            unfreeze_amount: r.unfreeze_amount,
            exchange_received_amount: r.exchange_received_amount,
            exchange_inject_another_amount: r.exchange_inject_another_amount,
            exchange_withdraw_another_amount: r
                .exchange_withdraw_another_amount,
            exchange_id: r.exchange_id,
            shielded_transaction_fee: r.shielded_transaction_fee,
            order_id: r.order_id,
            order_details: r
                .order_details
                .into_iter()
                .map(Into::into)
                .collect(),
            withdraw_expire_amount: r.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: r.cancel_unfreeze_v2_amount,
        }
    }
}

impl From<domain::transaction::TransactionResult> for transaction::Result {
    fn from(r: domain::transaction::TransactionResult) -> Self {
        transaction::Result {
            fee: r.fee,
            ret: transaction::result::Code::from(r.ret).into(),
            contract_ret: transaction::result::ContractResult::from(
                r.contract_ret,
            )
            .into(),
            asset_issue_id: r.asset_issue_id,
            withdraw_amount: r.withdraw_amount,
            unfreeze_amount: r.unfreeze_amount,
            exchange_received_amount: r.exchange_received_amount,
            exchange_inject_another_amount: r.exchange_inject_another_amount,
            exchange_withdraw_another_amount: r
                .exchange_withdraw_another_amount,
            exchange_id: r.exchange_id,
            shielded_transaction_fee: r.shielded_transaction_fee,
            order_id: r.order_id,
            order_details: r
                .order_details
                .into_iter()
                .map(Into::into)
                .collect(),
            withdraw_expire_amount: r.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: r.cancel_unfreeze_v2_amount,
        }
    }
}

impl TryFrom<transaction::Raw> for domain::transaction::RawTransaction {
    type Error = ProtoConvError;
    fn try_from(mut r: transaction::Raw) -> Result<Self, Self::Error> {
        let ref_block_bytes: RefBlockBytes =
            r.ref_block_bytes.as_slice().try_into().map_err(|_| {
                ProtoConvError::BadRefBlockBytes {
                    got: r.ref_block_bytes.len(),
                }
            })?;
        let ref_block_hash: RefBlockHash =
            r.ref_block_hash.as_slice().try_into().map_err(|_| {
                ProtoConvError::BadRefBlockHash {
                    got: r.ref_block_hash.len(),
                }
            })?;

        Ok(domain::transaction::RawTransaction {
            ref_block_bytes,
            ref_block_num: r.ref_block_num,
            ref_block_hash,
            expiration: OffsetDateTime::try_from_tron(r.expiration)?,
            data: r.data.into(),
            contract: r.contract.into_iter().map(Into::into).collect(),
            scripts: r.scripts,
            timestamp: OffsetDateTime::try_from_tron(r.timestamp)?,
            fee_limit: r.fee_limit.into(),
            auths: r.auths.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<domain::transaction::RawTransaction> for transaction::Raw {
    fn from(r: domain::transaction::RawTransaction) -> Self {
        transaction::Raw {
            ref_block_bytes: r.ref_block_bytes.into(),
            ref_block_num: r.ref_block_num,
            ref_block_hash: r.ref_block_hash.try_into().unwrap_or_default(),
            expiration: r.expiration.to_tron(),
            data: r.data.into(),
            contract: r.contract.into_iter().map(Into::into).collect(),
            scripts: r.scripts,
            timestamp: r.timestamp.to_tron(),
            fee_limit: r.fee_limit.into(),
            auths: r.auths.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<Transaction> for domain::transaction::Transaction {
    type Error = ProtoConvError;

    fn try_from(t: Transaction) -> Result<Self, Self::Error> {
        let raw = t.raw_data.ok_or(ProtoConvError::Missing("raw_data"))?;
        Ok(Self {
            raw: raw.try_into()?, // ← пропагируем проверку длины
            signature: t
                .signature
                .into_iter()
                .map(|s| {
                    TryInto::<RecoverableSignature>::try_into(s.as_slice())
                })
                .collect::<Result<_, _>>()
                .map_err(|_| ProtoConvError::Missing("signature"))?,
            result: t.ret.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<domain::transaction::Transaction> for Transaction {
    fn from(t: domain::transaction::Transaction) -> Self {
        Transaction {
            raw_data: Some(t.raw.into()),
            signature: t.signature.into_iter().map(Into::into).collect(),
            ret: t.result.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<TransactionExtention>
    for domain::transaction::TransactionExtention
{
    type Error = ProtoConvError;

    fn try_from(txext: TransactionExtention) -> Result<Self, Self::Error> {
        Ok(domain::transaction::TransactionExtention {
            transaction: txext
                .transaction
                .map(TryInto::try_into)
                .transpose()?,
            txid: txext.txid.try_into().unwrap_or_default(),
            constant_result: txext.constant_result,
            energy_used: txext.energy_used,
            energy_penalty: txext.energy_penalty,
        })
    }
}

impl From<domain::transaction::TransactionExtention> for TransactionExtention {
    fn from(txext: domain::transaction::TransactionExtention) -> Self {
        TransactionExtention {
            transaction: txext.transaction.map(Into::into),
            txid: txext.txid.into(),
            constant_result: txext.constant_result,
            energy_used: txext.energy_used,
            energy_penalty: txext.energy_penalty,
            result: todo!(),
            logs: todo!(),
            internal_transactions: todo!(),
        }
    }
}

impl From<domain::transaction::Log> for transaction_info::Log {
    fn from(value: domain::transaction::Log) -> Self {
        transaction_info::Log {
            address: value.address,
            topics: value.topics,
            data: value.data,
        }
    }
}

impl From<transaction_info::Log> for domain::transaction::Log {
    fn from(value: transaction_info::Log) -> Self {
        domain::transaction::Log {
            address: value.address,
            topics: value.topics,
            data: value.data,
        }
    }
}

impl From<domain::transaction::ResourceReceipt> for ResourceReceipt {
    fn from(value: domain::transaction::ResourceReceipt) -> Self {
        ResourceReceipt {
            energy_usage: value.energy_usage,
            energy_fee: value.energy_fee,
            origin_energy_usage: value.origin_energy_usage,
            energy_usage_total: value.energy_usage_total,
            net_usage: value.net_usage,
            net_fee: value.net_fee.into(),
            result: transaction::result::ContractResult::from(value.result)
                .into(),
            energy_penalty_total: value.energy_penalty_total,
        }
    }
}

impl From<ResourceReceipt> for domain::transaction::ResourceReceipt {
    fn from(value: ResourceReceipt) -> Self {
        domain::transaction::ResourceReceipt {
            energy_usage: value.energy_usage,
            energy_fee: value.energy_fee,
            origin_energy_usage: value.origin_energy_usage,
            energy_usage_total: value.energy_usage_total,
            net_usage: value.net_usage,
            net_fee: value.net_fee.into(),
            result: value.result().into(),
            energy_penalty_total: value.energy_penalty_total,
        }
    }
}

impl From<domain::transaction::CallValueInfo>
    for internal_transaction::CallValueInfo
{
    fn from(value: domain::transaction::CallValueInfo) -> Self {
        internal_transaction::CallValueInfo {
            call_value: value.call_value,
            token_id: value.token_id,
        }
    }
}

impl From<internal_transaction::CallValueInfo>
    for domain::transaction::CallValueInfo
{
    fn from(value: internal_transaction::CallValueInfo) -> Self {
        domain::transaction::CallValueInfo {
            call_value: value.call_value,
            token_id: value.token_id,
        }
    }
}

impl From<domain::transaction::InternalTransaction> for InternalTransaction {
    fn from(value: domain::transaction::InternalTransaction) -> Self {
        InternalTransaction {
            hash: value.hash.into(),
            caller_address: value
                .caller_address
                .as_bytes()
                .try_into()
                .unwrap_or_default(),
            transfer_to_address: value
                .transfer_to_address
                .as_bytes()
                .try_into()
                .unwrap_or_default(),
            call_value_info: value
                .call_value_info
                .into_iter()
                .map(Into::into)
                .collect(),
            note: value.note.into(),
            rejected: value.rejected,
            extra: value.extra,
        }
    }
}

impl From<InternalTransaction> for domain::transaction::InternalTransaction {
    fn from(value: InternalTransaction) -> Self {
        domain::transaction::InternalTransaction {
            hash: value.hash.try_into().unwrap_or_default(),
            caller_address: value
                .caller_address
                .as_slice()
                .try_into()
                .unwrap_or_default(),
            transfer_to_address: value
                .transfer_to_address
                .as_slice()
                .try_into()
                .unwrap_or_default(),
            call_value_info: value
                .call_value_info
                .into_iter()
                .map(Into::into)
                .collect(),
            note: value.note.into(),
            rejected: value.rejected,
            extra: value.extra,
        }
    }
}

impl_enum_conversions! {
    transaction_info::Code => domain::transaction::TxCode {
        Sucess,
        Failed
    }
}

impl_enum_conversions! {
    transaction::result::Code => domain::transaction::TxCode {
        Sucess,
        Failed
    }
}

impl From<domain::transaction::TransactionInfo> for TransactionInfo {
    fn from(value: domain::transaction::TransactionInfo) -> Self {
        TransactionInfo {
            id: value.id.try_into().unwrap_or_default(),
            fee: value.fee.into(),
            block_number: value.block_number,
            block_time_stamp: value.block_time_stamp.to_tron(),
            contract_result: value
                .contract_result
                .into_iter()
                .map(Into::into)
                .collect(),
            // TODO: abc
            contract_address: value.contract_address.as_bytes().into(),
            receipt: value.receipt.map(Into::into),
            log: value.log.into_iter().map(Into::into).collect(),
            result: transaction_info::Code::from(value.result).into(),
            res_message: value.res_message.into(),
            asset_issue_id: value.asset_issue_id,
            withdraw_amount: value.withdraw_amount.into(),
            unfreeze_amount: value.unfreeze_amount.into(),
            internal_transactions: value
                .internal_transactions
                .into_iter()
                .map(Into::into)
                .collect(),
            exchange_received_amount: value.exchange_received_amount,
            exchange_inject_another_amount: value
                .exchange_inject_another_amount,
            exchange_withdraw_another_amount: value
                .exchange_withdraw_another_amount,
            exchange_id: value.exchange_id,
            shielded_transaction_fee: value.shielded_transaction_fee.into(),
            order_id: value.order_id.into(),
            order_details: value
                .order_details
                .into_iter()
                .map(Into::into)
                .collect(),
            packing_fee: value.packing_fee.into(),
            withdraw_expire_amount: value.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: value
                .cancel_unfreeze_v2_amount
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl TryFrom<TransactionInfo> for domain::transaction::TransactionInfo {
    type Error = ProtoConvError;
    fn try_from(value: TransactionInfo) -> Result<Self, Self::Error> {
        Ok(domain::transaction::TransactionInfo {
            result: value.result().into(),
            id: value.id.try_into().unwrap_or_default(),
            fee: value.fee.into(),
            block_number: value.block_number,
            block_time_stamp: OffsetDateTime::try_from_tron(
                value.block_time_stamp,
            )?,
            contract_result: value
                .contract_result
                .into_iter()
                .map(Into::into)
                .collect(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap_or_default(),
            receipt: value.receipt.map(Into::into),
            log: value.log.into_iter().map(Into::into).collect(),
            res_message: value.res_message.into(),
            asset_issue_id: value.asset_issue_id,
            withdraw_amount: value.withdraw_amount.into(),
            unfreeze_amount: value.unfreeze_amount.into(),
            internal_transactions: value
                .internal_transactions
                .into_iter()
                .map(Into::into)
                .collect(),
            exchange_received_amount: value.exchange_received_amount,
            exchange_inject_another_amount: value
                .exchange_inject_another_amount,
            exchange_withdraw_another_amount: value
                .exchange_withdraw_another_amount,
            exchange_id: value.exchange_id,
            shielded_transaction_fee: value.shielded_transaction_fee.into(),
            order_id: value.order_id.into(),
            order_details: value
                .order_details
                .into_iter()
                .map(Into::into)
                .collect(),
            packing_fee: value.packing_fee.into(),
            withdraw_expire_amount: value.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: value
                .cancel_unfreeze_v2_amount
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        })
    }
}

// ──────────────────────────────── Account ───────────────────────────────── //

impl From<Key> for crate::domain::permission::Key {
    fn from(k: Key) -> Self {
        Self {
            address: TronAddress::try_from(&k.address)
                .expect("invalid address"),
            weight: k.weight,
        }
    }
}

impl From<crate::domain::permission::Key> for Key {
    fn from(k: crate::domain::permission::Key) -> Self {
        Self {
            address: k.address.as_bytes().to_vec(),
            weight: k.weight,
        }
    }
}

impl_enum_conversions! {
    AccountType => domain::account::AccountType {
        Normal,
        AssetIssue,
        Contract
    }
}

impl_enum_conversions! {
    permission::PermissionType => domain::permission::PermissionType {
        Owner,
        Witness,
        Active
    }
}

impl From<Permission> for crate::domain::permission::Permission {
    fn from(p: Permission) -> Self {
        Self {
            permission_type: p.r#type().into(),
            id: p.id,
            permission_name: p.permission_name.try_into().unwrap_or_default(),
            threshold: p.threshold,
            parent_id: p.parent_id,
            operations: Ops::decode_ops(&p.operations),
            keys: p.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<crate::domain::permission::Permission> for Permission {
    fn from(p: crate::domain::permission::Permission) -> Self {
        Self {
            r#type: permission::PermissionType::from(p.permission_type).into(),
            id: p.id,
            permission_name: p.permission_name.into(),
            threshold: p.threshold,
            parent_id: p.parent_id,
            operations: Ops::encode_ops(&p.operations),
            keys: p.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<account::Frozen> for crate::domain::account::Frozen {
    type Error = ProtoConvError;
    fn try_from(f: account::Frozen) -> Result<Self, Self::Error> {
        Ok(Self {
            frozen_balance: domain::trx::Trx::from(f.frozen_balance),
            expire_time: OffsetDateTime::try_from_tron(f.expire_time)?,
        })
    }
}

impl From<crate::domain::account::Frozen> for account::Frozen {
    fn from(f: crate::domain::account::Frozen) -> Self {
        Self {
            frozen_balance: f.frozen_balance.to_sun(),
            expire_time: f.expire_time.to_tron(),
        }
    }
}

impl From<account::FreezeV2> for crate::domain::account::FreezeV2 {
    fn from(f: account::FreezeV2) -> Self {
        Self {
            freeze_type: f.r#type().into(),
            amount: f.amount.into(),
        }
    }
}

impl From<crate::domain::account::FreezeV2> for account::FreezeV2 {
    fn from(f: crate::domain::account::FreezeV2) -> Self {
        Self {
            r#type: ResourceCode::from(f.freeze_type).into(),
            amount: f.amount.to_sun(),
        }
    }
}

impl TryFrom<account::UnFreezeV2> for crate::domain::account::UnFreezeV2 {
    type Error = ProtoConvError;
    fn try_from(f: account::UnFreezeV2) -> Result<Self, Self::Error> {
        Ok(Self {
            unfreeze_type: f.r#type().into(),
            unfreeze_amount: f.unfreeze_amount.into(),
            unfreeze_expire_time: OffsetDateTime::try_from_tron(
                f.unfreeze_expire_time,
            )?,
        })
    }
}

impl From<crate::domain::account::UnFreezeV2> for account::UnFreezeV2 {
    fn from(f: crate::domain::account::UnFreezeV2) -> Self {
        Self {
            r#type: ResourceCode::from(f.unfreeze_type).into(),
            unfreeze_amount: f.unfreeze_amount.to_sun(),
            unfreeze_expire_time: f.unfreeze_expire_time.to_tron(),
        }
    }
}

impl From<Vote> for crate::domain::account::Vote {
    fn from(v: Vote) -> Self {
        Self {
            vote_address: TronAddress::try_from(&v.vote_address)
                .expect("invalid vote address"),
            vote_count: v.vote_count,
        }
    }
}

impl From<crate::domain::account::Vote> for Vote {
    fn from(v: crate::domain::account::Vote) -> Self {
        Self {
            vote_address: v.vote_address.as_bytes().to_vec(),
            vote_count: v.vote_count,
        }
    }
}

impl TryFrom<account::AccountResource> for domain::account::AccountResource {
    type Error = ProtoConvError;
    fn try_from(r: account::AccountResource) -> Result<Self, Self::Error> {
        Ok(Self {
            energy_usage: r.energy_usage,
            frozen_balance_for_energy: r
                .frozen_balance_for_energy
                .map(TryInto::try_into)
                .transpose()?,
            latest_consume_time_for_energy: OffsetDateTime::try_from_tron(
                r.latest_consume_time_for_energy,
            )?,
            acquired_delegated_frozen_balance_for_energy: r
                .acquired_delegated_frozen_balance_for_energy
                .into(),
            delegated_frozen_balance_for_energy: r
                .delegated_frozen_balance_for_energy,
            storage_limit: r.storage_limit,
            storage_usage: r.storage_usage,
            latest_exchange_storage_time: r.latest_exchange_storage_time,
            energy_window_size: r.energy_window_size,
            delegated_frozen_v2_balance_for_energy: r
                .delegated_frozen_v2_balance_for_energy
                .into(),
            acquired_delegated_frozen_v2_balance_for_energy: r
                .acquired_delegated_frozen_v2_balance_for_energy
                .into(),
            energy_window_optimized: r.energy_window_optimized,
        })
    }
}

impl From<domain::account::AccountResource> for account::AccountResource {
    fn from(r: domain::account::AccountResource) -> Self {
        Self {
            energy_usage: r.energy_usage,
            frozen_balance_for_energy: r
                .frozen_balance_for_energy
                .map(Into::into),
            latest_consume_time_for_energy: r
                .latest_consume_time_for_energy
                .to_tron(),
            acquired_delegated_frozen_balance_for_energy: r
                .acquired_delegated_frozen_balance_for_energy
                .into(),
            delegated_frozen_balance_for_energy: r
                .delegated_frozen_balance_for_energy,
            storage_limit: r.storage_limit,
            storage_usage: r.storage_usage,
            latest_exchange_storage_time: r.latest_exchange_storage_time,
            energy_window_size: r.energy_window_size,
            delegated_frozen_v2_balance_for_energy: r
                .delegated_frozen_v2_balance_for_energy
                .into(),
            acquired_delegated_frozen_v2_balance_for_energy: r
                .acquired_delegated_frozen_v2_balance_for_energy
                .into(),
            energy_window_optimized: r.energy_window_optimized,
        }
    }
}

impl From<AccountResourceMessage> for domain::account::AccountResourceUsage {
    fn from(r: AccountResourceMessage) -> Self {
        domain::account::AccountResourceUsage {
            free_net_used: r.free_net_used,
            free_net_limit: r.free_net_limit,
            net_used: r.net_used,
            net_limit: r.net_limit,
            asset_net_used: r.asset_net_used,
            asset_net_limit: r.asset_net_limit,
            total_net_limit: r.total_net_limit,
            total_net_weight: r.total_net_weight.into(),
            total_tron_power_weight: r.total_tron_power_weight,
            tron_power_used: r.tron_power_used.into(),
            tron_power_limit: r.tron_power_limit.into(),
            energy_used: r.energy_used,
            energy_limit: r.energy_limit,
            total_energy_limit: r.total_energy_limit,
            total_energy_weight: r.total_energy_weight.into(),
            storage_used: r.storage_used,
            storage_limit: r.storage_limit,
        }
    }
}

impl TryFrom<Account> for domain::account::Account {
    type Error = ProtoConvError;
    fn try_from(a: Account) -> Result<Self, Self::Error> {
        Ok(Self {
            account_type: a.r#type().into(),
            account_name: a.account_name.into(),
            address: a.address.as_slice().try_into().unwrap_or_default(),
            balance: a.balance.into(),
            votes: a.votes.into_iter().map(Into::into).collect(),
            asset: a.asset,
            asset_v2: a.asset_v2,
            frozen: a
                .frozen
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            net_usage: a.net_usage,
            acquired_delegated_frozen_balance_for_bandwidth: a
                .acquired_delegated_frozen_balance_for_bandwidth
                .into(),
            delegated_frozen_balance_for_bandwidth: a
                .delegated_frozen_balance_for_bandwidth
                .into(),
            old_tron_power: a.old_tron_power,
            tron_power: a.tron_power.map(TryInto::try_into).transpose()?,
            asset_optimized: a.asset_optimized,
            create_time: OffsetDateTime::try_from_tron(a.create_time)?,
            latest_opration_time: OffsetDateTime::try_from_tron(
                a.latest_opration_time,
            )?,
            allowance: a.allowance,
            latest_withdraw_time: OffsetDateTime::try_from_tron(
                a.latest_withdraw_time,
            )?,
            code: a.code,
            is_witness: a.is_witness,
            is_committee: a.is_committee,
            frozen_supply: a
                .frozen_supply
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            asset_issued_name: a.asset_issued_name,
            asset_issued_id: a.asset_issued_id,
            latest_asset_operation_time: a.latest_asset_operation_time,
            latest_asset_operation_time_v2: a.latest_asset_operation_time_v2,
            free_net_usage: a.free_net_usage,
            free_asset_net_usage: a.free_asset_net_usage,
            free_asset_net_usage_v2: a.free_asset_net_usage_v2,
            latest_consume_time: OffsetDateTime::try_from_tron(
                a.latest_consume_time,
            )?,
            latest_consume_free_time: OffsetDateTime::try_from_tron(
                a.latest_consume_free_time,
            )?,
            account_id: a.account_id,
            net_window_size: a.net_window_size,
            net_window_optimized: a.net_window_optimized,
            account_resource: a
                .account_resource
                .map(TryInto::try_into)
                .transpose()?
                .unwrap_or_default(),
            code_hash: a.code_hash,
            owner_permission: a
                .owner_permission
                .map(Into::into)
                .unwrap_or_default(),
            witness_permission: a.witness_permission.map(Into::into),
            active_permission: a
                .active_permission
                .into_iter()
                .map(Into::into)
                .collect(),
            frozen_v2: a.frozen_v2.into_iter().map(Into::into).collect(),
            unfrozen_v2: a
                .unfrozen_v2
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            delegated_frozen_v2_balance_for_bandwidth: a
                .delegated_frozen_v2_balance_for_bandwidth
                .into(),
            acquired_delegated_frozen_v2_balance_for_bandwidth: a
                .acquired_delegated_frozen_v2_balance_for_bandwidth
                .into(),
        })
    }
}

impl From<domain::account::Account> for Account {
    fn from(a: domain::account::Account) -> Self {
        Self {
            account_name: a.account_name.as_bytes().to_vec(),
            r#type: AccountType::from(a.account_type).into(),
            address: a.address.as_bytes().to_vec(),
            balance: a.balance.into(),
            votes: a.votes.into_iter().map(Into::into).collect(),
            asset: a.asset,
            asset_v2: a.asset_v2,
            frozen: a.frozen.into_iter().map(Into::into).collect(),
            net_usage: a.net_usage,
            acquired_delegated_frozen_balance_for_bandwidth: a
                .acquired_delegated_frozen_balance_for_bandwidth
                .into(),
            delegated_frozen_balance_for_bandwidth: a
                .delegated_frozen_balance_for_bandwidth
                .into(),
            old_tron_power: a.old_tron_power,
            tron_power: a.tron_power.map(Into::into),
            asset_optimized: a.asset_optimized,
            create_time: a.create_time.to_tron(),
            latest_opration_time: a.latest_opration_time.to_tron(),
            allowance: a.allowance,
            latest_withdraw_time: a.latest_withdraw_time.to_tron(),
            code: a.code,
            is_witness: a.is_witness,
            is_committee: a.is_committee,
            frozen_supply: a
                .frozen_supply
                .into_iter()
                .map(Into::into)
                .collect(),
            asset_issued_name: a.asset_issued_name,
            asset_issued_id: a.asset_issued_id,
            latest_asset_operation_time: a.latest_asset_operation_time,
            latest_asset_operation_time_v2: a.latest_asset_operation_time_v2,
            free_net_usage: a.free_net_usage,
            free_asset_net_usage: a.free_asset_net_usage,
            free_asset_net_usage_v2: a.free_asset_net_usage_v2,
            latest_consume_time: a.latest_consume_time.to_tron(),
            latest_consume_free_time: a.latest_consume_free_time.to_tron(),
            account_id: a.account_id,
            net_window_size: a.net_window_size,
            net_window_optimized: a.net_window_optimized,
            account_resource: Some(a.account_resource.into()),
            code_hash: a.code_hash,
            owner_permission: Some(a.owner_permission.into()),
            witness_permission: a.witness_permission.map(Into::into),
            active_permission: a
                .active_permission
                .into_iter()
                .map(Into::into)
                .collect(),
            frozen_v2: a.frozen_v2.into_iter().map(Into::into).collect(),
            unfrozen_v2: a.unfrozen_v2.into_iter().map(Into::into).collect(),
            delegated_frozen_v2_balance_for_bandwidth: a
                .delegated_frozen_v2_balance_for_bandwidth
                .into(),
            acquired_delegated_frozen_v2_balance_for_bandwidth: a
                .acquired_delegated_frozen_v2_balance_for_bandwidth
                .into(),
        }
    }
}

impl TryFrom<DelegatedResourceAccountIndex>
    for domain::account::DelegatedResourceAccountIndex
{
    type Error = ProtoConvError;
    fn try_from(
        value: DelegatedResourceAccountIndex,
    ) -> Result<Self, Self::Error> {
        Ok(domain::account::DelegatedResourceAccountIndex {
            account: value.account.as_slice().try_into().unwrap_or_default(),
            from_accounts: value
                .from_accounts
                .into_iter()
                .map(|a| a.as_slice().try_into().unwrap_or_default())
                .collect(),
            to_accounts: value
                .to_accounts
                .into_iter()
                .map(|a| a.as_slice().try_into().unwrap_or_default())
                .collect(),
            timestamp: OffsetDateTime::try_from_tron(value.timestamp)?,
        })
    }
}

impl From<domain::account::DelegatedResourceAccountIndex>
    for DelegatedResourceAccountIndex
{
    fn from(value: domain::account::DelegatedResourceAccountIndex) -> Self {
        DelegatedResourceAccountIndex {
            account: value.account.as_bytes().to_vec(),
            from_accounts: value
                .from_accounts
                .into_iter()
                .map(|a| a.as_bytes().to_vec())
                .collect(),
            to_accounts: value
                .to_accounts
                .into_iter()
                .map(|a| a.as_bytes().to_vec())
                .collect(),
            timestamp: value.timestamp.to_tron(),
        }
    }
}

impl TryFrom<DelegatedResource> for domain::account::DelegatedResource {
    type Error = ProtoConvError;
    fn try_from(value: DelegatedResource) -> Result<Self, Self::Error> {
        Ok(domain::account::DelegatedResource {
            from: value.from.as_slice().try_into().unwrap_or_default(),
            to: value.to.as_slice().try_into().unwrap_or_default(),
            frozen_balance_for_bandwidth: value
                .frozen_balance_for_bandwidth
                .into(),
            frozen_balance_for_energy: value.frozen_balance_for_energy.into(),
            expire_time_for_bandwidth: OffsetDateTime::try_from_tron(
                value.expire_time_for_bandwidth,
            )?,
            expire_time_for_energy: OffsetDateTime::try_from_tron(
                value.expire_time_for_energy,
            )?,
        })
    }
}

impl From<domain::account::DelegatedResource> for DelegatedResource {
    fn from(value: domain::account::DelegatedResource) -> Self {
        DelegatedResource {
            from: value.from.as_bytes().to_vec(),
            to: value.to.as_bytes().to_vec(),
            frozen_balance_for_bandwidth: value
                .frozen_balance_for_bandwidth
                .into(),
            frozen_balance_for_energy: value.frozen_balance_for_energy.into(),
            expire_time_for_bandwidth: value
                .expire_time_for_bandwidth
                .to_tron(),
            expire_time_for_energy: value.expire_time_for_energy.to_tron(),
        }
    }
}

// ───────────────────────────────── Block ────────────────────────────────── //

impl TryFrom<block_header::Raw> for domain::block::RawBlockHeader {
    type Error = ProtoConvError;
    fn try_from(p: block_header::Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            timestamp: OffsetDateTime::try_from_tron(p.timestamp)?,
            tx_trie_root: p.tx_trie_root.try_into().unwrap_or_default(),
            parent_hash: p.parent_hash.try_into().unwrap_or_default(),
            number: p.number,
            witness_id: p.witness_id,
            witness_address: TronAddress::try_from(&p.witness_address)
                .expect("invalid witness address"),
            version: p.version,
            account_state_root: p
                .account_state_root
                .try_into()
                .unwrap_or_default(),
        })
    }
}

impl From<domain::block::RawBlockHeader> for block_header::Raw {
    fn from(r: domain::block::RawBlockHeader) -> Self {
        Self {
            timestamp: r.timestamp.to_tron(),
            tx_trie_root: r.tx_trie_root.into(),
            parent_hash: r.parent_hash.into(),
            number: r.number,
            witness_id: r.witness_id,
            witness_address: r.witness_address.as_bytes().to_vec(),
            version: r.version,
            account_state_root: r.account_state_root.into(),
        }
    }
}

impl TryFrom<BlockHeader> for domain::block::BlockHeader {
    type Error = ProtoConvError;
    fn try_from(p: BlockHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            raw_data: p
                .raw_data
                .map(TryInto::try_into)
                .ok_or(ProtoConvError::Missing("raw block header"))??,
            witness_signature: p
                .witness_signature
                .as_slice()
                .try_into()
                .expect("failed to build recoverable signature from bytes"),
        })
    }
}

impl From<domain::block::BlockHeader> for BlockHeader {
    fn from(d: domain::block::BlockHeader) -> Self {
        Self {
            raw_data: Some(d.raw_data.into()),
            witness_signature: d.witness_signature.into(),
        }
    }
}

impl TryFrom<Block> for domain::block::Block {
    type Error = ProtoConvError;
    fn try_from(p: Block) -> Result<Self, Self::Error> {
        Ok(Self {
            transactions: p
                .transactions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            block_header: p.block_header.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<domain::block::Block> for Block {
    fn from(b: domain::block::Block) -> Self {
        Self {
            transactions: b.transactions.into_iter().map(Into::into).collect(),
            block_header: b.block_header.map(Into::into),
        }
    }
}

impl TryFrom<BlockExtention> for domain::block::BlockExtention {
    type Error = ProtoConvError;
    fn try_from(p: BlockExtention) -> Result<Self, Self::Error> {
        Ok(Self {
            transactions: p
                .transactions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            block_header: p
                .block_header
                .ok_or(ProtoConvError::Missing("block header"))?
                .try_into()?,
            blockid: p.blockid.try_into().unwrap_or_default(),
        })
    }
}

impl From<domain::block::BlockExtention> for BlockExtention {
    fn from(p: domain::block::BlockExtention) -> Self {
        Self {
            transactions: p.transactions.into_iter().map(Into::into).collect(),
            block_header: Some(p.block_header.into()),
            blockid: p.blockid.into(),
        }
    }
}

impl_enum_conversions! {
    ResourceCode => domain::contract::ResourceCode {
        Bandwidth,
        Energy,
        TronPower
    }
}
