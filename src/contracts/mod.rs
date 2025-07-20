use alloy_primitives::U256;
use alloy_sol_macro::sol;

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
