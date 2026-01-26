use std::marker::PhantomData;

use alloy_primitives::U256;
use eyre::eyre;

use crate::{
    client::{Client, pending::PendingTransaction},
    contracts::{AbiDecode, AbiEncode, token::Token},
    domain::{
        Message,
        account::AccountStatus,
        address::TronAddress,
        contract::{Contract, TriggerSmartContract},
        transaction::Transaction,
        trx::Trx,
    },
    error::Error,
    provider::TronProvider,
    signer::PrehashSigner,
    trx,
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

    macro_rules! generate_trc20_structs {
        // Handle non-generic structs
        ($($name:ident { $($field:ident: $ty:ty),+ }),+) => {
            $(
                #[derive(Clone, Debug)]
                pub struct $name {
                    $(pub $field: $ty),+
                }
            )+
        };
        // Handle generic structs
        ($($name:ident<$generic:ident> { $($field:ident: $ty:ty),+ }),+) => {
            $(
                #[derive(Clone, Debug)]
                pub struct $name<$generic> {
                    $(pub $field: $ty),+
                }
            )+
        };
        // Mixed case (both generic and non-generic)
        ($(
            $name:ident $(<$generic:ident>)? { $($field:ident: $ty:ty),+ }
        ),+) => {
            $(
                #[derive(Clone, Debug)]
                pub struct $name $(<$generic>)? {
                    $(pub $field: $ty),+
                }
            )+
        };
    }

    // Usage with generic parameters
    generate_trc20_structs! {
        transferCall<T> { recipient: TronAddress, amount: T },
        balanceOfCall { account: TronAddress },
        approveCall<T> { spender: TronAddress, amount: T },
        allowanceCall { owner: TronAddress, spender: TronAddress },
        transferFromCall<T> { sender: TronAddress, recipient: TronAddress, amount: T }
    }

    // Macro for non-generic types
    macro_rules! impl_from_erc20 {
        ($trc20_type:ident, $erc20_type:path { $($field:ident),+ }) => {
            impl From<$erc20_type> for $trc20_type {
                fn from(call: $erc20_type) -> Self {
                    $trc20_type {
                        $($field: call.$field.into()),+
                    }
                }
            }
        };
    }

    // Macro for non-generic types (reverse direction)
    macro_rules! impl_from_trc20 {
        ($trc20_type:ident, $erc20_type:path { $($field:ident),+ }) => {
            impl From<$trc20_type> for $erc20_type {
                fn from(call: $trc20_type) -> Self {
                    $erc20_type {
                        $($field: call.$field.into()),+
                    }
                }
            }
        };
    }

    // Macro for generic types (ERC20 → TRC20)
    macro_rules! impl_from_erc20_generic {
        ($trc20_type:ident<$generic:ident>, $erc20_type:path { $($field:ident),+ }) => {
            impl<$generic: From<U256>> From<$erc20_type> for $trc20_type<$generic> {
                fn from(call: $erc20_type) -> Self {
                    $trc20_type {
                        $($field: call.$field.into()),+
                    }
                }
            }
        };
    }

    // Macro for generic types (TRC20 → ERC20)
    macro_rules! impl_from_trc20_generic {
        ($trc20_type:ident<$generic:ident>, $erc20_type:path { $($field:ident),+ }) => {
            impl<$generic: Into<U256>> From<$trc20_type<$generic>> for $erc20_type {
                fn from(call: $trc20_type<$generic>) -> Self {
                    $erc20_type {
                        $($field: call.$field.into()),+
                    }
                }
            }
        };
    }

    // Implement conversions for non-generic structs
    impl_from_erc20!(balanceOfCall, Erc20::balanceOfCall { account });
    impl_from_erc20!(allowanceCall, Erc20::allowanceCall { owner, spender });
    impl_from_trc20!(balanceOfCall, Erc20::balanceOfCall { account });
    impl_from_trc20!(allowanceCall, Erc20::allowanceCall { owner, spender });

    // Implement conversions for generic structs
    impl_from_erc20_generic!(
        transferCall<T>,
        Erc20::transferCall { recipient, amount }
    );
    impl_from_erc20_generic!(
        approveCall<T>,
        Erc20::approveCall { spender, amount }
    );
    impl_from_erc20_generic!(
        transferFromCall<T>,
        Erc20::transferFromCall {
            sender,
            recipient,
            amount
        }
    );

    impl_from_trc20_generic!(
        transferCall<T>,
        Erc20::transferCall { recipient, amount }
    );
    impl_from_trc20_generic!(
        approveCall<T>,
        Erc20::approveCall { spender, amount }
    );
    impl_from_trc20_generic!(
        transferFromCall<T>,
        Erc20::transferFromCall {
            sender,
            recipient,
            amount
        }
    );

    #[macro_export]
    macro_rules! impl_abi_encode_decode_new {
        // For non-generic types
        ($struct_name:ident, $sol_type:path) => {
            impl AbiEncode for $struct_name {
                fn encode(self) -> Vec<u8> {
                    let sol_type: $sol_type = self.into();
                    sol_type.abi_encode()
                }
            }

            impl AbiDecode for $struct_name {
                type Error = String;

                fn decode(data: &[u8]) -> Result<Self, Self::Error> {
                    <$sol_type>::abi_decode(data)
                        .map(|decoded| decoded.into())
                        .map_err(|e| format!("Failed to decode: {}", e))
                }
            }
        };

        // For generic types
        ($struct_name:ident<$generic:ident>, $sol_type:path) => {
            impl<$generic: Into<U256> + From<U256>> AbiEncode
                for $struct_name<$generic>
            {
                fn encode(self) -> Vec<u8> {
                    let sol_type: $sol_type = self.into();
                    sol_type.abi_encode()
                }
            }

            impl<$generic: Into<U256> + From<U256>> AbiDecode
                for $struct_name<$generic>
            {
                type Error = String;

                fn decode(data: &[u8]) -> Result<Self, Self::Error> {
                    <$sol_type>::abi_decode(data)
                        .map(|decoded| decoded.into())
                        .map_err(|e| format!("Failed to decode: {}", e))
                }
            }
        };
    }

    impl_abi_encode_decode_new!(transferCall<T>, Erc20::transferCall);
    impl_abi_encode_decode_new!(balanceOfCall, Erc20::balanceOfCall);
    impl_abi_encode_decode_new!(approveCall<T>, Erc20::approveCall);
    impl_abi_encode_decode_new!(allowanceCall, Erc20::allowanceCall);
    impl_abi_encode_decode_new!(transferFromCall<T>, Erc20::transferFromCall);
}

