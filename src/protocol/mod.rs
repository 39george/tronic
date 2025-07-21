#![cfg(not(doctest))]

//! That module contains auto-generated submodules, using tonic-build.

use k256::ecdsa::{RecoveryId, Signature};
pub use protocol::*;

use crate::domain::{self, RecoverableSignature, address::TronAddress};

// #[path = "google.api.rs"]
// mod google_api;
mod protocol;

pub const TRON_PROTOCOL_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("tron_protocol_descriptor");

// ────────────────────────────── Transaction ─────────────────────────────── //

impl From<transaction::Result> for domain::transaction::TransactionResult {
    fn from(r: transaction::Result) -> Self {
        domain::transaction::TransactionResult {
            fee: r.fee,
            ret: r.ret,
            contract_ret: r.contract_ret,
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
            contract_ret: r.contract_ret,
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

impl From<transaction::Contract> for domain::contract::Contract {
    fn from(c: transaction::Contract) -> Self {
        // TODO: implement
        domain::contract::Contract {}
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
            ref_block_bytes: r.ref_block_bytes,
            ref_block_num: r.ref_block_num,
            ref_block_hash: r.ref_block_hash,
            expiration: r.expiration,
            data: r.data,
            contract: r.contract.pop().map(Into::into),
            scripts: r.scripts,
            timestamp: r.timestamp,
            fee_limit: r.fee_limit,
        }
    }
}

impl From<domain::transaction::RawTransaction> for transaction::Raw {
    fn from(r: domain::transaction::RawTransaction) -> Self {
        transaction::Raw {
            ref_block_bytes: r.ref_block_bytes,
            ref_block_num: r.ref_block_num,
            ref_block_hash: r.ref_block_hash,
            expiration: r.expiration,
            data: r.data,
            contract: r.contract.map(|c| vec![c.into()]).unwrap_or_default(),
            scripts: r.scripts,
            timestamp: r.timestamp,
            fee_limit: r.fee_limit,
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
            txid: txext.txid.into(),
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
            txid: txext.txid.0,
            constant_result: txext.constant_result,
            energy_used: txext.energy_used,
            energy_penalty: txext.energy_penalty,
            ..Default::default()
        }
    }
}

impl From<TriggerSmartContract> for domain::contract::TriggerSmartContract {
    fn from(t: TriggerSmartContract) -> Self {
        domain::contract::TriggerSmartContract {
            owner_address: TronAddress::try_from(&t.owner_address)
                .expect("Invalid owner address"),
            contract_address: TronAddress::try_from(&t.contract_address)
                .expect("Invalid contract address"),
            call_value: t.call_value,
            data: t.data,
            call_token_value: t.call_token_value,
            token_id: t.token_id,
        }
    }
}

impl From<domain::contract::TriggerSmartContract> for TriggerSmartContract {
    fn from(t: domain::contract::TriggerSmartContract) -> Self {
        TriggerSmartContract {
            owner_address: t.owner_address.as_bytes().to_vec(),
            contract_address: t.contract_address.as_bytes().to_vec(),
            call_value: t.call_value,
            data: t.data,
            call_token_value: t.call_token_value,
            token_id: t.token_id,
        }
    }
}

// ──────────────────────────────── Account ───────────────────────────────── //

impl From<Key> for crate::domain::account::Key {
    fn from(k: Key) -> Self {
        Self {
            address: TronAddress::try_from(&k.address)
                .expect("invalid address"),
            weight: k.weight,
        }
    }
}

impl From<crate::domain::account::Key> for Key {
    fn from(k: crate::domain::account::Key) -> Self {
        Self {
            address: k.address.as_bytes().to_vec(),
            weight: k.weight,
        }
    }
}

impl From<i32> for crate::domain::account::PermissionType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::Owner,
            1 => Self::Witness,
            2 => Self::Active,
            _ => Self::Active,
        }
    }
}

impl From<crate::domain::account::PermissionType> for i32 {
    fn from(p: crate::domain::account::PermissionType) -> Self {
        p as i32
    }
}

