use crate::domain;
use crate::domain::address::TronAddress;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AddressType {
    Owner,
    To,
    Contract,
}

impl super::Contract {
    fn get_address(&self, address_type: AddressType) -> Option<TronAddress> {
        match &self.contract_type {
            domain::contract::ContractType::AccountCreateContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.account_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::AccountUpdateContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::TransferContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.to_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::TransferAssetContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.to_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::TriggerSmartContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.contract_address),
                    AddressType::Contract => Some(contract.contract_address),
                }
            }
            domain::contract::ContractType::AssetIssueContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ParticipateAssetIssueContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.to_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::AccountPermissionUpdateContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::FreezeBalanceContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.receiver_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::UnfreezeBalanceContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.receiver_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::DelegateResourceContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.receiver_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::UnDelegateResourceContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.receiver_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ExchangeCreateContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ExchangeInjectContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ExchangeWithdrawContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ExchangeTransactionContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::MarketSellAssetContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None, // No 'to' address here
                    AddressType::Contract => None, // No contract address here
                }
            }
            domain::contract::ContractType::ShieldedTransferContract(
                contract,
            ) => {
                match address_type {
                    AddressType::Owner => {
                        Some(contract.transparent_from_address)
                    }
                    AddressType::To => Some(contract.transparent_to_address),
                    AddressType::Contract => None, // No contract address here
                }
            }
            _ => None, // Return None if the contract doesn't match any case
        }
    }

    pub fn owner_address(&self) -> Option<TronAddress> {
        self.get_address(AddressType::Owner)
    }

    pub fn contract_address(&self) -> Option<TronAddress> {
        self.get_address(AddressType::Contract)
    }

    pub fn to_address(&self) -> Option<TronAddress> {
        self.get_address(AddressType::To)
    }
}
