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
            start_time: tron_to_datetime(value.start_time),
            end_time: tron_to_datetime(value.end_time),
            order: value.order,
            vote_score: value.vote_score,
            description: value.description.into(),
            url: value.url.into(),
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: tron_to_datetime(
                value.public_latest_free_net_time,
            ),
        }
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
            start_time: datetime_to_tron(value.start_time),
            end_time: datetime_to_tron(value.end_time),
            order: value.order,
            vote_score: value.vote_score,
            description: value.description.into(),
            url: value.url.into(),
            free_asset_net_limit: value.free_asset_net_limit,
            public_free_asset_net_limit: value.public_free_asset_net_limit,
            public_free_asset_net_usage: value.public_free_asset_net_usage,
            public_latest_free_net_time: datetime_to_tron(
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

impl From<UnfreezeAssetContract> for domain::contract::UnfreezeAssetContract {
    fn from(value: UnfreezeAssetContract) -> Self {
        domain::contract::UnfreezeAssetContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<domain::contract::UnfreezeAssetContract> for UnfreezeAssetContract {
    fn from(value: domain::contract::UnfreezeAssetContract) -> Self {
        UnfreezeAssetContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
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

impl From<domain::contract::AccountCreateContract> for AccountCreateContract {
    fn from(value: domain::contract::AccountCreateContract) -> Self {
        AccountCreateContract {
            r#type: AccountType::from(value.account_type).into(),
            owner_address: value.owner_address.as_bytes().to_vec(),
            account_address: value
                .account_address
                .as_bytes()
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

impl From<domain::contract::AccountUpdateContract> for AccountUpdateContract {
    fn from(value: domain::contract::AccountUpdateContract) -> Self {
        AccountUpdateContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
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

impl From<domain::contract::SetAccountIdContract> for SetAccountIdContract {
    fn from(value: domain::contract::SetAccountIdContract) -> Self {
        SetAccountIdContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
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

impl From<WitnessCreateContract> for domain::contract::WitnessCreateContract {
    fn from(value: WitnessCreateContract) -> Self {
        domain::contract::WitnessCreateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            url: value.url.into(),
        }
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

impl From<WitnessUpdateContract> for domain::contract::WitnessUpdateContract {
    fn from(value: WitnessUpdateContract) -> Self {
        domain::contract::WitnessUpdateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            update_url: value.update_url.into(),
        }
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

impl From<vote_witness_contract::Vote> for domain::contract::Vote {
    fn from(value: vote_witness_contract::Vote) -> Self {
        domain::contract::Vote {
            vote_address: value.vote_address.as_slice().try_into().unwrap(),
            vote_count: value.vote_count,
        }
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

impl From<VoteWitnessContract> for domain::contract::VoteWitnessContract {
    fn from(value: VoteWitnessContract) -> Self {
        domain::contract::VoteWitnessContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            votes: value.votes.into_iter().map(Into::into).collect(),
            support: value.support,
        }
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

impl From<domain::contract::FreezeBalanceContract> for FreezeBalanceContract {
    fn from(value: domain::contract::FreezeBalanceContract) -> Self {
        FreezeBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            frozen_balance: value.frozen_balance.into(),
            frozen_duration: value.frozen_duration.whole_days(),
            resource: ResourceCode::from(value.resource).into(),
            receiver_address: value
                .receiver_address
                .as_bytes()
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
                .unwrap_or_default(),
        }
    }
}

impl From<domain::contract::UnfreezeBalanceContract>
    for UnfreezeBalanceContract
{
    fn from(value: domain::contract::UnfreezeBalanceContract) -> Self {
        UnfreezeBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            receiver_address: value
                .receiver_address
                .as_bytes()
                .try_into()
                .unwrap_or_default(),
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

impl From<domain::contract::WithdrawBalanceContract>
    for WithdrawBalanceContract
{
    fn from(value: domain::contract::WithdrawBalanceContract) -> Self {
        WithdrawBalanceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
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

impl From<domain::contract::TransferContract> for TransferContract {
    fn from(value: domain::contract::TransferContract) -> Self {
        TransferContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            to_address: value.to_address.as_bytes().to_vec(),
            amount: value.amount.into(),
        }
    }
}

impl From<transaction_balance_trace::Operation>
    for domain::contract::Operation
{
    fn from(value: transaction_balance_trace::Operation) -> Self {
        domain::contract::Operation {
            operation_identifier: value.operation_identifier,
            address: value.address.as_slice().try_into().unwrap_or_default(),
            amount: value.amount.into(),
        }
    }
}

impl From<domain::contract::Operation>
    for transaction_balance_trace::Operation
{
    fn from(value: domain::contract::Operation) -> Self {
        transaction_balance_trace::Operation {
            operation_identifier: value.operation_identifier,
            address: value.address.as_bytes().try_into().unwrap_or_default(),
            amount: value.amount.into(),
        }
    }
}

impl From<TransactionBalanceTrace>
    for domain::contract::TransactionBalanceTrace
{
    fn from(value: TransactionBalanceTrace) -> Self {
        domain::contract::TransactionBalanceTrace {
            transaction_identifier: value.transaction_identifier.into(),
            operation: value.operation.into_iter().map(Into::into).collect(),
            r#type: value.r#type,
            status: value.status,
        }
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

impl From<BlockBalanceTrace> for domain::contract::BlockBalanceTrace {
    fn from(value: BlockBalanceTrace) -> Self {
        domain::contract::BlockBalanceTrace {
            block_identifier: value.block_identifier.unwrap_or_default().into(),
            timestamp: tron_to_datetime(value.timestamp),
            transaction_balance_trace: value
                .transaction_balance_trace
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<domain::contract::BlockBalanceTrace> for BlockBalanceTrace {
    fn from(value: domain::contract::BlockBalanceTrace) -> Self {
        BlockBalanceTrace {
            block_identifier: Some(value.block_identifier.into()),
            timestamp: datetime_to_tron(value.timestamp),
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

impl From<AccountIdentifier> for domain::contract::AccountIdentifier {
    fn from(value: AccountIdentifier) -> Self {
        domain::contract::AccountIdentifier {
            address: value.address.as_slice().try_into().unwrap(),
        }
    }
}

impl From<domain::contract::AccountIdentifier> for AccountIdentifier {
    fn from(value: domain::contract::AccountIdentifier) -> Self {
        AccountIdentifier {
            address: value.address.as_bytes().to_vec(),
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

impl From<WithdrawExpireUnfreezeContract>
    for domain::contract::WithdrawExpireUnfreezeContract
{
    fn from(value: WithdrawExpireUnfreezeContract) -> Self {
        domain::contract::WithdrawExpireUnfreezeContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
        }
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

impl From<domain::contract::DelegateResourceContract>
    for DelegateResourceContract
{
    fn from(value: domain::contract::DelegateResourceContract) -> Self {
        DelegateResourceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            balance: value.balance.into(),
            receiver_address: value
                .receiver_address
                .as_bytes()
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

impl From<domain::contract::UnDelegateResourceContract>
    for UnDelegateResourceContract
{
    fn from(value: domain::contract::UnDelegateResourceContract) -> Self {
        UnDelegateResourceContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            resource: ResourceCode::from(value.resource).into(),
            balance: value.balance.into(),
            receiver_address: value
                .receiver_address
                .as_bytes()
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

impl From<domain::contract::CancelAllUnfreezeV2Contract>
    for CancelAllUnfreezeV2Contract
{
    fn from(value: domain::contract::CancelAllUnfreezeV2Contract) -> Self {
        CancelAllUnfreezeV2Contract {
            owner_address: value.owner_address.as_bytes().to_vec(),
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

impl From<ProposalCreateContract> for domain::contract::ProposalCreateContract {
    fn from(value: ProposalCreateContract) -> Self {
        domain::contract::ProposalCreateContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            parameters: value.parameters,
        }
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

impl From<ProposalDeleteContract> for domain::contract::ProposalDeleteContract {
    fn from(value: ProposalDeleteContract) -> Self {
        domain::contract::ProposalDeleteContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            proposal_id: value.proposal_id,
        }
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

impl From<BuyStorageContract> for domain::contract::BuyStorageContract {
    fn from(value: BuyStorageContract) -> Self {
        domain::contract::BuyStorageContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            quant: value.quant.into(),
        }
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

impl From<SellStorageContract> for domain::contract::SellStorageContract {
    fn from(value: SellStorageContract) -> Self {
        domain::contract::SellStorageContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            storage_bytes: value.storage_bytes,
        }
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

impl From<SmartContract> for domain::contract::SmartContract {
    fn from(value: SmartContract) -> Self {
        domain::contract::SmartContract {
            origin_address: value.origin_address.as_slice().try_into().unwrap(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap_or_default(),
            abi: value.abi.unwrap_or_default().into(),
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

impl From<domain::contract::SmartContract> for SmartContract {
    fn from(value: domain::contract::SmartContract) -> Self {
        SmartContract {
            origin_address: value.origin_address.as_bytes().to_vec(),
            contract_address: value
                .contract_address
                .as_bytes()
                .try_into()
                .unwrap_or_default(),
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

impl From<CreateSmartContract> for domain::contract::CreateSmartContract {
    fn from(value: CreateSmartContract) -> Self {
        domain::contract::CreateSmartContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            new_contract: value.new_contract.unwrap_or_default().into(),
            call_token_value: value.call_token_value.into(),
            token_id: value.token_id,
        }
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

impl From<ClearAbiContract> for domain::contract::ClearAbiContract {
    fn from(value: ClearAbiContract) -> Self {
        domain::contract::ClearAbiContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<domain::contract::ClearAbiContract> for ClearAbiContract {
    fn from(value: domain::contract::ClearAbiContract) -> Self {
        ClearAbiContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value
                .contract_address
                .as_bytes()
                .try_into()
                .unwrap(),
        }
    }
}

impl From<UpdateSettingContract> for domain::contract::UpdateSettingContract {
    fn from(value: UpdateSettingContract) -> Self {
        domain::contract::UpdateSettingContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap(),
            consume_user_resource_percent: value.consume_user_resource_percent,
        }
    }
}

impl From<domain::contract::UpdateSettingContract> for UpdateSettingContract {
    fn from(value: domain::contract::UpdateSettingContract) -> Self {
        UpdateSettingContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value
                .contract_address
                .as_bytes()
                .try_into()
                .unwrap(),
            consume_user_resource_percent: value.consume_user_resource_percent,
        }
    }
}

impl From<UpdateEnergyLimitContract>
    for domain::contract::UpdateEnergyLimitContract
{
    fn from(value: UpdateEnergyLimitContract) -> Self {
        domain::contract::UpdateEnergyLimitContract {
            owner_address: value.owner_address.as_slice().try_into().unwrap(),
            contract_address: value
                .contract_address
                .as_slice()
                .try_into()
                .unwrap(),
            origin_energy_limit: value.origin_energy_limit,
        }
    }
}

impl From<domain::contract::UpdateEnergyLimitContract>
    for UpdateEnergyLimitContract
{
    fn from(value: domain::contract::UpdateEnergyLimitContract) -> Self {
        UpdateEnergyLimitContract {
            owner_address: value.owner_address.as_bytes().to_vec(),
            contract_address: value
                .contract_address
                .as_bytes()
                .try_into()
                .unwrap(),
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

impl From<ShieldedTransferContract>
    for domain::contract::ShieldedTransferContract
{
    fn from(value: ShieldedTransferContract) -> Self {
        domain::contract::ShieldedTransferContract {
            transparent_from_address: value
                .transparent_from_address
                .as_slice()
                .try_into()
                .unwrap(),
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
                .as_slice()
                .try_into()
                .unwrap(),
            to_amount: value.to_amount,
        }
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
                .try_into()
                .unwrap(),
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
                .try_into()
                .unwrap(),
            to_amount: value.to_amount,
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

                let (contract_type, parameter) = match c.contract_type {
                    $(
                        ContractType::$variant(inner) => {
                            let tx_contract: $variant = inner.into();
                            let mut buf = Vec::new();
                            tx_contract.encode(&mut buf).unwrap();
                            (transaction::contract::ContractType::$variant, Some(buf))
                        }
                    )+
                    other => {
                        tracing::warn!("Unsupported contract type conversion: {:?}", other);
                        (transaction::contract::ContractType::from_i32(other.id()).unwrap(), None)
                    }
                };

                transaction::Contract {
                    parameter: parameter.map(|value| prost_types::Any {
                        type_url: format!("type.googleapis.com/protocol.{}", contract_type.as_str_name()),
                        value,
                    }),
                    r#type: contract_type.into(),
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
