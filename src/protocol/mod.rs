#![cfg(not(doctest))]

//! That module contains auto-generated submodules, using tonic-build.

use k256::ecdsa::{RecoveryId, Signature};
pub use protocol::*;

use crate::{
    domain::{
        self, RecoverableSignature, address::TronAddress, permission::Ops,
    },
    impl_enum_conversions,
};

// #[path = "google.api.rs"]
// mod google_api;
mod protocol;

pub mod contracts_conversions;

pub const TRON_PROTOCOL_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("tron_protocol_descriptor");

// ────────────────────────────── Transaction ─────────────────────────────── //

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

impl From<transaction::Result> for domain::transaction::TransactionResult {
    fn from(r: transaction::Result) -> Self {
        domain::transaction::TransactionResult {
            fee: r.fee,
            ret: r.ret,
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
            // TODO: Use appropriate type
            order_details: vec![domain::transaction::UnknownType],
            withdraw_expire_amount: r.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: r.cancel_unfreeze_v2_amount,
        }
    }
}

impl From<domain::transaction::TransactionResult> for transaction::Result {
    fn from(r: domain::transaction::TransactionResult) -> Self {
        transaction::Result {
            fee: r.fee,
            ret: r.ret,
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
            // TODO: Use appropriate type
            order_details: Default::default(),
            withdraw_expire_amount: r.withdraw_expire_amount,
            cancel_unfreeze_v2_amount: r.cancel_unfreeze_v2_amount,
        }
    }
}

impl From<domain::contract::Contract> for transaction::Contract {
    fn from(c: domain::contract::Contract) -> Self {
        // TODO: implement
        transaction::Contract::default()
    }
}

impl From<transaction::Raw> for domain::transaction::RawTransaction {
    fn from(mut r: transaction::Raw) -> Self {
        domain::transaction::RawTransaction {
            ref_block_bytes: r.ref_block_bytes.try_into().unwrap(),
            ref_block_num: r.ref_block_num,
            ref_block_hash: r.ref_block_hash.try_into().unwrap_or_default(),
            expiration: tron_to_datetime(r.expiration),
            data: r.data.into(),
            contract: r.contract.pop().map(Into::into),
            scripts: r.scripts,
            timestamp: tron_to_datetime(r.timestamp),
            fee_limit: r.fee_limit.into(),
        }
    }
}

impl From<domain::transaction::RawTransaction> for transaction::Raw {
    fn from(r: domain::transaction::RawTransaction) -> Self {
        transaction::Raw {
            ref_block_bytes: r.ref_block_bytes.into(),
            ref_block_num: r.ref_block_num,
            ref_block_hash: r.ref_block_hash.try_into().unwrap_or_default(),
            expiration: datetime_to_tron(r.expiration),
            data: r.data.into(),
            contract: r.contract.map(|c| vec![c.into()]).unwrap_or_default(),
            scripts: r.scripts,
            timestamp: datetime_to_tron(r.timestamp),
            fee_limit: r.fee_limit.into(),
            auths: Default::default(),
        }
    }
}

