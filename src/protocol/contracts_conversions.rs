use super::*;

impl TryFrom<TriggerSmartContract> for domain::contract::TriggerSmartContract {
    type Error = ProtoConvError;
    fn try_from(value: TriggerSmartContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::TriggerSmartContract {
            owner_address: addr(
                &value.owner_address,
                "TriggerSmartContract.owner_address",
            )?,
            contract_address: addr(
                &value.contract_address,
                "TriggerSmartContract.contract_address",
            )?,
            call_value: value.call_value.into(),
            data: value.data.into(),
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        })
    }
}

impl From<domain::contract::TriggerSmartContract> for TriggerSmartContract {
    fn from(value: domain::contract::TriggerSmartContract) -> Self {
        TriggerSmartContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: if value.contract_address.eq(&TronAddress::ZERO) {
                Vec::new()
            } else {
                value.contract_address.as_bytes().to_vec()
            },
            call_value: value.call_value.into(),
            data: value.data.to_bytes_vec(),
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

impl From<domain::contract::FrozenSupply>
    for asset_issue_contract::FrozenSupply
{
    fn from(value: domain::contract::FrozenSupply) -> Self {
        asset_issue_contract::FrozenSupply {
            frozen_amount: value.frozen_amount.into(),
            frozen_days: value.frozen_days,
        }
    }
}

impl TryFrom<AssetIssueContract> for domain::contract::AssetIssueContract {
    type Error = ProtoConvError;
    fn try_from(value: AssetIssueContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::AssetIssueContract {
            owner_address: addr(
                &value.owner_address,
                "AssetIssueContract.owner_address",
            )?,
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
            start_time: OffsetDateTime::try_from_tron(value.start_time)?,
            end_time: OffsetDateTime::try_from_tron(value.end_time)?,
            order: value.order,
            vote_score: value.vote_score,
            description: value.description.into(),
            url: value.url.into(),
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: OffsetDateTime::try_from_tron(
                value.public_latest_free_net_time,
            )?,
        })
    }
}

impl From<domain::contract::AssetIssueContract> for AssetIssueContract {
    fn from(value: domain::contract::AssetIssueContract) -> Self {
        AssetIssueContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            id: value.id,
            name: value.name.into(),
            abbr: value.abbr.into(),
            total_supply: value.total_supply,
            frozen_supply: value
                .frozen_supply
                .into_iter()
                .map(Into::into)
                .collect(),
            trx_num: i64::from(value.trx_num) as i32,
            precision: value.precision,
            num: value.num,
            start_time: value.start_time.to_tron(),
            end_time: value.end_time.to_tron(),
            order: value.order,
            vote_score: value.vote_score,
            description: value.description.into(),
            url: value.url.into(),
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: value
                .public_latest_free_net_time
                .to_tron(),
        }
    }
}

impl TryFrom<TransferAssetContract>
    for domain::contract::TransferAssetContract
{
    type Error = ProtoConvError;
    fn try_from(value: TransferAssetContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::TransferAssetContract {
            owner_address: addr(
                &value.owner_address,
                "TransferAssetContract.owner_address",
            )?,
            to_address: addr(
                &value.to_address,
                "TransferAssetContract.to_address",
            )?,
            amount: value.amount.into(),
            asset_name: value.asset_name.into(),
        })
    }
}

impl From<domain::contract::TransferAssetContract> for TransferAssetContract {
    fn from(value: domain::contract::TransferAssetContract) -> Self {
        TransferAssetContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            to_address: value.to_address.as_bytes().to_vec(),
            amount: value.amount.into(),
            asset_name: value.asset_name.into(),
        }
    }
}

impl TryFrom<UnfreezeAssetContract>
    for domain::contract::UnfreezeAssetContract
{
    type Error = ProtoConvError;
    fn try_from(value: UnfreezeAssetContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UnfreezeAssetContract {
            owner_address: addr(
                &value.owner_address,
                "UnfreezeAssetContract.owner_address",
            )?,
        })
    }
}

impl From<domain::contract::UnfreezeAssetContract> for UnfreezeAssetContract {
    fn from(value: domain::contract::UnfreezeAssetContract) -> Self {
        UnfreezeAssetContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<UpdateAssetContract> for domain::contract::UpdateAssetContract {
    type Error = ProtoConvError;
    fn try_from(value: UpdateAssetContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UpdateAssetContract {
            owner_address: addr(
                &value.owner_address,
                "UpdateAssetContract.owner_address",
            )?,
            description: value.description.into(),
            url: value.url.into(),
            new_limit: value.new_limit,
            new_public_limit: value.new_public_limit,
        })
    }
}

impl From<domain::contract::UpdateAssetContract> for UpdateAssetContract {
    fn from(value: domain::contract::UpdateAssetContract) -> Self {
        UpdateAssetContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            description: value.description.into(),
            url: value.url.into(),
            new_limit: value.new_limit,
            new_public_limit: value.new_public_limit,
        }
    }
}

impl TryFrom<ParticipateAssetIssueContract>
    for domain::contract::ParticipateAssetIssueContract
{
    type Error = ProtoConvError;
    fn try_from(
        value: ParticipateAssetIssueContract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::ParticipateAssetIssueContract {
            owner_address: addr(
                &value.owner_address,
                "ParticipateAssetIssueContract.owner_address",
            )?,
            to_address: addr(
                &value.to_address,
                "ParticipateAssetIssueContract.to_address",
            )?,
            asset_name: value.asset_name.into(),
            amount: value.amount.into(),
        })
    }
}

impl From<domain::contract::ParticipateAssetIssueContract>
    for ParticipateAssetIssueContract
{
    fn from(value: domain::contract::ParticipateAssetIssueContract) -> Self {
        ParticipateAssetIssueContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            to_address: value.to_address.as_bytes().to_vec(),
            asset_name: value.asset_name.into(),
            amount: value.amount.into(),
        }
    }
}

impl TryFrom<AccountCreateContract>
    for domain::contract::AccountCreateContract
{
    type Error = ProtoConvError;
    fn try_from(value: AccountCreateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::AccountCreateContract {
            account_type: value.r#type().into(),
            owner_address: addr(
                &value.owner_address,
                "AccountCreateContract.owner_address",
            )?,
            account_address: addr(
                &value.account_address,
                "AccountCreateContract.account_address",
            )?,
        })
    }
}

