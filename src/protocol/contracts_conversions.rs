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
            name: value.name.into(),
            abbr: value.abbr.into(),
            total_supply: value.total_supply,
            frozen_supply: value
                .frozen_supply
                .into_iter()
                .map(Into::into)
                .collect(),
            trx_num: (value.trx_num as i64).into(),
            precision: value.precision,
            num: value.num,
            start_time: time_unix_millis(value.start_time),
            end_time: time_unix_millis(value.end_time),
            order: value.order,
            vote_score: value.vote_score,
            description: value.description.into(),
            url: value.url.into(),
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: time_unix_millis(
                value.public_latest_free_net_time,
            ),
        }
    }
}

impl From<TransferAssetContract> for domain::contract::TransferAssetContract {
    fn from(value: TransferAssetContract) -> Self {
        domain::contract::TransferAssetContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            to_address: value.to_address.as_slice().try_into().unwrap(),
            amount: value.amount.into(),
            asset_name: value.asset_name.into(),
        }
    }
}

impl From<UnfreezeAssetContract> for domain::contract::UnfreezeAssetContract {
    fn from(value: UnfreezeAssetContract) -> Self {
        domain::contract::UnfreezeAssetContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<UpdateAssetContract> for domain::contract::UpdateAssetContract {
    fn from(value: UpdateAssetContract) -> Self {
        domain::contract::UpdateAssetContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            description: value.description.into(),
            url: value.url.into(),
            new_limit: value.new_limit,
            new_public_limit: value.new_public_limit,
        }
    }
}

impl From<ParticipateAssetIssueContract>
    for domain::contract::ParticipateAssetIssueContract
{
    fn from(value: ParticipateAssetIssueContract) -> Self {
        domain::contract::ParticipateAssetIssueContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            to_address: value.to_address.as_slice().try_into().unwrap(),
            asset_name: value.asset_name.into(),
            amount: value.amount.into(),
        }
    }
}

impl From<AccountCreateContract> for domain::contract::AccountCreateContract {
    fn from(value: AccountCreateContract) -> Self {
        domain::contract::AccountCreateContract {
            account_type: value.r#type().into(),
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            account_address: value
                .account_address
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<AccountUpdateContract> for domain::contract::AccountUpdateContract {
    fn from(value: AccountUpdateContract) -> Self {
        domain::contract::AccountUpdateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            account_name: value.account_name.into(),
        }
    }
}

impl From<SetAccountIdContract> for domain::contract::SetAccountIdContract {
    fn from(value: SetAccountIdContract) -> Self {
        domain::contract::SetAccountIdContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            account_id: value.account_id.into(),
        }
    }
}

impl From<AccountPermissionUpdateContract>
    for domain::contract::AccountPermissionUpdateContract
{
    fn from(value: AccountPermissionUpdateContract) -> Self {
        domain::contract::AccountPermissionUpdateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            owner: value.owner.map(Into::into),
            witness: value.witness.map(Into::into),
            actives: value.actives.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<WitnessCreateContract> for domain::contract::WitnessCreateContract {
    fn from(value: WitnessCreateContract) -> Self {
        domain::contract::WitnessCreateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            url: value.url.into(),
        }
    }
}

impl From<WitnessUpdateContract> for domain::contract::WitnessUpdateContract {
    fn from(value: WitnessUpdateContract) -> Self {
        domain::contract::WitnessUpdateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            update_url: value.update_url.into(),
        }
    }
}

impl From<vote_witness_contract::Vote> for domain::contract::Vote {
    fn from(value: vote_witness_contract::Vote) -> Self {
        domain::contract::Vote {
            vote_address: value.vote_address.as_slice().try_into().unwrap(),
            vote_count: value.vote_count,
        }
    }
}

impl From<VoteWitnessContract> for domain::contract::VoteWitnessContract {
    fn from(value: VoteWitnessContract) -> Self {
        domain::contract::VoteWitnessContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            votes: value.votes.into_iter().map(Into::into).collect(),
            support: value.support,
        }
    }
}