#[derive(Debug, Clone)]
pub struct Trc20Contract<T> {
    contract_address: TronAddress,
    _token: PhantomData<T>,
}

impl<T: Token> Trc20Contract<T> {
    pub fn new(contract_address: TronAddress) -> Self {
        Trc20Contract {
            contract_address,
            _token: Default::default(),
        }
    }

    pub fn address(&self) -> TronAddress {
        self.contract_address
    }

    /// Send amount of tokens from sender to recipient
    pub fn transfer(
        &self,
        recipient: TronAddress,
        amount: T,
    ) -> Trc20::transferCall<T> {
        Trc20::transferCall { recipient, amount }
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
    ) -> Trc20::approveCall<T> {
        Trc20::approveCall {
            spender,
            amount: U256::from(amount).into(),
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
    ) -> Trc20::transferFromCall<T> {
        Trc20::transferFromCall {
            sender,
            recipient,
            amount: U256::from(amount).into(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Trc20Call<T> {
    BalanceOf(Trc20::balanceOfCall),
    Transfer(Trc20::transferCall<T>),
}

impl<T: Token> TryFromData for Trc20Call<T> {
    type Error = String;
    fn try_from_data(data: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(call) = Trc20::transferCall::<T>::decode(data) {
            Ok(Trc20Call::Transfer(call))
        } else if let Ok(call) = Trc20::balanceOfCall::decode(data) {
            Ok(Trc20Call::BalanceOf(call))
        } else {
            Err("unknown call".into())
        }
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Trc20Transfer<'a, P, S, T> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: Trc20Contract<T>,
    pub(super) to: TronAddress,
    pub(super) amount: T,
    pub(super) owner: Option<TronAddress>,
    pub(super) memo: Option<Message>,
    pub(super) call_value: Option<Trx>,
    pub(super) call_token_value: Option<Trx>,
    pub(super) token_id: Option<i64>,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, T, State: trc20_transfer_builder::IsComplete>
    Trc20TransferBuilder<'a, P, S, T, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
    T: Token,
{
    pub async fn build<M>(
        self,
    ) -> crate::Result<PendingTransaction<'a, P, S, M>> {
        let transfer = self.build_internal();
        let owner = transfer
            .owner
            .or_else(|| transfer.client.signer_address())
            .ok_or_else(|| Error::Unexpected(eyre!("missing owner address")))?;
        let call = transfer.contract.transfer(transfer.to, transfer.amount);
        let contract_address = transfer.contract.address();

        // Check balance
        {
            let balance = Trc20BalanceOf::with_client(transfer.client)
                .contract(transfer.contract)
                .owner(owner)
                .get()
                .await?;
            if transfer.amount > balance {
                return Err(Error::InsufficientTokenBalance {
                    balance: balance.into(),
                    need: transfer.amount.into(),
                    token: T::symbol(),
                });
            }
        }

        let latest_block = transfer.client.provider.get_now_block().await?;
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::TriggerSmartContract(
                        TriggerSmartContract {
                            owner_address: owner,
                            contract_address,
                            call_value: transfer.call_value.unwrap_or_default(),
                            data: call.encode().into(),
                            call_token_value: transfer
                                .call_token_value
                                .unwrap_or_default(),
                            token_id: transfer.token_id.unwrap_or_default(),
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            transfer.memo.unwrap_or_default(),
        );
        let check = transfer.client.check_account(transfer.to).await?;
        let additional_fee = if matches!(check, AccountStatus::NotExists) {
            trx!(0.1 TRX)
        } else {
            Trx::ZERO
        };
        PendingTransaction::new(
            transfer.client,
            transaction,
            owner,
            additional_fee,
            transfer.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Trc20BalanceOf<'a, P, S, T> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: Trc20Contract<T>,
    pub(super) owner: Option<TronAddress>,
}

impl<'a, P, S, T, State: trc20_balance_of_builder::IsComplete>
    Trc20BalanceOfBuilder<'a, P, S, T, State>
where
    P: TronProvider,
    S: PrehashSigner,
    T: Token,
{
    pub async fn get(self) -> crate::Result<T> {
        let balance_of = self.build_internal();
        let owner = balance_of
            .owner
            .or_else(|| balance_of.client.signer_address())
            .ok_or_else(|| {
                Error::Unexpected(eyre!(
                    "missing address to check trc20 balance for"
                ))
            })?;

        let call = balance_of.contract.balance_of(owner);
        let trigger = TriggerSmartContract {
            owner_address: owner,
            contract_address: balance_of.contract.address(),
            data: call.encode().into(),
            ..Default::default()
        };
        let mut extention = balance_of
            .client
            .provider()
            .trigger_constant_contract(trigger)
            .await?;
        let balance = if let Some(result) = extention.constant_result.pop() {
            if result.len() == 32 {
                let balance_bytes: [u8; 32] = result.try_into().unwrap(); // We sure in length
                alloy_primitives::U256::from_be_bytes(balance_bytes)
            } else {
                return Err(eyre!("unexpected constant result length").into());
            }
        } else {
            return Err(eyre!("no constant result returned",).into());
        };

        Ok(T::from(balance))
    }
}

pub trait Trc20Calls<P, S, T> {
    fn trc20_balance_of(&self) -> Trc20BalanceOfBuilder<'_, P, S, T>;
    fn trc20_transfer(&self) -> Trc20TransferBuilder<'_, P, S, T>;
}

impl<P, S, T> Trc20Calls<P, S, T> for Client<P, S> {
    fn trc20_balance_of(&self) -> Trc20BalanceOfBuilder<'_, P, S, T> {
        Trc20BalanceOf::with_client(self)
    }

    fn trc20_transfer(&self) -> Trc20TransferBuilder<'_, P, S, T> {
        Trc20Transfer::with_client(self)
    }
}