impl From<domain::contract::AccountCreateContract> for AccountCreateContract {
    fn from(value: domain::contract::AccountCreateContract) -> Self {
        AccountCreateContract {
            r#type: AccountType::from(value.account_type).into(),
            owner_address: value.owner_address.as_bytes().to_vec(),
            account_address: value.account_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<AccountUpdateContract>
    for domain::contract::AccountUpdateContract
{
    type Error = ProtoConvError;
    fn try_from(value: AccountUpdateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::AccountUpdateContract {
            owner_address: addr(
                &value.owner_address,
                "AccountUpdateContract.owner_address",
            )?,
            account_name: value.account_name.into(),
        })
    }
}

impl From<domain::contract::AccountUpdateContract> for AccountUpdateContract {
    fn from(value: domain::contract::AccountUpdateContract) -> Self {
        AccountUpdateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            account_name: value.account_name.into(),
        }
    }
}

impl TryFrom<SetAccountIdContract> for domain::contract::SetAccountIdContract {
    type Error = ProtoConvError;
    fn try_from(value: SetAccountIdContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::SetAccountIdContract {
            owner_address: addr(
                &value.owner_address,
                "SetAccountIdContract.owner_address",
            )?,
            account_id: value.account_id.into(),
        })
    }
}

impl From<domain::contract::SetAccountIdContract> for SetAccountIdContract {
    fn from(value: domain::contract::SetAccountIdContract) -> Self {
        SetAccountIdContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            account_id: value.account_id.into(),
        }
    }
}

impl TryFrom<AccountPermissionUpdateContract>
    for domain::contract::AccountPermissionUpdateContract
{
    type Error = ProtoConvError;
    fn try_from(
        value: AccountPermissionUpdateContract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::AccountPermissionUpdateContract {
            owner_address: addr(
                &value.owner_address,
                "AccountPermissionUpdateContract.owner_address",
            )?,
            owner: value.owner.map(TryInto::try_into).transpose()?,
            witness: value.witness.map(TryInto::try_into).transpose()?,
            actives: value
                .actives
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<domain::contract::AccountPermissionUpdateContract>
    for AccountPermissionUpdateContract
{
    fn from(value: domain::contract::AccountPermissionUpdateContract) -> Self {
        AccountPermissionUpdateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            owner: value.owner.map(Into::into),
            witness: value.witness.map(Into::into),
            actives: value.actives.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<WitnessCreateContract>
    for domain::contract::WitnessCreateContract
{
    type Error = ProtoConvError;
    fn try_from(value: WitnessCreateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::WitnessCreateContract {
            owner_address: addr(
                &value.owner_address,
                "WitnessCreateContract.owner_address",
            )?,
            url: value.url.into(),
        })
    }
}

impl From<domain::contract::WitnessCreateContract> for WitnessCreateContract {
    fn from(value: domain::contract::WitnessCreateContract) -> Self {
        WitnessCreateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            url: value.url.into(),
        }
    }
}

impl TryFrom<WitnessUpdateContract>
    for domain::contract::WitnessUpdateContract
{
    type Error = ProtoConvError;
    fn try_from(value: WitnessUpdateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::WitnessUpdateContract {
            owner_address: addr(
                &value.owner_address,
                "WitnessUpdateContract.owner_address",
            )?,
            update_url: value.update_url.into(),
        })
    }
}

impl From<domain::contract::WitnessUpdateContract> for WitnessUpdateContract {
    fn from(value: domain::contract::WitnessUpdateContract) -> Self {
        WitnessUpdateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            update_url: value.update_url.into(),
        }
    }
}

impl TryFrom<vote_witness_contract::Vote> for domain::contract::Vote {
    type Error = ProtoConvError;
    fn try_from(
        value: vote_witness_contract::Vote,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::Vote {
            vote_address: addr(
                &value.vote_address,
                "VoteWitnessContract.Vote.vote_address",
            )?,
            vote_count: value.vote_count,
        })
    }
}

impl From<domain::contract::Vote> for vote_witness_contract::Vote {
    fn from(value: domain::contract::Vote) -> Self {
        vote_witness_contract::Vote {
            vote_address: value.vote_address.as_bytes().to_vec(),
            vote_count: value.vote_count,
        }
    }
}

impl TryFrom<VoteWitnessContract> for domain::contract::VoteWitnessContract {
    type Error = ProtoConvError;
    fn try_from(value: VoteWitnessContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::VoteWitnessContract {
            owner_address: addr(
                &value.owner_address,
                "VoteWitnessContract.owner_address",
            )?,
            votes: value
                .votes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            support: value.support,
        })
    }
}

impl From<domain::contract::VoteWitnessContract> for VoteWitnessContract {
    fn from(value: domain::contract::VoteWitnessContract) -> Self {
        VoteWitnessContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            votes: value.votes.into_iter().map(Into::into).collect(),
            support: value.support,
        }
    }
}

impl TryFrom<FreezeBalanceContract>
    for domain::contract::FreezeBalanceContract
{
    type Error = ProtoConvError;
    fn try_from(value: FreezeBalanceContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::FreezeBalanceContract {
            owner_address: addr(
                &value.owner_address,
                "FreezeBalanceContract.owner_address",
            )?,
            frozen_balance: value.frozen_balance.into(),
            frozen_duration: time::Duration::days(value.frozen_duration),
            resource: value.resource().into(),
            receiver_address: addr_or_zero(
                &value.receiver_address,
                "FreezeBalanceContract.receiver_address",
            )?,
        })
    }
}

