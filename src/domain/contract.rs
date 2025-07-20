use crate::domain::address::TronAddress;

#[derive(Clone, PartialEq)]
pub struct Contract {}

pub struct TriggerSmartContract {
    pub owner_address: TronAddress,
    pub contract_address: TronAddress,
    pub call_value: i64,
    pub data: Vec<u8>,
    pub call_token_value: i64,
    pub token_id: i64,
}
