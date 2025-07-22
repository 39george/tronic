use super::*;

impl From<TriggerSmartContract> for domain::contract::TriggerSmartContract {
    fn from(value: TriggerSmartContract) -> Self {
        domain::contract::TriggerSmartContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap(),
            call_value: value.call_value.into(),
            data: value.data.into(),
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        }
    }
}

impl From<domain::contract::TriggerSmartContract> for TriggerSmartContract {
    fn from(value: domain::contract::TriggerSmartContract) -> Self {
        TriggerSmartContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value.contract_address.as_bytes().to_vec(),
            call_value: value.call_value.into(),
            data: value.data.as_bytes().to_vec(),
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        }
    }
}

impl From<asset_issue_contract::FrozenSupply>
    for domain::contract::FrozenSupply
{
    fn from(value: asset_issue_contract::FrozenSupply) -> Self {
        domain::contract::FrozenSupply {
            frozen_amount: value.frozen_amount.into(),
            frozen_days: value.frozen_days,
        }
    }
}

impl From<AssetIssueContract> for domain::contract::AssetIssueContract {
    fn from(value: AssetIssueContract) -> Self {
        domain::contract::AssetIssueContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            id: value.id,
            name: value.name,
            abbr: value.abbr,
            total_supply: value.total_supply,
            frozen_supply: value
                .frozen_supply
                .into_iter()
                .map(Into::into)
                .collect(),
            trx_num: value.trx_num,
            precision: value.precision,
            num: value.num,
            start_time: value.start_time,
            end_time: value.end_time,
            order: value.order,
            vote_score: value.vote_score,
            description: value.description,
            url: value.url,
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: value.public_latest_free_net_time,
        }
    }
}

impl From<TransferAssetContract> for domain::contract::TransferAssetContract {
    fn from(value: TransferAssetContract) -> Self {
        domain::contract::TransferAssetContract {
            owner_address: value.owner_address.into(),
            to_address: value.to_address.into(),
            amount: value.amount.into(),
            asset_name: value.asset_name.into(),
        }
    }
}

impl From<TransferContract> for domain::contract::TransferContract {
    fn from(value: TransferContract) -> Self {
        domain::contract::TransferContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            to_address: value.to_address.as_slice().try_into().unwrap(),
            amount: value.amount.into(),
        }
    }
}

impl From<ResourceCode> for domain::contract::ResourceCode {
    fn from(value: ResourceCode) -> Self {
        match value {
            ResourceCode::Bandwidth => {
                domain::contract::ResourceCode::Bandwidth
            }
            ResourceCode::Energy => domain::contract::ResourceCode::Energy,
            ResourceCode::TronPower => {
                domain::contract::ResourceCode::TronPower
            }
        }
    }
}

impl From<DelegateResourceContract>
    for domain::contract::DelegateResourceContract
{
    fn from(value: DelegateResourceContract) -> Self {
        domain::contract::DelegateResourceContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            resource: value.resource().into(),
            balance: value.balance.into(),
            receiver_address: value
                .receiver_address
                .as_slice()
                .try_into()
                .unwrap(),
            lock: value.lock,
            lock_period: value.lock_period,
        }
    }
}

impl From<UnDelegateResourceContract>
    for domain::contract::UnDelegateResourceContract
{
    fn from(value: UnDelegateResourceContract) -> Self {
        domain::contract::UnDelegateResourceContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            resource: value.resource().into(),
            balance: value.balance.into(),
            receiver_address: value
                .receiver_address
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<transaction::Contract> for domain::contract::Contract {
    fn from(c: transaction::Contract) -> Self {
        // Here, we determine the contract type based on the "type" field
        let contract_type = match c.r#type() {
            transaction::contract::ContractType::TransferContract => {
                // Decode TransferContract parameter
                let transfer_contract: TransferContract =
                    prost::Message::decode(
                        &c.parameter.unwrap_or_default().value[..],
                    )
                    .unwrap_or_default();
                domain::contract::ContractType::TransferContract(
                    transfer_contract.into(),
                )
            }
            transaction::contract::ContractType::TriggerSmartContract => {
                // Decode TransferContract parameter
                let trigger_smart_contract: TriggerSmartContract =
                    prost::Message::decode(
                        &c.parameter.unwrap_or_default().value[..],
                    )
                    .unwrap_or_default();
                if let Ok(d) = crate::contracts::decode_transfer_call(
                    &trigger_smart_contract.data,
                ) {
                    println!(
                        "FOUND transfer: {:?}, trona: {}",
                        d,
                        TronAddress::from(d.recipient)
                    );
                }
                if let Ok(d) = crate::contracts::decode_balance_of_call(
                    &trigger_smart_contract.data,
                ) {
                    println!(
                        "FOUND balance: {:?}, trona: {}",
                        d,
                        TronAddress::from(d.account)
                    );
                }
                let tsc = domain::contract::ContractType::TriggerSmartContract(
                    trigger_smart_contract.into(),
                );
                tsc
            }
            transaction::contract::ContractType::DelegateResourceContract => {
                // Decode TransferContract parameter
                let delegate_resource_contract: DelegateResourceContract =
                    prost::Message::decode(
                        &c.parameter.unwrap_or_default().value[..],
                    )
                    .unwrap_or_default();
                domain::contract::ContractType::DelegateResourceContract(
                    delegate_resource_contract.into(),
                )
            }
            transaction::contract::ContractType::UnDelegateResourceContract => {
                // Decode TransferContract parameter
                let delegate_resource_contract: UnDelegateResourceContract =
                    prost::Message::decode(
                        &c.parameter.unwrap_or_default().value[..],
                    )
                    .unwrap_or_default();
                domain::contract::ContractType::UnDelegateResourceContract(
                    delegate_resource_contract.into(),
                )
            }
            transaction::contract::ContractType::TransferAssetContract => {
                // Decode TransferContract parameter
                let tac: TransferAssetContract = prost::Message::decode(
                    &c.parameter.unwrap_or_default().value[..],
                )
                .unwrap_or_default();
                domain::contract::ContractType::TransferAssetContract(
                    tac.into(),
                )
            }
            _ => {
                println!("{:?}", c);
                panic!()
            }
        };

        domain::contract::Contract {
            contract_type,
            provider: c.provider,
            contract_name: c.contract_name.into(),
            permission_id: c.permission_id,
        }
    }
}