impl From<domain::contract::FreezeBalanceContract> for FreezeBalanceContract {
    fn from(value: domain::contract::FreezeBalanceContract) -> Self {
        FreezeBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            frozen_balance: value.frozen_balance.into(),
            frozen_duration: value.frozen_duration.whole_days(),
            resource: ResourceCode::from(value.resource).into(),
            receiver_address: value.receiver_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<UnfreezeBalanceContract>
    for domain::contract::UnfreezeBalanceContract
{
    type Error = ProtoConvError;
    fn try_from(value: UnfreezeBalanceContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UnfreezeBalanceContract {
            owner_address: addr(
                &value.owner_address,
                "UnfreezeBalanceContract.owner_address",
            )?,
            resource: value.resource().into(),
            receiver_address: addr_or_zero(
                &value.receiver_address,
                "UnfreezeBalanceContract.receiver_address",
            )?,
        })
    }
}

impl From<domain::contract::UnfreezeBalanceContract>
    for UnfreezeBalanceContract
{
    fn from(value: domain::contract::UnfreezeBalanceContract) -> Self {
        UnfreezeBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            receiver_address: value.receiver_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<WithdrawBalanceContract>
    for domain::contract::WithdrawBalanceContract
{
    type Error = ProtoConvError;
    fn try_from(value: WithdrawBalanceContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::WithdrawBalanceContract {
            owner_address: addr(
                &value.owner_address,
                "WithdrawBalanceContract.owner_address",
            )?,
        })
    }
}

impl From<domain::contract::WithdrawBalanceContract>
    for WithdrawBalanceContract
{
    fn from(value: domain::contract::WithdrawBalanceContract) -> Self {
        WithdrawBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<TransferContract> for domain::contract::TransferContract {
    type Error = ProtoConvError;
    fn try_from(value: TransferContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::TransferContract {
            owner_address: addr(
                &value.owner_address,
                "TransferContract.owner_address",
            )?,
            to_address: addr(&value.to_address, "TransferContract.to_address")?,
            amount: value.amount.into(),
        })
    }
}

impl From<domain::contract::TransferContract> for TransferContract {
    fn from(value: domain::contract::TransferContract) -> Self {
        TransferContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            to_address: value.to_address.as_bytes().to_vec(),
            amount: value.amount.into(),
        }
    }
}

impl TryFrom<transaction_balance_trace::Operation>
    for domain::contract::Operation
{
    type Error = ProtoConvError;
    fn try_from(
        value: transaction_balance_trace::Operation,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::Operation {
            operation_identifier: value.operation_identifier,
            address: addr_or_zero(&value.address, "Operation.address")?,
            amount: value.amount.into(),
        })
    }
}

impl From<domain::contract::Operation>
    for transaction_balance_trace::Operation
{
    fn from(value: domain::contract::Operation) -> Self {
        transaction_balance_trace::Operation {
            operation_identifier: value.operation_identifier,
            address: value.address.as_bytes().to_vec(),
            amount: value.amount.into(),
        }
    }
}

impl TryFrom<TransactionBalanceTrace>
    for domain::contract::TransactionBalanceTrace
{
    type Error = ProtoConvError;
    fn try_from(value: TransactionBalanceTrace) -> Result<Self, Self::Error> {
        Ok(domain::contract::TransactionBalanceTrace {
            transaction_identifier: value.transaction_identifier.into(),
            operation: value
                .operation
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            r#type: value.r#type,
            status: value.status,
        })
    }
}

impl From<domain::contract::TransactionBalanceTrace>
    for TransactionBalanceTrace
{
    fn from(value: domain::contract::TransactionBalanceTrace) -> Self {
        TransactionBalanceTrace {
            transaction_identifier: value.transaction_identifier.into(),
            operation: value.operation.into_iter().map(Into::into).collect(),
            r#type: value.r#type,
            status: value.status,
        }
    }
}

impl TryFrom<BlockBalanceTrace> for domain::contract::BlockBalanceTrace {
    type Error = ProtoConvError;
    fn try_from(value: BlockBalanceTrace) -> Result<Self, Self::Error> {
        Ok(domain::contract::BlockBalanceTrace {
            block_identifier: value.block_identifier.unwrap_or_default().into(),
            timestamp: OffsetDateTime::try_from_tron(value.timestamp)?,
            transaction_balance_trace: value
                .transaction_balance_trace
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<domain::contract::BlockBalanceTrace> for BlockBalanceTrace {
    fn from(value: domain::contract::BlockBalanceTrace) -> Self {
        BlockBalanceTrace {
            block_identifier: Some(value.block_identifier.into()),
            timestamp: value.timestamp.to_tron(),
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

impl From<domain::contract::BlockIdentifier>
    for block_balance_trace::BlockIdentifier
{
    fn from(value: domain::contract::BlockIdentifier) -> Self {
        block_balance_trace::BlockIdentifier {
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

impl From<domain::contract::AccountTrace> for AccountTrace {
    fn from(value: domain::contract::AccountTrace) -> Self {
        AccountTrace {
            balance: value.balance.into(),
            placeholder: value.placeholder.into(),
        }
    }
}

impl TryFrom<AccountIdentifier> for domain::contract::AccountIdentifier {
    type Error = ProtoConvError;
    fn try_from(value: AccountIdentifier) -> Result<Self, Self::Error> {
        Ok(domain::contract::AccountIdentifier {
            address: addr(&value.address, "AccountIdentifier.address")?,
        })
    }
}

impl From<domain::contract::AccountIdentifier> for AccountIdentifier {
    fn from(value: domain::contract::AccountIdentifier) -> Self {
        AccountIdentifier {
            address: value.address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<AccountBalanceRequest>
    for domain::contract::AccountBalanceRequest
{
    type Error = ProtoConvError;
    fn try_from(value: AccountBalanceRequest) -> Result<Self, Self::Error> {
        Ok(domain::contract::AccountBalanceRequest {
            account_identifier: value
                .account_identifier
                .unwrap_or_default()
                .try_into()?,
            block_identifier: value.block_identifier.unwrap_or_default().into(),
        })
    }
}

impl From<domain::contract::AccountBalanceRequest> for AccountBalanceRequest {
    fn from(value: domain::contract::AccountBalanceRequest) -> Self {
        AccountBalanceRequest {
            account_identifier: Some(value.account_identifier.into()),
            block_identifier: Some(value.block_identifier.into()),
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

impl From<domain::contract::AccountBalanceResponse> for AccountBalanceResponse {
    fn from(value: domain::contract::AccountBalanceResponse) -> Self {
        AccountBalanceResponse {
            balance: value.balance.into(),
            block_identifier: Some(value.block_identifier.into()),
        }
    }
}

impl TryFrom<FreezeBalanceV2Contract>
    for domain::contract::FreezeBalanceV2Contract
{
    type Error = ProtoConvError;
    fn try_from(value: FreezeBalanceV2Contract) -> Result<Self, Self::Error> {
        Ok(domain::contract::FreezeBalanceV2Contract {
            owner_address: addr(
                &value.owner_address,
                "FreezeBalanceV2Contract.owner_address",
            )?,
            frozen_balance: value.frozen_balance.into(),
            resource: value.resource().into(),
        })
    }
}

impl From<domain::contract::FreezeBalanceV2Contract>
    for FreezeBalanceV2Contract
{
    fn from(value: domain::contract::FreezeBalanceV2Contract) -> Self {
        FreezeBalanceV2Contract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            frozen_balance: value.frozen_balance.into(),
            resource: ResourceCode::from(value.resource).into(),
        }
    }
}

impl TryFrom<UnfreezeBalanceV2Contract>
    for domain::contract::UnfreezeBalanceV2Contract
{
    type Error = ProtoConvError;
    fn try_from(value: UnfreezeBalanceV2Contract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UnfreezeBalanceV2Contract {
            owner_address: addr(
                &value.owner_address,
                "UnfreezeBalanceV2Contract.owner_address",
            )?,
            resource: value.resource().into(),
            unfreeze_balance: value.unfreeze_balance.into(),
        })
    }
}

impl From<domain::contract::UnfreezeBalanceV2Contract>
    for UnfreezeBalanceV2Contract
{
    fn from(value: domain::contract::UnfreezeBalanceV2Contract) -> Self {
        UnfreezeBalanceV2Contract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            unfreeze_balance: value.unfreeze_balance.into(),
        }
    }
}

impl TryFrom<WithdrawExpireUnfreezeContract>
    for domain::contract::WithdrawExpireUnfreezeContract
{
    type Error = ProtoConvError;
    fn try_from(
        value: WithdrawExpireUnfreezeContract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::WithdrawExpireUnfreezeContract {
            owner_address: addr(
                &value.owner_address,
                "WithdrawExpireUnfreezeContract.owner_address",
            )?,
        })
    }
}

impl From<domain::contract::WithdrawExpireUnfreezeContract>
    for WithdrawExpireUnfreezeContract
{
    fn from(value: domain::contract::WithdrawExpireUnfreezeContract) -> Self {
        WithdrawExpireUnfreezeContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<DelegateResourceContract>
    for domain::contract::DelegateResourceContract
{
    type Error = ProtoConvError;
    fn try_from(value: DelegateResourceContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::DelegateResourceContract {
            owner_address: addr(
                &value.owner_address,
                "DelegateResourceContract.owner_address",
            )?,
            resource: value.resource().into(),
            balance: value.balance.into(),
            receiver_address: addr(
                &value.receiver_address,
                "DelegateResourceContract.receiver_address",
            )?,
            lock_period: value
                .lock
                .then_some(time::Duration::seconds(value.lock_period)),
        })
    }
}

impl From<domain::contract::DelegateResourceContract>
    for DelegateResourceContract
{
    fn from(value: domain::contract::DelegateResourceContract) -> Self {
        DelegateResourceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            balance: value.balance.into(),
            receiver_address: value.receiver_address.as_bytes().to_vec(),
            lock: value.lock_period.is_some(),
            lock_period: value.lock_period.unwrap_or_default().whole_seconds(),
        }
    }
}

impl TryFrom<UnDelegateResourceContract>
    for domain::contract::UnDelegateResourceContract
{
    type Error = ProtoConvError;
    fn try_from(
        value: UnDelegateResourceContract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::UnDelegateResourceContract {
            owner_address: addr(
                &value.owner_address,
                "UnDelegateResourceContract.owner_address",
            )?,
            resource: value.resource().into(),
            balance: value.balance.into(),
            receiver_address: addr(
                &value.receiver_address,
                "UnDelegateResourceContract.receiver_address",
            )?,
        })
    }
}

impl From<domain::contract::UnDelegateResourceContract>
    for UnDelegateResourceContract
{
    fn from(value: domain::contract::UnDelegateResourceContract) -> Self {
        UnDelegateResourceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            balance: value.balance.into(),
            receiver_address: value.receiver_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<CancelAllUnfreezeV2Contract>
    for domain::contract::CancelAllUnfreezeV2Contract
{
    type Error = ProtoConvError;
    fn try_from(
        value: CancelAllUnfreezeV2Contract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::CancelAllUnfreezeV2Contract {
            owner_address: addr(
                &value.owner_address,
                "CancelAllUnfreezeV2Contract.owner_address",
            )?,
        })
    }
}

impl From<domain::contract::CancelAllUnfreezeV2Contract>
    for CancelAllUnfreezeV2Contract
{
    fn from(value: domain::contract::CancelAllUnfreezeV2Contract) -> Self {
        CancelAllUnfreezeV2Contract {
            owner_address: value.owner_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<ProposalApproveContract>
    for domain::contract::ProposalApproveContract
{
    type Error = ProtoConvError;
    fn try_from(value: ProposalApproveContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ProposalApproveContract {
            owner_address: addr(
                &value.owner_address,
                "ProposalApproveContract.owner_address",
            )?,
            proposal_id: value.proposal_id,
            is_add_approval: value.is_add_approval,
        })
    }
}

impl From<domain::contract::ProposalApproveContract>
    for ProposalApproveContract
{
    fn from(value: domain::contract::ProposalApproveContract) -> Self {
        ProposalApproveContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            proposal_id: value.proposal_id,
            is_add_approval: value.is_add_approval,
        }
    }
}

impl TryFrom<ProposalCreateContract>
    for domain::contract::ProposalCreateContract
{
    type Error = ProtoConvError;
    fn try_from(value: ProposalCreateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ProposalCreateContract {
            owner_address: addr(
                &value.owner_address,
                "ProposalCreateContract.owner_address",
            )?,
            parameters: value.parameters,
        })
    }
}

impl From<domain::contract::ProposalCreateContract> for ProposalCreateContract {
    fn from(value: domain::contract::ProposalCreateContract) -> Self {
        ProposalCreateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            parameters: value.parameters,
        }
    }
}

impl TryFrom<ProposalDeleteContract>
    for domain::contract::ProposalDeleteContract
{
    type Error = ProtoConvError;
    fn try_from(value: ProposalDeleteContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ProposalDeleteContract {
            owner_address: addr(
                &value.owner_address,
                "ProposalDeleteContract.owner_address",
            )?,
            proposal_id: value.proposal_id,
        })
    }
}

impl From<domain::contract::ProposalDeleteContract> for ProposalDeleteContract {
    fn from(value: domain::contract::ProposalDeleteContract) -> Self {
        ProposalDeleteContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            proposal_id: value.proposal_id,
        }
    }
}

impl TryFrom<BuyStorageContract> for domain::contract::BuyStorageContract {
    type Error = ProtoConvError;
    fn try_from(value: BuyStorageContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::BuyStorageContract {
            owner_address: addr(
                &value.owner_address,
                "BuyStorageContract.owner_address",
            )?,
            quant: value.quant.into(),
        })
    }
}

impl From<domain::contract::BuyStorageContract> for BuyStorageContract {
    fn from(value: domain::contract::BuyStorageContract) -> Self {
        BuyStorageContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            quant: value.quant.into(),
        }
    }
}

impl TryFrom<SellStorageContract> for domain::contract::SellStorageContract {
    type Error = ProtoConvError;
    fn try_from(value: SellStorageContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::SellStorageContract {
            owner_address: addr(
                &value.owner_address,
                "SellStorageContract.owner_address",
            )?,
            storage_bytes: value.storage_bytes,
        })
    }
}

impl From<domain::contract::SellStorageContract> for SellStorageContract {
    fn from(value: domain::contract::SellStorageContract) -> Self {
        SellStorageContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            storage_bytes: value.storage_bytes,
        }
    }
}

impl TryFrom<UpdateBrokerageContract>
    for domain::contract::UpdateBrokerageContract
{
    type Error = ProtoConvError;
    fn try_from(value: UpdateBrokerageContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UpdateBrokerageContract {
            owner_address: addr(
                &value.owner_address,
                "UpdateBrokerageContract.owner_address",
            )?,
            brokerage: value.brokerage,
        })
    }
}

impl From<domain::contract::UpdateBrokerageContract>
    for UpdateBrokerageContract
{
    fn from(value: domain::contract::UpdateBrokerageContract) -> Self {
        UpdateBrokerageContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            brokerage: value.brokerage,
        }
    }
}

impl TryFrom<ExchangeCreateContract>
    for domain::contract::ExchangeCreateContract
{
    type Error = ProtoConvError;
    fn try_from(value: ExchangeCreateContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ExchangeCreateContract {
            owner_address: addr(
                &value.owner_address,
                "ExchangeCreateContract.owner_address",
            )?,
            first_token_id: value.first_token_id.into(),
            first_token_balance: value.first_token_balance,
            second_token_id: value.second_token_id.into(),
            second_token_balance: value.second_token_balance,
        })
    }
}

impl From<domain::contract::ExchangeCreateContract> for ExchangeCreateContract {
    fn from(value: domain::contract::ExchangeCreateContract) -> Self {
        ExchangeCreateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            first_token_id: value.first_token_id.into(),
            first_token_balance: value.first_token_balance,
            second_token_id: value.second_token_id.into(),
            second_token_balance: value.second_token_balance,
        }
    }
}

impl TryFrom<ExchangeInjectContract>
    for domain::contract::ExchangeInjectContract
{
    type Error = ProtoConvError;
    fn try_from(value: ExchangeInjectContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ExchangeInjectContract {
            owner_address: addr(
                &value.owner_address,
                "ExchangeInjectContract.owner_address",
            )?,
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        })
    }
}

impl From<domain::contract::ExchangeInjectContract> for ExchangeInjectContract {
    fn from(value: domain::contract::ExchangeInjectContract) -> Self {
        ExchangeInjectContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        }
    }
}

