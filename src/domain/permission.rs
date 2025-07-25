use bitvec::{bitvec, view::BitView};

use crate::{
    define_fixed_hash, define_fixed_string, domain::address::TronAddress,
};

#[derive(Debug, Clone, Copy, PartialEq, strum_macros::FromRepr)]
#[repr(usize)]
pub enum Ops {
    AccountCreateContract = 0,
    TransferContract = 1,
    TransferAssetContract = 2,
    VoteAssetContract = 3,
    VoteWitnessContract = 4,
    WitnessCreateContract = 5,
    AssetIssueContract = 6,
    WitnessUpdateContract = 8,
    ParticipateAssetIssueContract = 9,
    AccountUpdateContract = 10,
    FreezeBalanceContract = 11,
    UnfreezeBalanceContract = 12,
    WithdrawBalanceContract = 13,
    UnfreezeAssetContract = 14,
    UpdateAssetContract = 15,
    ProposalCreateContract = 16,
    ProposalApproveContract = 17,
    ProposalDeleteContract = 18,
    SetAccountIdContract = 19,
    CustomContract = 20,
    CreateSmartContract = 30,
    TriggerSmartContract = 31,
    GetContract = 32,
    UpdateSettingContract = 33,
    ExchangeCreateContract = 41,
    ExchangeInjectContract = 42,
    ExchangeWithdrawContract = 43,
    ExchangeTransactionContract = 44,
    UpdateEnergyLimitContract = 45,
    AccountPermissionUpdateContract = 46,
    ClearAbiContract = 48,
    UpdateBrokerageContract = 49,
    ShieldedTransferContract = 51,
    MarketSellAssetContract = 52,
    MarketCancelOrderContract = 53,
    FreezeBalanceV2Contract = 54,
    UnfreezeBalanceV2Contract = 55,
    WithdrawExpireUnfreezeContract = 56,
    DelegateResourceContract = 57,
    UnDelegateResourceContract = 58,
    CancelAllUnfreezeV2Contract = 59,
}

impl Ops {
    /// Encodes a list of contract types into a 32-byte operations string
    pub fn encode_ops(ops: &[Ops]) -> Vec<u8> {
        if ops.is_empty() {
            return Vec::new();
        }

        let mut bits = bitvec![u8, bitvec::order::Lsb0; 0; 256]; // 32 bytes * 8 bits

        for op in ops {
            let num: usize = (*op) as usize;
            if (0..256).contains(&num) {
                bits.set(num, true);
            }
        }

        bits.into_vec()
    }

    /// Decodes an operations string into a list of contract types
    pub fn decode_ops(operations: &[u8]) -> Vec<Ops> {
        let mut result = Vec::new();

        let bits = operations.view_bits::<bitvec::order::Lsb0>();

        for (pos, bit) in bits.iter().enumerate() {
            if *bit {
                let contract = Ops::from_repr(pos).unwrap();
                result.push(contract);
            }
        }

        result
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Key {
    pub address: TronAddress,
    pub weight: i64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PermissionType {
    #[default]
    Owner = 0,
    Witness = 1,
    Active = 2,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Permission {
    pub permission_type: PermissionType,
    /// Owner id=0, Witness id=1, Active id start by 2
    pub id: i32,
    pub permission_name: PermissionName,
    pub threshold: i64,
    pub parent_id: i32,
    pub operations: Vec<Ops>,
    pub keys: Vec<Key>,
}

define_fixed_string!(PermissionName, 32, "TRON permission name (max 32 bytes)");
