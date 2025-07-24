use alloy_primitives::U256;
use alloy_sol_macro::sol;
use alloy_sol_types::SolCall;

use crate::domain::address::TronAddress;

sol! {
    #[derive(Debug)]
    contract Trc20 {
         /// Send amount of tokens from sender to recipient
        function transfer(address recipient, uint256 amount) external returns (bool);

        /// Check the token balance of a given account
        function balanceOf(address account) external view returns (uint256);

        /// Allow spender to withdraw from the sender's account multiple times
        function approve(address spender, uint256 amount) external returns (bool);

        /// Check the remaining amount spender is allowed to spend on behalf of owner
        function allowance(address owner, address spender) external view returns (uint256);

        /// Transfer tokens from one account to another, using the allowance
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

        /// Events to emit for Transfer and Approval actions
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);
    }
}

pub struct Trc20Contract {
    contract_address: TronAddress,
}

impl Trc20Contract {
    pub fn new(contract_address: TronAddress) -> Self {
        Trc20Contract { contract_address }
    }

    pub fn address(&self) -> TronAddress {
        self.contract_address
    }

    /// Send amount of tokens from sender to recipient
    pub fn transfer(
        &self,
        recipient: TronAddress,
        amount: u64,
    ) -> Trc20::transferCall {
        let recipient: alloy_primitives::Address = recipient.into();
        Trc20::transferCall {
            recipient,
            amount: U256::from(amount),
        }
    }

    /// Check the token balance of a given account
    pub fn balance_of(&self, account: TronAddress) -> Trc20::balanceOfCall {
        let account: alloy_primitives::Address = account.into();
        Trc20::balanceOfCall { account }
    }

    /// Allow spender to withdraw from the sender's account multiple times
    pub fn approve(
        &self,
        spender: TronAddress,
        amount: u64,
    ) -> Trc20::approveCall {
        let spender: alloy_primitives::Address = spender.into();
        Trc20::approveCall {
            spender,
            amount: U256::from(amount),
        }
    }

    /// Check the remaining amount spender is allowed to spend on behalf of owner
    pub fn allowance(
        &self,
        owner: TronAddress,
        spender: TronAddress,
    ) -> Trc20::allowanceCall {
        let owner: alloy_primitives::Address = owner.into();
        let spender: alloy_primitives::Address = spender.into();
        Trc20::allowanceCall { owner, spender }
    }

    /// Transfer tokens from one account to another, using the allowance
    pub fn transfer_from(
        &self,
        sender: TronAddress,
        recipient: TronAddress,
        amount: u64,
    ) -> Trc20::transferFromCall {
        let sender: alloy_primitives::Address = sender.into();
        let recipient: alloy_primitives::Address = recipient.into();
        Trc20::transferFromCall {
            sender,
            recipient,
            amount: U256::from(amount),
        }
    }

    /// Decode a `transfer` function call from raw bytes
    pub fn decode_transfer_call(
        data: &[u8],
    ) -> Result<Trc20::transferCall, anyhow::Error> {
        let transfer_call = Trc20::transferCall::abi_decode(data)?;
        Ok(transfer_call)
    }

    /// Decode a `balanceOf` function call from raw bytes
    pub fn decode_balance_of_call(
        data: &[u8],
    ) -> Result<Trc20::balanceOfCall, anyhow::Error> {
        let balance_of_call = Trc20::balanceOfCall::abi_decode(data)?;
        Ok(balance_of_call)
    }

    // Decode an `approve` function call from raw bytes
    pub fn decode_approve_call(
        data: &[u8],
    ) -> Result<Trc20::approveCall, anyhow::Error> {
        let approve_call = Trc20::approveCall::abi_decode(data)?;
        Ok(approve_call)
    }

    /// Decode an `allowance` function call from raw bytes
    pub fn decode_allowance_call(
        data: &[u8],
    ) -> Result<Trc20::allowanceCall, anyhow::Error> {
        let allowance_call = Trc20::allowanceCall::abi_decode(data)?;
        Ok(allowance_call)
    }

    /// Decode a `transferFrom` function call from raw bytes
    pub fn decode_transfer_from_call(
        data: &[u8],
    ) -> Result<Trc20::transferFromCall, anyhow::Error> {
        let transfer_from_call = Trc20::transferFromCall::abi_decode(data)?;
        Ok(transfer_from_call)
    }
}