impl From<Permission> for crate::domain::account::Permission {
    fn from(p: Permission) -> Self {
        Self {
            permission_type: p.r#type.into(),
            id: p.id,
            permission_name: p.permission_name,
            threshold: p.threshold,
            parent_id: p.parent_id,
            operations: p.operations,
            keys: p.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<crate::domain::account::Permission> for Permission {
    fn from(p: crate::domain::account::Permission) -> Self {
        Self {
            r#type: p.permission_type.into(),
            id: p.id,
            permission_name: p.permission_name,
            threshold: p.threshold,
            parent_id: p.parent_id,
            operations: p.operations,
            keys: p.keys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<account::Frozen> for crate::domain::account::Frozen {
    fn from(f: account::Frozen) -> Self {
        Self {
            frozen_balance: domain::trx::Trx::from(f.frozen_balance),
            expire_time: time::OffsetDateTime::from_unix_timestamp(
                f.expire_time,
            )
            .inspect(|e| {
                tracing::error!(
                    "failed to create OffsetDateTime from unix_timestamp: {e}"
                )
            })
            .unwrap_or_else(|_| time::OffsetDateTime::UNIX_EPOCH),
        }
    }
}

impl From<crate::domain::account::Frozen> for account::Frozen {
    fn from(f: crate::domain::account::Frozen) -> Self {
        Self {
            frozen_balance: f.frozen_balance.to_sun(),
            expire_time: f.expire_time.unix_timestamp(),
        }
    }
}

impl From<account::FreezeV2> for crate::domain::account::FreezeV2 {
    fn from(f: account::FreezeV2) -> Self {
        Self {
            freeze_type: f.r#type,
            amount: f.amount.into(),
        }
    }
}

impl From<crate::domain::account::FreezeV2> for account::FreezeV2 {
    fn from(f: crate::domain::account::FreezeV2) -> Self {
        Self {
            r#type: f.freeze_type,
            amount: f.amount.to_sun(),
        }
    }
}

impl From<account::UnFreezeV2> for crate::domain::account::UnFreezeV2 {
    fn from(f: account::UnFreezeV2) -> Self {
        Self {
            unfreeze_type: f.r#type,
            unfreeze_amount: f.unfreeze_amount.into(),
            unfreeze_expire_time: time::OffsetDateTime::from_unix_timestamp(
                f.unfreeze_expire_time,
            )
            .inspect(|e| {
                tracing::error!(
                    "failed to create OffsetDateTime from unix_timestamp: {e}"
                )
            })
            .unwrap_or_else(|_| time::OffsetDateTime::UNIX_EPOCH),
        }
    }
}

impl From<crate::domain::account::UnFreezeV2> for account::UnFreezeV2 {
    fn from(f: crate::domain::account::UnFreezeV2) -> Self {
        Self {
            r#type: f.unfreeze_type,
            unfreeze_amount: f.unfreeze_amount.to_sun(),
            unfreeze_expire_time: f.unfreeze_expire_time.unix_timestamp(),
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
            latest_consume_time_for_energy: r.latest_consume_time_for_energy,
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
            latest_consume_time_for_energy: r.latest_consume_time_for_energy,
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

impl From<Account> for domain::account::Account {
    fn from(a: Account) -> Self {
        Self {
            account_name: a.account_name,
            r#type: a.r#type,
            address: a.address,
            balance: a.balance,
            votes: a.votes.into_iter().map(Into::into).collect(),
            asset: a.asset,
            asset_v2: a.asset_v2,
            frozen: a.frozen.into_iter().map(Into::into).collect(),
            net_usage: a.net_usage,
            acquired_delegated_frozen_balance_for_bandwidth: a
                .acquired_delegated_frozen_balance_for_bandwidth,
            delegated_frozen_balance_for_bandwidth: a
                .delegated_frozen_balance_for_bandwidth,
            old_tron_power: a.old_tron_power,
            tron_power: a.tron_power.map(Into::into),
            asset_optimized: a.asset_optimized,
            create_time: a.create_time,
            latest_opration_time: a.latest_opration_time,
            allowance: a.allowance,
            latest_withdraw_time: a.latest_withdraw_time,
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
            latest_consume_time: a.latest_consume_time,
            latest_consume_free_time: a.latest_consume_free_time,
            account_id: a.account_id,
            net_window_size: a.net_window_size,
            net_window_optimized: a.net_window_optimized,
            account_resource: a.account_resource.map(Into::into),
            code_hash: a.code_hash,
            owner_permission: a.owner_permission.map(Into::into),
            witness_permission: a.witness_permission.map(Into::into),
            active_permission: a
                .active_permission
                .into_iter()
                .map(Into::into)
                .collect(),
            frozen_v2: a.frozen_v2.into_iter().map(Into::into).collect(),
            unfrozen_v2: a.unfrozen_v2.into_iter().map(Into::into).collect(),
            delegated_frozen_v2_balance_for_bandwidth: a
                .delegated_frozen_v2_balance_for_bandwidth,
            acquired_delegated_frozen_v2_balance_for_bandwidth: a
                .acquired_delegated_frozen_v2_balance_for_bandwidth,
        }
    }
}

impl From<domain::account::Account> for Account {
    fn from(a: domain::account::Account) -> Self {
        Self {
            account_name: a.account_name,
            r#type: a.r#type,
            address: a.address,
            balance: a.balance,
            votes: a.votes.into_iter().map(Into::into).collect(),
            asset: a.asset,
            asset_v2: a.asset_v2,
            frozen: a.frozen.into_iter().map(Into::into).collect(),
            net_usage: a.net_usage,
            acquired_delegated_frozen_balance_for_bandwidth: a
                .acquired_delegated_frozen_balance_for_bandwidth,
            delegated_frozen_balance_for_bandwidth: a
                .delegated_frozen_balance_for_bandwidth,
            old_tron_power: a.old_tron_power,
            tron_power: a.tron_power.map(Into::into),
            asset_optimized: a.asset_optimized,
            create_time: a.create_time,
            latest_opration_time: a.latest_opration_time,
            allowance: a.allowance,
            latest_withdraw_time: a.latest_withdraw_time,
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
            latest_consume_time: a.latest_consume_time,
            latest_consume_free_time: a.latest_consume_free_time,
            account_id: a.account_id,
            net_window_size: a.net_window_size,
            net_window_optimized: a.net_window_optimized,
            account_resource: a.account_resource.map(Into::into),
            code_hash: a.code_hash,
            owner_permission: a.owner_permission.map(Into::into),
            witness_permission: a.witness_permission.map(Into::into),
            active_permission: a
                .active_permission
                .into_iter()
                .map(Into::into)
                .collect(),
            frozen_v2: a.frozen_v2.into_iter().map(Into::into).collect(),
            unfrozen_v2: a.unfrozen_v2.into_iter().map(Into::into).collect(),
            delegated_frozen_v2_balance_for_bandwidth: a
                .delegated_frozen_v2_balance_for_bandwidth,
            acquired_delegated_frozen_v2_balance_for_bandwidth: a
                .acquired_delegated_frozen_v2_balance_for_bandwidth,
        }
    }
}

// ───────────────────────────────── Block ────────────────────────────────── //

impl From<block_header::Raw> for domain::block::RawBlockHeader {
    fn from(p: block_header::Raw) -> Self {
        Self {
            timestamp: 
             time::OffsetDateTime::from_unix_timestamp(
                p.timestamp,
            )
            .inspect(|e| {
                tracing::error!(
                    "failed to create OffsetDateTime from unix_timestamp: {e}"
                )
            })
            .unwrap_or_else(|_| time::OffsetDateTime::UNIX_EPOCH),
            tx_trie_root: p.tx_trie_root,
            parent_hash: p.parent_hash,
            number: p.number,
            witness_id: p.witness_id,
            witness_address: TronAddress::try_from(&p.witness_address).expect("invalid witness address"),
            version: p.version,
            account_state_root: p.account_state_root,
        }
    }
}

impl From<domain::block::RawBlockHeader> for block_header::Raw {
    fn from(r: domain::block::RawBlockHeader) -> Self {
        Self {
            timestamp: r.timestamp.unix_timestamp(),
            tx_trie_root: r.tx_trie_root,
            parent_hash: r.parent_hash,
            number: r.number,
            witness_id: r.witness_id,
            witness_address: r.witness_address.as_bytes().to_vec(),
            version: r.version,
            account_state_root: r.account_state_root,
        }
    }
}

impl From<BlockHeader> for domain::block::BlockHeader {
    fn from(p: BlockHeader) -> Self {
        Self {
            raw_data: p.raw_data.map(Into::into),
            witness_signature: p.witness_signature.as_slice().try_into().expect("failed to build recoverable signature from bytes"),
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
            blockid: p.blockid.into(),
        }
    }
}

impl From<domain::block::BlockExtention> for BlockExtention {
    fn from(p: domain::block::BlockExtention) -> Self {
        Self {
            transactions: p.transactions.into_iter().map(Into::into).collect(),
            block_header: p.block_header.map(Into::into),
            blockid: p.blockid.0,
        }
    }
}