impl TryFrom<ExchangeWithdrawContract>
    for domain::contract::ExchangeWithdrawContract
{
    type Error = ProtoConvError;
    fn try_from(value: ExchangeWithdrawContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ExchangeWithdrawContract {
            owner_address: addr(
                &value.owner_address,
                "ExchangeWithdrawContract.owner_address",
            )?,
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        })
    }
}

impl From<domain::contract::ExchangeWithdrawContract>
    for ExchangeWithdrawContract
{
    fn from(value: domain::contract::ExchangeWithdrawContract) -> Self {
        ExchangeWithdrawContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
        }
    }
}

impl TryFrom<ExchangeTransactionContract>
    for domain::contract::ExchangeTransactionContract
{
    type Error = ProtoConvError;
    fn try_from(
        value: ExchangeTransactionContract,
    ) -> Result<Self, Self::Error> {
        Ok(domain::contract::ExchangeTransactionContract {
            owner_address: addr(
                &value.owner_address,
                "ExchangeTransactionContract.owner_address",
            )?,
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
            expected: value.expected,
        })
    }
}

impl From<domain::contract::ExchangeTransactionContract>
    for ExchangeTransactionContract
{
    fn from(value: domain::contract::ExchangeTransactionContract) -> Self {
        ExchangeTransactionContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            exchange_id: value.exchange_id,
            token_id: value.token_id.into(),
            quant: value.quant,
            expected: value.expected,
        }
    }
}