impl From<FreezeBalanceContract> for domain::contract::FreezeBalanceContract {
    fn from(value: FreezeBalanceContract) -> Self {
        domain::contract::FreezeBalanceContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            frozen_balance: value.frozen_balance.into(),
            frozen_duration: time::Duration::days(value.frozen_duration),
            resource: value.resource().into(),
            receiver_address: value
                .receiver_address
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<UnfreezeBalanceContract>
    for domain::contract::UnfreezeBalanceContract
{
    fn from(value: UnfreezeBalanceContract) -> Self {
        domain::contract::UnfreezeBalanceContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            resource: value.resource().into(),
            receiver_address: value
                .receiver_address
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<WithdrawBalanceContract>
    for domain::contract::WithdrawBalanceContract
{
    fn from(value: WithdrawBalanceContract) -> Self {
        domain::contract::WithdrawBalanceContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
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

impl From<TransactionBalanceTrace>
    for domain::contract::TransactionBalanceTrace
{
    fn from(value: TransactionBalanceTrace) -> Self {
        domain::contract::TransactionBalanceTrace {
            transaction_identifier: todo!(),
            operation: value.operation.into_iter().map(Into::into).collect(),
            r#type: value.r#type,
            status: value.status,
        }
    }
}

impl From<transaction_balance_trace::Operation>
    for domain::contract::Operation
{
    fn from(value: transaction_balance_trace::Operation) -> Self {
        domain::contract::Operation {
            amount: value.amount.into(),
            operation_identifier: value.operation_identifier,
            address: value.address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<BlockBalanceTrace> for domain::contract::BlockBalanceTrace {
    fn from(value: BlockBalanceTrace) -> Self {
        domain::contract::BlockBalanceTrace {
            block_identifier: value.block_identifier.unwrap_or_default().into(),
            timestamp: time_unix_millis(value.timestamp),
            transaction_balance_trace: value
                .transaction_balance_trace
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<block_balance_trace::BlockIdentifier>
    for domain::contract::BlockIdentifier
{
    fn from(value: block_balance_trace::BlockIdentifier) -> Self {
        domain::contract::BlockIdentifier {
            hash: value.hash.into(),
            number: value.number,
        }
    }
}

impl From<AccountTrace> for domain::contract::AccountTrace {
    fn from(value: AccountTrace) -> Self {
        domain::contract::AccountTrace {
            balance: value.balance.into(),
            placeholder: value.placeholder.into(),
        }
    }
}

impl From<AccountIdentifier> for domain::contract::AccountIdentifier {
    fn from(value: AccountIdentifier) -> Self {
        domain::contract::AccountIdentifier {
            address: value.address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<AccountBalanceRequest> for domain::contract::AccountBalanceRequest {
    fn from(value: AccountBalanceRequest) -> Self {
        domain::contract::AccountBalanceRequest {
            account_identifier: value
                .account_identifier
                .unwrap_or_default()
                .into(),
            block_identifier: value.block_identifier.unwrap_or_default().into(),
        }
    }
}

impl From<AccountBalanceResponse> for domain::contract::AccountBalanceResponse {
    fn from(value: AccountBalanceResponse) -> Self {
        domain::contract::AccountBalanceResponse {
            balance: value.balance.into(),
            block_identifier: value.block_identifier.unwrap_or_default().into(),
        }
    }
}

impl From<FreezeBalanceV2Contract>
    for domain::contract::FreezeBalanceV2Contract
{
    fn from(value: FreezeBalanceV2Contract) -> Self {
        domain::contract::FreezeBalanceV2Contract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            frozen_balance: value.frozen_balance.into(),
            resource: value.resource().into(),
        }
    }
}

impl From<UnfreezeBalanceV2Contract>
    for domain::contract::UnfreezeBalanceV2Contract
{
    fn from(value: UnfreezeBalanceV2Contract) -> Self {
        domain::contract::UnfreezeBalanceV2Contract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            resource: value.resource().into(),
            unfreeze_balance: value.unfreeze_balance.into(),
        }
    }
}

impl From<WithdrawExpireUnfreezeContract>
    for domain::contract::WithdrawExpireUnfreezeContract
{
    fn from(value: WithdrawExpireUnfreezeContract) -> Self {
        domain::contract::WithdrawExpireUnfreezeContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
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

impl From<CancelAllUnfreezeV2Contract>
    for domain::contract::CancelAllUnfreezeV2Contract
{
    fn from(value: CancelAllUnfreezeV2Contract) -> Self {
        domain::contract::CancelAllUnfreezeV2Contract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<ProposalApproveContract>
    for domain::contract::ProposalApproveContract
{
    fn from(value: ProposalApproveContract) -> Self {
        domain::contract::ProposalApproveContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            proposal_id: value.proposal_id,
            is_add_approval: value.is_add_approval,
        }
    }
}

impl From<ProposalCreateContract> for domain::contract::ProposalCreateContract {
    fn from(value: ProposalCreateContract) -> Self {
        domain::contract::ProposalCreateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            parameters: value.parameters,
        }
    }
}

impl From<ProposalDeleteContract> for domain::contract::ProposalDeleteContract {
    fn from(value: ProposalDeleteContract) -> Self {
        domain::contract::ProposalDeleteContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            proposal_id: value.proposal_id,
        }
    }
}

impl From<BuyStorageContract> for domain::contract::BuyStorageContract {
    fn from(value: BuyStorageContract) -> Self {
        domain::contract::BuyStorageContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            quant: value.quant.into(),
        }
    }
}

impl From<SellStorageContract> for domain::contract::SellStorageContract {
    fn from(value: SellStorageContract) -> Self {
        domain::contract::SellStorageContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            storage_bytes: value.storage_bytes,
        }
    }
}

impl From<UpdateBrokerageContract>
    for domain::contract::UpdateBrokerageContract
{
    fn from(value: UpdateBrokerageContract) -> Self {
        domain::contract::UpdateBrokerageContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            brokerage: value.brokerage,
        }
    }
}

impl From<ExchangeCreateContract> for domain::contract::ExchangeCreateContract {
    fn from(value: ExchangeCreateContract) -> Self {
        domain::contract::ExchangeCreateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            first_token_id: value.first_token_id.into(),
            first_token_balance: value.first_token_balance,
            second_token_id: value.second_token_id.into(),
            second_token_balance: value.second_token_balance,
        }
    }
}

impl From<ExchangeInjectContract> for domain::contract::ExchangeInjectContract {
    fn from(value: ExchangeInjectContract) -> Self {
        domain::contract::ExchangeInjectContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        }
    }
}

impl From<ExchangeWithdrawContract>
    for domain::contract::ExchangeWithdrawContract
{
    fn from(value: ExchangeWithdrawContract) -> Self {
        domain::contract::ExchangeWithdrawContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        }
    }
}

impl From<ExchangeTransactionContract>
    for domain::contract::ExchangeTransactionContract
{
    fn from(value: ExchangeTransactionContract) -> Self {
        domain::contract::ExchangeTransactionContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
            expected: value.expected,
        }
    }
}

impl From<MarketSellAssetContract>
    for domain::contract::MarketSellAssetContract
{
    fn from(value: MarketSellAssetContract) -> Self {
        domain::contract::MarketSellAssetContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            sell_token_id: value.sell_token_id.into(),
            sell_token_quantity: value.sell_token_quantity,
            buy_token_id: value.buy_token_id.into(),
            buy_token_quantity: value.buy_token_quantity,
        }
    }
}

impl From<MarketCancelOrderContract>
    for domain::contract::MarketCancelOrderContract
{
    fn from(value: MarketCancelOrderContract) -> Self {
        domain::contract::MarketCancelOrderContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            order_id: value.order_id.into(),
        }
    }
}

macro_rules! impl_contract_conversion {
    ($($variant:ident),+ $(,)?) => {
        impl From<transaction::Contract> for domain::contract::Contract {
            fn from(c: transaction::Contract) -> Self {
                use transaction::contract::ContractType;

                let contract_type = match c.r#type() {
                    $(
                        ContractType::$variant => {
                            let decoded: $variant = prost::Message::decode(
                                &c.parameter.unwrap_or_default().value[..]
                            ).unwrap_or_default();
                            domain::contract::ContractType::$variant(decoded.into())
                        }
                    )+
                    _ => panic!("Unsupported contract type: {:?}", c.r#type()),
                };

                domain::contract::Contract {
                    contract_type,
                    provider: c.provider,
                    contract_name: c.contract_name.into(),
                    permission_id: c.permission_id,
                }
            }
        }
    };
}

impl_contract_conversion!(
    AccountCreateContract,
    TransferContract,
    TransferAssetContract,
    VoteWitnessContract,
    WitnessCreateContract,
    AssetIssueContract,
    WitnessUpdateContract,
    ParticipateAssetIssueContract,
    AccountUpdateContract,
    FreezeBalanceContract,
    UnfreezeBalanceContract,
    WithdrawBalanceContract,
    UnfreezeAssetContract,
    UpdateAssetContract,
    ProposalCreateContract,
    ProposalApproveContract,
    ProposalDeleteContract,
    // SetAccountIdContract,
    // CreateSmartContract,
    TriggerSmartContract,
    // UpdateSettingContract,
    ExchangeCreateContract,
    ExchangeInjectContract,
    ExchangeWithdrawContract,
    ExchangeTransactionContract,
    // UpdateEnergyLimitContract,
    AccountPermissionUpdateContract,
    // ClearAbiContract,
    UpdateBrokerageContract,
    // ShieldedTransferContract,
    MarketSellAssetContract,
    MarketCancelOrderContract,
    FreezeBalanceV2Contract,
    UnfreezeBalanceV2Contract,
    WithdrawExpireUnfreezeContract,
    DelegateResourceContract,
    UnDelegateResourceContract,
    CancelAllUnfreezeV2Contract
);