impl From<Transaction> for domain::transaction::Transaction {
    fn from(t: Transaction) -> Self {
        domain::transaction::Transaction {
            raw: t.raw_data.map(Into::into),
            signature: t
                .signature
                .into_iter()
                .map(|s| {
                    TryInto::<RecoverableSignature>::try_into(s.as_slice())
                })
                .collect::<Result<_, _>>()
                .unwrap(),
            result: t.ret.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::transaction::Transaction> for Transaction {
    fn from(t: domain::transaction::Transaction) -> Self {
        Transaction {
            raw_data: t.raw.map(Into::into),
            signature: t.signature.into_iter().map(Into::into).collect(),
            ret: t.result.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TransactionExtention> for domain::transaction::TransactionExtention {
    fn from(txext: TransactionExtention) -> Self {
        domain::transaction::TransactionExtention {
            transaction: txext.transaction.map(Into::into),
            txid: txext.txid.try_into().unwrap_or_default(),
            constant_result: txext.constant_result,
            energy_used: txext.energy_used,
            energy_penalty: txext.energy_penalty,
        }
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
            ..Default::default()
        }
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

impl From<account::Frozen> for crate::domain::account::Frozen {
    fn from(f: account::Frozen) -> Self {
        Self {
            frozen_balance: domain::trx::Trx::from(f.frozen_balance),
            expire_time: tron_to_datetime(f.expire_time),
        }
    }
}

impl From<crate::domain::account::Frozen> for account::Frozen {
    fn from(f: crate::domain::account::Frozen) -> Self {
        Self {
            frozen_balance: f.frozen_balance.to_sun(),
            expire_time: datetime_to_tron(f.expire_time),
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

impl From<account::UnFreezeV2> for crate::domain::account::UnFreezeV2 {
    fn from(f: account::UnFreezeV2) -> Self {
        Self {
            unfreeze_type: f.r#type,
            unfreeze_amount: f.unfreeze_amount.into(),
            unfreeze_expire_time: tron_to_datetime(f.unfreeze_expire_time),
        }
    }
}

impl From<crate::domain::account::UnFreezeV2> for account::UnFreezeV2 {
    fn from(f: crate::domain::account::UnFreezeV2) -> Self {
        Self {
            r#type: f.unfreeze_type,
            unfreeze_amount: f.unfreeze_amount.to_sun(),
            unfreeze_expire_time: datetime_to_tron(f.unfreeze_expire_time),
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

impl From<account::AccountResource> for domain::account::AccountResource {
    fn from(r: account::AccountResource) -> Self {
        Self {
            energy_usage: r.energy_usage,
            frozen_balance_for_energy: r
                .frozen_balance_for_energy
                .map(Into::into),
            latest_consume_time_for_energy: tron_to_datetime(
                r.latest_consume_time_for_energy,
            ),
            acquired_delegated_frozen_balance_for_energy: r
                .acquired_delegated_frozen_balance_for_energy,
            delegated_frozen_balance_for_energy: r
                .delegated_frozen_balance_for_energy,
            storage_limit: r.storage_limit,
            storage_usage: r.storage_usage,
            latest_exchange_storage_time: r.latest_exchange_storage_time,
            energy_window_size: r.energy_window_size,
            delegated_frozen_v2_balance_for_energy: r
                .delegated_frozen_v2_balance_for_energy,
            acquired_delegated_frozen_v2_balance_for_energy: r
                .acquired_delegated_frozen_v2_balance_for_energy,
            energy_window_optimized: r.energy_window_optimized,
        }
    }
}

impl From<domain::account::AccountResource> for account::AccountResource {
    fn from(r: domain::account::AccountResource) -> Self {
        Self {
            energy_usage: r.energy_usage,
            frozen_balance_for_energy: r
                .frozen_balance_for_energy
                .map(Into::into),
            latest_consume_time_for_energy: datetime_to_tron(
                r.latest_consume_time_for_energy,
            ),
            acquired_delegated_frozen_balance_for_energy: r
                .acquired_delegated_frozen_balance_for_energy,
            delegated_frozen_balance_for_energy: r
                .delegated_frozen_balance_for_energy,
            storage_limit: r.storage_limit,
            storage_usage: r.storage_usage,
            latest_exchange_storage_time: r.latest_exchange_storage_time,
            energy_window_size: r.energy_window_size,
            delegated_frozen_v2_balance_for_energy: r
                .delegated_frozen_v2_balance_for_energy,
            acquired_delegated_frozen_v2_balance_for_energy: r
                .acquired_delegated_frozen_v2_balance_for_energy,
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
            total_net_weight: r.total_net_weight,
            total_tron_power_weight: r.total_tron_power_weight,
            tron_power_used: r.tron_power_used,
            tron_power_limit: r.tron_power_limit,
            energy_used: r.energy_used,
            energy_limit: r.energy_limit,
            total_energy_limit: r.total_energy_limit,
            total_energy_weight: r.total_energy_weight,
            storage_used: r.storage_used,
            storage_limit: r.storage_limit,
        }
    }
}

impl From<Account> for domain::account::Account {
    fn from(a: Account) -> Self {
        Self {
            account_type: a.r#type().into(),
            account_name: a.account_name.into(),
            address: a.address.as_slice().try_into().unwrap_or_default(),
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
            create_time: tron_to_datetime(a.create_time),
            latest_opration_time: tron_to_datetime(a.latest_opration_time),
            allowance: a.allowance,
            latest_withdraw_time: tron_to_datetime(a.latest_withdraw_time),
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
            latest_consume_time: tron_to_datetime(a.latest_consume_time),
            latest_consume_free_time: tron_to_datetime(
                a.latest_consume_free_time,
            ),
            account_id: a.account_id,
            net_window_size: a.net_window_size,
            net_window_optimized: a.net_window_optimized,
            account_resource: a
                .account_resource
                .map(Into::into)
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
            create_time: datetime_to_tron(a.create_time),
            latest_opration_time: datetime_to_tron(a.latest_opration_time),
            allowance: a.allowance,
            latest_withdraw_time: datetime_to_tron(a.latest_withdraw_time),
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
            latest_consume_time: datetime_to_tron(a.latest_consume_time),
            latest_consume_free_time: datetime_to_tron(
                a.latest_consume_free_time,
            ),
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

// ───────────────────────────────── Block ────────────────────────────────── //

impl From<block_header::Raw> for domain::block::RawBlockHeader {
    fn from(p: block_header::Raw) -> Self {
        Self {
            timestamp: tron_to_datetime(p.timestamp),
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
        }
    }
}

impl From<domain::block::RawBlockHeader> for block_header::Raw {
    fn from(r: domain::block::RawBlockHeader) -> Self {
        Self {
            timestamp: datetime_to_tron(r.timestamp),
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

impl From<BlockHeader> for domain::block::BlockHeader {
    fn from(p: BlockHeader) -> Self {
        Self {
            raw_data: p.raw_data.map(Into::into),
            witness_signature: p
                .witness_signature
                .as_slice()
                .try_into()
                .expect("failed to build recoverable signature from bytes"),
        }
    }
}

impl From<domain::block::BlockHeader> for BlockHeader {
    fn from(d: domain::block::BlockHeader) -> Self {
        Self {
            raw_data: d.raw_data.map(Into::into),
            witness_signature: d.witness_signature.into(),
        }
    }
}

impl From<Block> for domain::block::Block {
    fn from(p: Block) -> Self {
        Self {
            transactions: p.transactions.into_iter().map(Into::into).collect(),
            block_header: p.block_header.map(Into::into),
        }
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

impl From<BlockExtention> for domain::block::BlockExtention {
    fn from(p: BlockExtention) -> Self {
        Self {
            transactions: p.transactions.into_iter().map(Into::into).collect(),
            block_header: p.block_header.map(Into::into),
            blockid: p.blockid.try_into().unwrap_or_default(),
        }
    }
}

impl From<domain::block::BlockExtention> for BlockExtention {
    fn from(p: domain::block::BlockExtention) -> Self {
        Self {
            transactions: p.transactions.into_iter().map(Into::into).collect(),
            block_header: p.block_header.map(Into::into),
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

// ──────────────────────────────── Helpers ───────────────────────────────── //

fn tron_to_datetime(time: i64) -> time::OffsetDateTime {
    time::OffsetDateTime::from_unix_timestamp_nanos(time as i128 * 1_000_000)
        .inspect_err(|e| {
            tracing::error!(
                "failed to create OffsetDateTime from unix_timestamp: {e}"
            )
        })
        .unwrap_or_else(|_| time::OffsetDateTime::UNIX_EPOCH)
}

fn datetime_to_tron(dt: time::OffsetDateTime) -> i64 {
    (dt.unix_timestamp_nanos() / 1_000_000) as i64
}