impl TryFrom<MarketSellAssetContract>
    for domain::contract::MarketSellAssetContract
{
    type Error = ProtoConvError;
    fn try_from(value: MarketSellAssetContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::MarketSellAssetContract {
            owner_address: addr(
                &value.owner_address,
                "MarketSellAssetContract.owner_address",
            )?,
            sell_token_id: value.sell_token_id.into(),
            sell_token_quantity: value.sell_token_quantity,
            buy_token_id: value.buy_token_id.into(),
            buy_token_quantity: value.buy_token_quantity,
        })
    }
}

impl From<domain::contract::MarketSellAssetContract>
    for MarketSellAssetContract
{
    fn from(value: domain::contract::MarketSellAssetContract) -> Self {
        MarketSellAssetContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            sell_token_id: value.sell_token_id.into(),
            sell_token_quantity: value.sell_token_quantity,
            buy_token_id: value.buy_token_id.into(),
            buy_token_quantity: value.buy_token_quantity,
        }
    }
}

impl TryFrom<MarketCancelOrderContract>
    for domain::contract::MarketCancelOrderContract
{
    type Error = ProtoConvError;
    fn try_from(value: MarketCancelOrderContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::MarketCancelOrderContract {
            owner_address: addr(
                &value.owner_address,
                "MarketCancelOrderContract.owner_address",
            )?,
            order_id: value.order_id.into(),
        })
    }
}

impl From<domain::contract::MarketCancelOrderContract>
    for MarketCancelOrderContract
{
    fn from(value: domain::contract::MarketCancelOrderContract) -> Self {
        MarketCancelOrderContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            order_id: value.order_id.into(),
        }
    }
}

impl TryFrom<SmartContract> for domain::contract::SmartContract {
    type Error = ProtoConvError;
    fn try_from(value: SmartContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::SmartContract {
            origin_address: addr(
                &value.origin_address,
                "SmartContract.origin_address",
            )?,
            contract_address: addr_or_zero(
                &value.contract_address,
                "SmartContract.contract_address",
            )?,
            abi: value.abi.unwrap_or_default().into(),
            bytecode: value.bytecode,
            call_value: value.call_value.into(),
            consume_user_resource_percent: value.consume_user_resource_percent,
            name: value.name,
            origin_energy_limit: value.origin_energy_limit,
            code_hash: value.code_hash.into(),
            trx_hash: value.trx_hash.into(),
            version: value.version,
        })
    }
}

