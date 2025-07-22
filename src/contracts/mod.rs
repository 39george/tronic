use std::str::FromStr;

use alloy_primitives::U256;
use alloy_sol_macro::sol;
use alloy_sol_types::SolCall;

use crate::domain::address::TronAddress;

sol! {
    #[derive(Debug)]
    contract Trc20 {
        function transfer(address recipient, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

pub fn trc20_transfer(
    address: TronAddress,
    amount: u64,
) -> Trc20::transferCall {
    let address: alloy_primitives::Address = address.into();
    Trc20::transferCall {
        recipient: address,
        amount: U256::from(amount),
    }
}

pub fn trc20_balance_of(address: TronAddress) -> Trc20::balanceOfCall {
    let address: alloy_primitives::Address = address.into();
    Trc20::balanceOfCall { account: address }
}

// Decode a transfer function call from raw bytes
pub fn decode_transfer_call(
    data: &[u8],
) -> Result<Trc20::transferCall, anyhow::Error> {
    // Decode the data bytes into the transfer call
    let transfer_call = Trc20::transferCall::abi_decode(data)?;

    Ok(transfer_call)
}

// Decode a balanceOf function call from raw bytes
pub fn decode_balance_of_call(
    data: &[u8],
) -> Result<Trc20::balanceOfCall, anyhow::Error> {
    // Decode the data bytes into the balanceOf call
    let balance_of_call = Trc20::balanceOfCall::abi_decode(data)?;

    Ok(balance_of_call)
}
