use crate::domain;
use crate::domain::address::TronAddress;
use crate::domain::contract::TriggerSmartContract;

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
                    AddressType::Contract => None,
                }
            }
            domain::contract::ContractType::AccountUpdateContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => None,
                    AddressType::Contract => None,
                }
            }
            domain::contract::ContractType::TransferContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.to_address),
                    AddressType::Contract => None,
                }
            }
            domain::contract::ContractType::TransferAssetContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.to_address),
                    AddressType::Contract => None,
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
                    AddressType::To => None,
                    AddressType::Contract => None,
                }
            }
            domain::contract::ContractType::ParticipateAssetIssueContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => Some(contract.to_address),
                AddressType::Contract => None,
            },
            domain::contract::ContractType::AccountPermissionUpdateContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::FreezeBalanceContract(contract) => {
                match address_type {
                    AddressType::Owner => Some(contract.owner_address),
                    AddressType::To => Some(contract.receiver_address),
                    AddressType::Contract => None,
                }
            }
            domain::contract::ContractType::UnfreezeBalanceContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => Some(contract.receiver_address),
                AddressType::Contract => None,
            },
            domain::contract::ContractType::FreezeBalanceV2Contract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::UnfreezeBalanceV2Contract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::DelegateResourceContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => Some(contract.receiver_address),
                AddressType::Contract => None,
            },
            domain::contract::ContractType::UnDelegateResourceContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => Some(contract.receiver_address),
                AddressType::Contract => None,
            },
            domain::contract::ContractType::ExchangeCreateContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::ExchangeInjectContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::ExchangeWithdrawContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::ExchangeTransactionContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::MarketSellAssetContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.owner_address),
                AddressType::To => None,
                AddressType::Contract => None,
            },
            domain::contract::ContractType::ShieldedTransferContract(
                contract,
            ) => match address_type {
                AddressType::Owner => Some(contract.transparent_from_address),
                AddressType::To => Some(contract.transparent_to_address),
                AddressType::Contract => None,
            },
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

    pub fn trigger_smart_contract(&self) -> Option<TriggerSmartContract> {
        match self.contract_type {
            domain::contract::ContractType::TriggerSmartContract(
                ref contract,
            ) => Some(contract.clone()),
            _ => None,
        }
    }
}