impl From<domain::contract::SmartContract> for SmartContract {
    fn from(value: domain::contract::SmartContract) -> Self {
        SmartContract {
            origin_address: value.origin_address.as_bytes().to_vec(),
            contract_address: if value.contract_address.eq(&TronAddress::ZERO) {
                Vec::new()
            } else {
                value.contract_address.as_bytes().to_vec()
            },
            abi: Some(value.abi.into()),
            bytecode: value.bytecode,
            call_value: value.call_value.into(),
            consume_user_resource_percent: value.consume_user_resource_percent,
            name: value.name,
            origin_energy_limit: value.origin_energy_limit,
            code_hash: value.code_hash.into(),
            trx_hash: value.trx_hash.into(),
            version: value.version,
        }
    }
}

impl From<smart_contract::Abi> for domain::contract::Abi {
    fn from(value: smart_contract::Abi) -> Self {
        domain::contract::Abi {
            entrys: value.entrys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::contract::Abi> for smart_contract::Abi {
    fn from(value: domain::contract::Abi) -> Self {
        smart_contract::Abi {
            entrys: value.entrys.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<smart_contract::abi::Entry> for domain::contract::Entry {
    fn from(value: smart_contract::abi::Entry) -> Self {
        domain::contract::Entry {
            anonymous: value.anonymous,
            constant: value.constant,
            entry_type: value.r#type().into(),
            state_mutability: value.state_mutability().into(),
            name: value.name,
            inputs: value.inputs.into_iter().map(Into::into).collect(),
            outputs: value.outputs.into_iter().map(Into::into).collect(),
            payable: value.payable,
        }
    }
}

impl From<domain::contract::Entry> for smart_contract::abi::Entry {
    fn from(value: domain::contract::Entry) -> Self {
        smart_contract::abi::Entry {
            anonymous: value.anonymous,
            constant: value.constant,
            r#type: smart_contract::abi::entry::EntryType::from(
                value.entry_type,
            )
            .into(),
            state_mutability:
                smart_contract::abi::entry::StateMutabilityType::from(
                    value.state_mutability,
                )
                .into(),
            name: value.name,
            inputs: value.inputs.into_iter().map(Into::into).collect(),
            outputs: value.outputs.into_iter().map(Into::into).collect(),
            payable: value.payable,
        }
    }
}

impl From<smart_contract::abi::entry::Param> for domain::contract::Param {
    fn from(value: smart_contract::abi::entry::Param) -> Self {
        domain::contract::Param {
            indexed: value.indexed,
            name: value.name,
            param_type: value.r#type,
        }
    }
}

impl From<domain::contract::Param> for smart_contract::abi::entry::Param {
    fn from(value: domain::contract::Param) -> Self {
        smart_contract::abi::entry::Param {
            indexed: value.indexed,
            name: value.name,
            r#type: value.param_type,
        }
    }
}

impl_enum_conversions! {
    smart_contract::abi::entry::EntryType => domain::contract::EntryType {
        UnknownEntryType,
        Constructor,
        Function,
        Event,
        Fallback,
        Receive,
        Error
    }
}

impl_enum_conversions! {
    smart_contract::abi::entry::StateMutabilityType => domain::contract::StateMutabilityType {
        UnknownMutabilityType,
        Pure,
        View,
        Nonpayable,
        Payable
    }
}

impl From<ContractState> for domain::contract::ContractState {
    fn from(value: ContractState) -> Self {
        domain::contract::ContractState {
            energy_usage: value.energy_usage,
            energy_factor: value.energy_factor,
            update_cycle: value.update_cycle,
        }
    }
}

impl TryFrom<CreateSmartContract> for domain::contract::CreateSmartContract {
    type Error = ProtoConvError;
    fn try_from(value: CreateSmartContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::CreateSmartContract {
            owner_address: addr(
                &value.owner_address,
                "CreateSmartContract.owner_address",
            )?,
            new_contract: value.new_contract.unwrap_or_default().try_into()?,
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        })
    }
}

impl From<domain::contract::CreateSmartContract> for CreateSmartContract {
    fn from(value: domain::contract::CreateSmartContract) -> Self {
        CreateSmartContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            new_contract: Some(value.new_contract.into()),
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        }
    }
}

impl TryFrom<ClearAbiContract> for domain::contract::ClearAbiContract {
    type Error = ProtoConvError;
    fn try_from(value: ClearAbiContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ClearAbiContract {
            owner_address: addr(
                &value.owner_address,
                "ClearAbiContract.owner_address",
            )?,
            contract_address: addr(
                &value.contract_address,
                "ClearAbiContract.contract_address",
            )?,
        })
    }
}

impl From<domain::contract::ClearAbiContract> for ClearAbiContract {
    fn from(value: domain::contract::ClearAbiContract) -> Self {
        ClearAbiContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value.contract_address.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<UpdateSettingContract>
    for domain::contract::UpdateSettingContract
{
    type Error = ProtoConvError;
    fn try_from(value: UpdateSettingContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UpdateSettingContract {
            owner_address: addr(
                &value.owner_address,
                "UpdateSettingContract.owner_address",
            )?,
            contract_address: addr(
                &value.contract_address,
                "UpdateSettingContract.contract_address",
            )?,
            consume_user_resource_percent: value.consume_user_resource_percent,
        })
    }
}

impl From<domain::contract::UpdateSettingContract> for UpdateSettingContract {
    fn from(value: domain::contract::UpdateSettingContract) -> Self {
        UpdateSettingContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value.contract_address.as_bytes().to_vec(),
            consume_user_resource_percent: value.consume_user_resource_percent,
        }
    }
}

impl TryFrom<UpdateEnergyLimitContract>
    for domain::contract::UpdateEnergyLimitContract
{
    type Error = ProtoConvError;
    fn try_from(value: UpdateEnergyLimitContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::UpdateEnergyLimitContract {
            owner_address: addr(
                &value.owner_address,
                "UpdateEnergyLimitContract.owner_address",
            )?,
            contract_address: addr(
                &value.contract_address,
                "UpdateEnergyLimitContract.contract_address",
            )?,
            origin_energy_limit: value.origin_energy_limit,
        })
    }
}

impl From<domain::contract::UpdateEnergyLimitContract>
    for UpdateEnergyLimitContract
{
    fn from(value: domain::contract::UpdateEnergyLimitContract) -> Self {
        UpdateEnergyLimitContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value.contract_address.as_bytes().to_vec(),
            origin_energy_limit: value.origin_energy_limit,
        }
    }
}

