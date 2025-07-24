use alloy_primitives::U256;

use crate::{
    AddressExtractor,
    contracts::AbiDecode,
    domain::{address::TronAddress, contract::TriggerSmartContract},
};

use super::TryFromData;

#[allow(non_camel_case_types, non_snake_case)]
pub mod Trc20 {
    use crate::{
        contracts::{AbiDecode, AbiEncode},
        domain::address::TronAddress,
    };
    use alloy_primitives::U256;
    use alloy_sol_macro::sol;
    use alloy_sol_types::SolCall;

    sol! {
        #[derive(Debug)]
        contract Erc20 {
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

    #[derive(Clone, Debug)]
    pub struct transferCall {
        pub recipient: TronAddress,
        pub amount: U256,
    }

    #[derive(Clone, Debug)]
    pub struct balanceOfCall {
        pub account: TronAddress,
    }

    #[derive(Clone, Debug)]
    pub struct approveCall {
        pub spender: TronAddress,
        pub amount: U256,
    }

    #[derive(Clone, Debug)]
    pub struct allowanceCall {
        pub owner: TronAddress,
        pub spender: TronAddress,
    }

    #[derive(Clone, Debug)]
    pub struct transferFromCall {
        pub sender: TronAddress,
        pub recipient: TronAddress,
        pub amount: U256,
    }

    // Conversions from Erc20 types to Trc20 types
    impl From<Erc20::transferCall> for transferCall {
        fn from(call: Erc20::transferCall) -> Self {
            transferCall {
                recipient: call.recipient.into(),
                amount: call.amount,
            }
        }
    }

    impl From<Erc20::balanceOfCall> for balanceOfCall {
        fn from(call: Erc20::balanceOfCall) -> Self {
            balanceOfCall {
                account: call.account.into(),
            }
        }
    }

    impl From<Erc20::approveCall> for approveCall {
        fn from(call: Erc20::approveCall) -> Self {
            approveCall {
                spender: call.spender.into(),
                amount: call.amount,
            }
        }
    }

    impl From<Erc20::allowanceCall> for allowanceCall {
        fn from(call: Erc20::allowanceCall) -> Self {
            allowanceCall {
                owner: call.owner.into(),
                spender: call.spender.into(),
            }
        }
    }

    impl From<Erc20::transferFromCall> for transferFromCall {
        fn from(call: Erc20::transferFromCall) -> Self {
            transferFromCall {
                sender: call.sender.into(),
                recipient: call.recipient.into(),
                amount: call.amount,
            }
        }
    }

    // Conversions from Trc20 types to Erc20 types
    impl From<transferCall> for Erc20::transferCall {
        fn from(call: transferCall) -> Self {
            Erc20::transferCall {
                recipient: call.recipient.into(),
                amount: call.amount,
            }
        }
    }

    impl From<balanceOfCall> for Erc20::balanceOfCall {
        fn from(call: balanceOfCall) -> Self {
            Erc20::balanceOfCall {
                account: call.account.into(),
            }
        }
    }

    impl From<approveCall> for Erc20::approveCall {
        fn from(call: approveCall) -> Self {
            Erc20::approveCall {
                spender: call.spender.into(),
                amount: call.amount,
            }
        }
    }

    impl From<allowanceCall> for Erc20::allowanceCall {
        fn from(call: allowanceCall) -> Self {
            Erc20::allowanceCall {
                owner: call.owner.into(),
                spender: call.spender.into(),
            }
        }
    }

    impl From<transferFromCall> for Erc20::transferFromCall {
        fn from(call: transferFromCall) -> Self {
            Erc20::transferFromCall {
                sender: call.sender.into(),
                recipient: call.recipient.into(),
                amount: call.amount,
            }
        }
    }

    #[macro_export]
    macro_rules! impl_abi_encode_decode {
        ($struct_name:ident, $sol_type:path) => {
            impl AbiEncode for $struct_name {
                fn encode(self) -> Vec<u8> {
                    let sol_type: $sol_type = self.into(); // Convert to the corresponding SolCall type
                    sol_type.abi_encode() // Use the `abi_encode` method from SolCall
                }
            }

            impl AbiDecode for $struct_name {
                type Error = String; // Define the error type

                fn decode(data: &[u8]) -> Result<Self, Self::Error> {
                    // Use the `abi_decode` method from SolCall to decode the data
                    <$sol_type>::abi_decode(data)
                        .map(|decoded| decoded.into()) // Convert the decoded value to the struct
                        .map_err(|e| format!("Failed to decode: {}", e))
                }
            }
        };
    }

    impl_abi_encode_decode!(transferCall, Erc20::transferCall);
    impl_abi_encode_decode!(balanceOfCall, Erc20::balanceOfCall);
    impl_abi_encode_decode!(approveCall, Erc20::approveCall);
    impl_abi_encode_decode!(allowanceCall, Erc20::allowanceCall);
    impl_abi_encode_decode!(transferFromCall, Erc20::transferFromCall);
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
        Trc20::transferCall {
            recipient,
            amount: U256::from(amount),
        }
    }

    /// Check the token balance of a given account
    pub fn balance_of(&self, account: TronAddress) -> Trc20::balanceOfCall {
        Trc20::balanceOfCall { account }
    }

    /// Allow spender to withdraw from the sender's account multiple times
    pub fn approve(
        &self,
        spender: TronAddress,
        amount: u64,
    ) -> Trc20::approveCall {
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
        Trc20::allowanceCall { owner, spender }
    }

    /// Transfer tokens from one account to another, using the allowance
    pub fn transfer_from(
        &self,
        sender: TronAddress,
        recipient: TronAddress,
        amount: u64,
    ) -> Trc20::transferFromCall {
        Trc20::transferFromCall {
            sender,
            recipient,
            amount: U256::from(amount),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Trc20Call {
    BalanceOf(Trc20::balanceOfCall),
    Transfer(Trc20::transferCall),
}

impl TryFromData for Trc20Call {
    type Error = String;
    fn try_from_data(data: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(call) = Trc20::transferCall::decode(data) {
            Ok(Trc20Call::Transfer(call))
        } else if let Ok(call) = Trc20::balanceOfCall::decode(data) {
            Ok(Trc20Call::BalanceOf(call))
        } else {
            Err("unknown call".into())
        }
    }
}

impl AddressExtractor<TriggerSmartContract> for Trc20Call {
    fn extract(from: TriggerSmartContract) -> Option<TronAddress> {
        let Ok(call) = Trc20Call::try_from_data(&Vec::<u8>::from(from.data))
        else {
            return None;
        };
        match call {
            Trc20Call::Transfer(transfer_call) => Some(transfer_call.recipient),
            _ => None,
        }
    }
}
