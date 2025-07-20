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
            energy_used: todo!(),
            energy_penalty: todo!(),
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