impl From<SpendDescription> for domain::contract::SpendDescription {
    fn from(value: SpendDescription) -> Self {
        domain::contract::SpendDescription {
            value_commitment: value.value_commitment,
            anchor: value.anchor,
            nullifier: value.nullifier,
            rk: value.rk,
            zkproof: value.zkproof,
            spend_authority_signature: value.spend_authority_signature,
        }
    }
}

impl From<domain::contract::SpendDescription> for SpendDescription {
    fn from(value: domain::contract::SpendDescription) -> Self {
        SpendDescription {
            value_commitment: value.value_commitment,
            anchor: value.anchor,
            nullifier: value.nullifier,
            rk: value.rk,
            zkproof: value.zkproof,
            spend_authority_signature: value.spend_authority_signature,
        }
    }
}

impl From<ReceiveDescription> for domain::contract::ReceiveDescription {
    fn from(value: ReceiveDescription) -> Self {
        domain::contract::ReceiveDescription {
            value_commitment: value.value_commitment,
            note_commitment: value.note_commitment,
            epk: value.epk,
            c_enc: value.c_enc,
            c_out: value.c_out,
            zkproof: value.zkproof,
        }
    }
}

impl From<domain::contract::ReceiveDescription> for ReceiveDescription {
    fn from(value: domain::contract::ReceiveDescription) -> Self {
        ReceiveDescription {
            value_commitment: value.value_commitment,
            note_commitment: value.note_commitment,
            epk: value.epk,
            c_enc: value.c_enc,
            c_out: value.c_out,
            zkproof: value.zkproof,
        }
    }
}

impl TryFrom<ShieldedTransferContract>
    for domain::contract::ShieldedTransferContract
{
    type Error = ProtoConvError;
    fn try_from(value: ShieldedTransferContract) -> Result<Self, Self::Error> {
        Ok(domain::contract::ShieldedTransferContract {
            transparent_from_address: addr_or_zero(
                &value.transparent_from_address,
                "ShieldedTransferContract.transparent_from_address",
            )?,
            from_amount: value.from_amount,
            spend_description: value
                .spend_description
                .into_iter()
                .map(Into::into)
                .collect(),
            receive_description: value
                .receive_description
                .into_iter()
                .map(Into::into)
                .collect(),
            binding_signature: value.binding_signature,
            transparent_to_address: addr_or_zero(
                &value.transparent_to_address,
                "ShieldedTransferContract.transparent_to_address",
            )?,
            to_amount: value.to_amount,
        })
    }
}

impl From<domain::contract::ShieldedTransferContract>
    for ShieldedTransferContract
{
    fn from(value: domain::contract::ShieldedTransferContract) -> Self {
        ShieldedTransferContract {
            transparent_from_address: value
                .transparent_from_address
                .as_bytes()
                .to_vec(),
            from_amount: value.from_amount,
            spend_description: value
                .spend_description
                .into_iter()
                .map(Into::into)
                .collect(),
            receive_description: value
                .receive_description
                .into_iter()
                .map(Into::into)
                .collect(),
            binding_signature: value.binding_signature,
            transparent_to_address: value
                .transparent_to_address
                .as_bytes()
                .to_vec(),
            to_amount: value.to_amount,
        }
    }
}

macro_rules! impl_contract_conversion {
    ($($variant:ident),+ $(,)?) => {
        impl TryFrom<transaction::Contract> for domain::contract::Contract {
            type Error = ProtoConvError;
            fn try_from(c: transaction::Contract) -> Result<Self, Self::Error> {
                use transaction::contract::ContractType as ProtoContractType;

                let type_id = c.r#type;
                let param = c.parameter.map(|p| p.value).unwrap_or_default();

                let contract_type = match ProtoContractType::try_from(type_id) {
                    $(
                        Ok(ProtoContractType::$variant) => {
                            let decoded: $variant =
                                prost::Message::decode(param.as_slice())
                                    .map_err(|e| {
                                        ProtoConvError::BadParameter(
                                            e.to_string(),
                                        )
                                    })?;
                            domain::contract::ContractType::$variant(
                                decoded.try_into()?,
                            )
                        }
                    )+
                    _ => domain::contract::ContractType::Unknown {
                        type_id,
                        raw: param,
                    },
                };

                Ok(domain::contract::Contract {
                    contract_type,
                    provider: c.provider,
                    contract_name: c.contract_name.into(),
                    permission_id: c.permission_id,
                })
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
    SetAccountIdContract,
    CreateSmartContract,
    TriggerSmartContract,
    UpdateSettingContract,
    ExchangeCreateContract,
    ExchangeInjectContract,
    ExchangeWithdrawContract,
    ExchangeTransactionContract,
    UpdateEnergyLimitContract,
    AccountPermissionUpdateContract,
    ClearAbiContract,
    UpdateBrokerageContract,
    ShieldedTransferContract,
    MarketSellAssetContract,
    MarketCancelOrderContract,
    FreezeBalanceV2Contract,
    UnfreezeBalanceV2Contract,
    WithdrawExpireUnfreezeContract,
    DelegateResourceContract,
    UnDelegateResourceContract,
    CancelAllUnfreezeV2Contract
);

macro_rules! impl_contract_reverse_conversion {
    ($($variant:ident),+ $(,)?) => {
        impl From<domain::contract::Contract> for transaction::Contract {
            fn from(c: domain::contract::Contract) -> Self {
                use domain::contract::ContractType;
                use prost::Message;

                let (type_id, parameter) = match c.contract_type {
                    $(
                        ContractType::$variant(inner) => {
                            let tx_contract: $variant = inner.into();
                            let proto_type =
                                transaction::contract::ContractType::$variant;
                            (
                                proto_type as i32,
                                Some(prost_types::Any {
                                    type_url: format!(
                                        "type.googleapis.com/protocol.{}",
                                        proto_type.as_str_name()
                                    ),
                                    value: tx_contract.encode_to_vec(),
                                }),
                            )
                        }
                    )+
                    ContractType::Unknown { type_id, raw } => {
                        let type_url =
                            transaction::contract::ContractType::try_from(
                                type_id,
                            )
                            .map(|t| {
                                format!(
                                    "type.googleapis.com/protocol.{}",
                                    t.as_str_name()
                                )
                            })
                            .unwrap_or_else(|_| {
                                "type.googleapis.com/protocol.UnknownContract"
                                    .to_string()
                            });
                        (
                            type_id,
                            (!raw.is_empty()).then(|| prost_types::Any {
                                type_url,
                                value: raw,
                            }),
                        )
                    }
                    other => {
                        let id = other.id();
                        let proto_type =
                            transaction::contract::ContractType::try_from(id)
                                .unwrap_or_else(|_| {
                                    tracing::warn!(
                                        type_id = id,
                                        "unknown contract type id, encoding as CustomContract"
                                    );
                                    transaction::contract::ContractType::CustomContract
                                });
                        (proto_type as i32, None)
                    }
                };

                transaction::Contract {
                    parameter,
                    r#type: type_id,
                    provider: c.provider,
                    contract_name: c.contract_name.into(),
                    permission_id: c.permission_id,
                }
            }
        }
    };
}

impl_contract_reverse_conversion!(
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
    SetAccountIdContract,
    CreateSmartContract,
    TriggerSmartContract,
    UpdateSettingContract,
    ExchangeCreateContract,
    ExchangeInjectContract,
    ExchangeWithdrawContract,
    ExchangeTransactionContract,
    UpdateEnergyLimitContract,
    AccountPermissionUpdateContract,
    ClearAbiContract,
    UpdateBrokerageContract,
    ShieldedTransferContract,
    MarketSellAssetContract,
    MarketCancelOrderContract,
    FreezeBalanceV2Contract,
    UnfreezeBalanceV2Contract,
    WithdrawExpireUnfreezeContract,
    DelegateResourceContract,
    UnDelegateResourceContract,
    CancelAllUnfreezeV2Contract
);

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn addr21() -> Vec<u8> {
        let mut v = vec![0x41u8];
        v.extend_from_slice(&[7u8; 20]);
        v
    }

    fn tx_with_contract(
        c: transaction::Contract,
        tag: u8,
    ) -> TransactionExtention {
        TransactionExtention {
            transaction: Some(Transaction {
                raw_data: Some(transaction::Raw {
                    ref_block_bytes: vec![0, 1],
                    ref_block_hash: vec![0u8; 8],
                    contract: vec![c],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            txid: {
                let mut t = vec![0u8; 32];
                t[0] = tag;
                t
            },
            ..Default::default()
        }
    }

    fn block_with(txs: Vec<TransactionExtention>) -> BlockExtention {
        BlockExtention {
            transactions: txs,
            block_header: Some(BlockHeader {
                raw_data: Some(block_header::Raw {
                    witness_address: addr21(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            blockid: vec![9u8; 32],
            ..Default::default()
        }
    }

    #[test]
    fn custom_contract_converts_to_unknown_without_panic() {
        let c = transaction::Contract {
            r#type: 20,
            parameter: Some(prost_types::Any {
                type_url: "type.googleapis.com/protocol.CustomContract".into(),
                value: vec![1, 2, 3],
            }),
            ..Default::default()
        };

        let block: domain::block::BlockExtention =
            block_with(vec![tx_with_contract(c, 1)]).try_into().unwrap();

        assert_eq!(block.transactions.len(), 1);
        assert!(block.conversion_errors.is_empty());

        let contract = &block.transactions[0]
            .transaction
            .as_ref()
            .unwrap()
            .raw
            .contract[0];
        match &contract.contract_type {
            domain::contract::ContractType::Unknown { type_id, raw } => {
                assert_eq!(*type_id, 20);
                assert_eq!(raw, &vec![1, 2, 3]);
            }
            other => panic!("expected Unknown, got {other:?}"),
        }
    }

    #[test]
    fn empty_contract_address_lands_in_conversion_errors() {
        let bad_trigger = TriggerSmartContract {
            owner_address: addr21(),
            contract_address: Vec::new(),
            ..Default::default()
        };
        let bad = transaction::Contract {
            r#type: transaction::contract::ContractType::TriggerSmartContract
                as i32,
            parameter: Some(prost_types::Any {
                type_url: "type.googleapis.com/protocol.TriggerSmartContract"
                    .into(),
                value: bad_trigger.encode_to_vec(),
            }),
            ..Default::default()
        };
        let good = transaction::Contract {
            r#type: transaction::contract::ContractType::TransferContract
                as i32,
            parameter: Some(prost_types::Any {
                type_url: "type.googleapis.com/protocol.TransferContract"
                    .into(),
                value: TransferContract {
                    owner_address: addr21(),
                    to_address: addr21(),
                    amount: 1,
                }
                .encode_to_vec(),
            }),
            ..Default::default()
        };

        let block: domain::block::BlockExtention = block_with(vec![
            tx_with_contract(bad, 1),
            tx_with_contract(good, 2),
        ])
        .try_into()
        .unwrap();

        assert_eq!(block.transactions.len(), 1, "good tx must survive");
        assert_eq!(block.conversion_errors.len(), 1);
        let (txid, err) = &block.conversion_errors[0];
        assert_eq!(txid.as_ref()[0], 1, "txid of the broken tx is reported");
        assert!(
            matches!(err, ProtoConvError::BadAddress { .. }),
            "got: {err:?}"
        );
    }

    #[test]
    fn unknown_roundtrip_preserves_type_and_bytes() {
        let dc = domain::contract::Contract {
            contract_type: domain::contract::ContractType::Unknown {
                type_id: 32,
                raw: vec![5, 6, 7],
            },
            ..Default::default()
        };
        let pc: transaction::Contract = dc.into();
        assert_eq!(pc.r#type, 32);
        assert_eq!(pc.parameter.unwrap().value, vec![5, 6, 7]);

        let dc = domain::contract::Contract {
            contract_type: domain::contract::ContractType::Unknown {
                type_id: 777,
                raw: vec![9],
            },
            ..Default::default()
        };
        let pc: transaction::Contract = dc.into();
        assert_eq!(pc.r#type, 777);
        assert_eq!(pc.parameter.unwrap().value, vec![9]);
    }

    #[test]
    fn internal_transaction_accepts_20_and_21_byte_addresses() {
        let it = InternalTransaction {
            caller_address: addr21(),
            transfer_to_address: vec![7u8; 20], // no 0x41 prefix
            ..Default::default()
        };
        let d: domain::transaction::InternalTransaction =
            it.try_into().unwrap();
        assert_eq!(
            d.caller_address, d.transfer_to_address,
            "20-byte address must be normalized to the same 21-byte address"
        );

        let bad = InternalTransaction {
            caller_address: vec![7u8; 19],
            transfer_to_address: addr21(),
            ..Default::default()
        };
        let res: Result<domain::transaction::InternalTransaction, _> =
            bad.try_into();
        assert!(matches!(res, Err(ProtoConvError::BadAddress { .. })));
    }
}
