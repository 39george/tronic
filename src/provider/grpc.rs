use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;

use bon::Builder;
use eyre::eyre;
use http::Uri;

use crate::Result;
use crate::client::Auth;
use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::transaction::ContractResult;
use crate::domain::trx::{self, Trx};
use crate::domain::{self, Hash32};
use crate::error::Error;
use crate::protocol;
use crate::protocol::wallet_client::WalletClient;
use crate::protocol::wallet_solidity_client::WalletSolidityClient;
use crate::provider::TxInfoConvError;
use crate::provider::grpc::middleware::auth_channel;

#[derive(Clone, Builder)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct ConnectOptions {
    #[builder(into)]
    pub auth: Option<Auth>,

    #[builder(into)]
    pub rate_limit: Option<super::RateLimit>,

    /// Optional endpoint of the `WalletSolidity` gRPC service.
    ///
    /// On a self-hosted java-tron FullNode this is `rpc.solidityPort`
    /// (default 50061, served by the same node); on TronGrid mainnet the
    /// solidity gRPC historically lives on :50052, so the port must be
    /// configured explicitly rather than derived.
    ///
    /// When set, `get_solid_block_number` uses `WalletSolidity.GetNowBlock2`
    /// over a second channel built with the same auth / rate limit / keepalive
    /// options. When unset, it falls back to `Wallet.GetNodeInfo` and parses
    /// the `solidity_block` field (`Num:<n>,ID:<hash>`).
    #[builder(into)]
    pub solidity_uri: Option<String>,

    pub http2_keep_alive_interval: Option<Duration>,
    pub keep_alive_timeout: Option<Duration>,
    pub keep_alive_while_idle: Option<bool>,
    pub tcp_keepalive: Option<Duration>,
    pub connect_timeout: Option<Duration>,
    pub concurrency_limit: Option<usize>,
    pub retry_enabled: Option<bool>,
}

impl<State: connect_options_builder::IsComplete> ConnectOptionsBuilder<State> {
    pub async fn connect(self, uri: impl AsRef<str>) -> Result<GrpcProvider> {
        let opts = self.build_internal();

        let channel = build_auth_channel(uri.as_ref(), &opts).await?;
        let solidity_channel = match opts.solidity_uri.as_deref() {
            Some(solidity_uri) => {
                Some(build_auth_channel(solidity_uri, &opts).await?)
            }
            None => None,
        };

        Ok(GrpcProvider {
            channel,
            solidity_channel,
            retry_enabled: opts.retry_enabled.unwrap_or(true),
        })
    }
}

/// Builds a channel with `opts`.
/// Reused for the main `Wallet` endpoint and the optional `WalletSolidity`
/// endpoint, so both share identical transport behaviour.
async fn build_auth_channel(
    uri: &str,
    opts: &ConnectOptions,
) -> Result<middleware::AuthChannel> {
    let uri: Uri = uri.parse()?;
    let scheme = uri.scheme().cloned();

    #[allow(unused_mut)]
    let mut builder = tonic::transport::Channel::builder(uri)
        .tcp_keepalive(opts.tcp_keepalive);

    if let Some(super::RateLimit { limit, duration }) = opts.rate_limit.clone()
    {
        builder = builder.rate_limit(limit, duration);
    }
    if let Some(param) = opts.http2_keep_alive_interval {
        builder = builder.http2_keep_alive_interval(param);
    }
    if let Some(param) = opts.keep_alive_timeout {
        builder = builder.keep_alive_timeout(param);
    }
    if let Some(param) = opts.keep_alive_while_idle {
        builder = builder.keep_alive_while_idle(param);
    }
    if let Some(param) = opts.connect_timeout {
        builder = builder.connect_timeout(param);
    }
    if let Some(param) = opts.concurrency_limit {
        builder = builder.concurrency_limit(param);
    }

    #[cfg(not(feature = "tonic-tls"))]
    if scheme.is_some_and(|s| s.eq(&http::uri::Scheme::HTTPS)) {
        return Err(Error::Unexpected(eyre!(
            "enable tonic-tls feature to use https"
        )));
    }
    #[cfg(feature = "tonic-tls")]
    if scheme.is_some_and(|s| s.eq(&http::uri::Scheme::HTTPS)) {
        let _ = rustls::crypto::CryptoProvider::install_default(
            rustls::crypto::ring::default_provider(),
        );

        builder = builder.tls_config(
            tonic::transport::ClientTlsConfig::new().with_native_roots(),
        )?;
    }
    let channel = builder.connect().await?;
    Ok(auth_channel(
        channel,
        match opts.auth.clone() {
            Some(Auth::Bearer { name, secret }) => Some(middleware::BHAuth {
                bearer_name: name.parse()?,
                bearer_secret: secret,
            }),
            None => None,
        },
    ))
}

#[derive(Clone)]
pub struct GrpcProvider {
    channel: middleware::AuthChannel,
    solidity_channel: Option<middleware::AuthChannel>,
    retry_enabled: bool,
}

fn parse_solidity_block_num(s: &str) -> Result<i64> {
    s.split(',')
        .find_map(|part| part.trim().strip_prefix("Num:"))
        .and_then(|n| n.trim().parse::<i64>().ok())
        .ok_or_else(|| {
            Error::InvalidResponse(format!(
                "cannot parse NodeInfo.solidity_block: {s:?}"
            ))
        })
}

impl GrpcProvider {
    pub fn builder() -> ConnectOptionsBuilder {
        ConnectOptions::builder()
    }
    fn wallet_client(&self) -> WalletClient<middleware::AuthChannel> {
        WalletClient::new(self.channel.clone())
    }
    fn return_to_result(ret: Option<protocol::Return>) -> Result<()> {
        if let Some(protocol::Return {
            result: false,
            code,
            message,
        }) = ret
        {
            let code = protocol::r#return::ResponseCode::try_from(code)
                .unwrap_or(protocol::r#return::ResponseCode::OtherError);
            Err(Error::NodeRejected {
                code: code.into(),
                message: String::from_utf8_lossy(&message).into_owned(),
            })
        } else {
            Ok(())
        }
    }
    async fn retry_grpc<T, Fut, F>(
        &self,
        op: &'static str,
        mut f: F,
    ) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = std::result::Result<T, tonic::Status>>,
    {
        use tonic::Code;
        fn is_retryable_grpc_error(status: &tonic::Status) -> bool {
            matches!(
                status.code(),
                Code::Unavailable
                    | Code::DeadlineExceeded
                    | Code::ResourceExhausted
                    | Code::Unknown
                    | Code::Internal
            )
        }

        if !self.retry_enabled {
            return f().await.map_err(Into::into);
        }

        let mut delay = Duration::from_millis(200);
        let max_attempts = 3;

        for attempt in 1..=max_attempts {
            match f().await {
                Ok(value) => return Ok(value),

                Err(status)
                    if is_retryable_grpc_error(&status)
                        && attempt < max_attempts =>
                {
                    tracing::warn!(
                        op,
                        attempt,
                        code = ?status.code(),
                        error = %status,
                        "grpc request failed, retrying"
                    );

                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }

                Err(status) => {
                    return Err(status.into());
                }
            }
        }

        unreachable!()
    }
}

#[async_trait::async_trait]
impl crate::provider::TronProvider for GrpcProvider {
    async fn transfer_contract(
        &self,
        owner: domain::address::TronAddress,
        to: domain::address::TronAddress,
        amount: trx::Trx,
    ) -> Result<domain::transaction::TransactionExtention> {
        let txext = self
            .retry_grpc("transfer_contract", || {
                let grpc_transfer_contract = protocol::TransferContract {
                    owner_address: owner.as_bytes().to_vec(),
                    to_address: to.as_bytes().to_vec(),
                    amount: amount.to_sun(),
                };
                let mut node = self.wallet_client();
                async move {
                    node.create_transaction2(grpc_transfer_contract)
                        .await
                }
            })
            .await?.into_inner();
        if txext.txid.is_empty() {
            if let Some(ref r) = txext.result
                && r.message == b"Contract validate error : Validate TransferContract error, no OwnerAccount."
            {
                return Err(Error::NoAccount(owner))
            }
            Err(Error::Unexpected(eyre!(
                "txid is empty: {}",
                String::from_utf8_lossy(
                    &txext.result.unwrap_or_default().message
                )
            )))
        } else {
            Ok(txext.try_into()?)
        }
    }
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention> {
        let call = call.encode();
        let reply = self
            .retry_grpc("trigger_smart_contract", || {
                let contract = protocol::TriggerSmartContract {
                    owner_address: owner.as_bytes().to_vec(),
                    contract_address: contract.as_bytes().to_vec(),
                    data: call.clone(),
                    ..Default::default()
                };
                let mut node = self.wallet_client();
                async move { node.trigger_contract(contract).await }
            })
            .await
            .map(|r| r.into_inner())?;
        Self::return_to_result(reply.result.clone())?;
        Ok(reply.try_into()?)
    }
    async fn broadcast_transaction(
        &self,
        transaction: domain::transaction::Transaction,
    ) -> Result<()> {
        let mut node = self.wallet_client();
        let transaction: protocol::Transaction = transaction.into();
        let response =
            node.broadcast_transaction(transaction).await?.into_inner();
        Self::return_to_result(Some(response))?;
        Ok(())
    }
    async fn estimate_energy(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<i64> {
        let contract: protocol::TriggerSmartContract = contract.into();

        let msg = self
            .retry_grpc("estimate_energy", || {
                let mut node = self.wallet_client();
                let contract = contract.clone();

                async move { node.estimate_energy(contract).await }
            })
            .await?
            .into_inner();

        Self::return_to_result(msg.result.clone())?;
        Ok(msg.energy_required)
    }

    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account> {
        let account = protocol::Account {
            address: address.as_bytes().to_vec(),
            ..Default::default()
        };

        let account = self
            .retry_grpc("get_account", || {
                let mut node = self.wallet_client();
                let account = account.clone();

                async move { node.get_account(account).await }
            })
            .await?
            .into_inner()
            .try_into()?;

        Ok(account)
    }

    async fn get_account_resources(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage> {
        let account = protocol::Account {
            address: address.as_bytes().to_vec(),
            ..Default::default()
        };

        let account_resource = self
            .retry_grpc("get_account_resources", || {
                let mut node = self.wallet_client();
                let account = account.clone();

                async move { node.get_account_resource(account).await }
            })
            .await?
            .into_inner();

        Ok(account_resource.into())
    }

    /// Three distinct outcomes:
    /// 1. `Error::Transport` / `Error::TronProtocol` — network / gRPC level,
    ///    retryable (and retried by `retry_grpc` where applicable);
    /// 2. `Error::NodeRejected` — the node refused before execution
    ///    (bad address, no contract deployed, etc)
    /// 3. `Error::ContractReverted` — the node executed the call, but the
    ///    contract reverted. Detected by either `Return.code ==
    ///    CONTRACT_EXE_ERROR` *or* `transaction.ret[0].contract_ret` not in
    ///    {SUCCESS, DEFAULT}. Which of the two actually fires on revert must
    ///    be verified on nile (contract with `revert()` in `name()`); both
    ///    are checked here so either shape maps to the right variant.
    ///
    /// A successful call with an empty `constant_result` is *not* an error
    /// of tronic: `Ok` with an empty vector is returned, the caller decides.
    async fn trigger_constant_contract(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract: protocol::TriggerSmartContract = contract.into();

        let txext = self
            .retry_grpc("trigger_constant_contract", || {
                let mut node = self.wallet_client();
                let contract = contract.clone();

                async move { node.trigger_constant_contract(contract).await }
            })
            .await?
            .into_inner();

        if let Some(ref ret) = txext.result
            && !ret.result
        {
            let code = protocol::r#return::ResponseCode::try_from(ret.code)
                .unwrap_or(protocol::r#return::ResponseCode::OtherError);
            let message = String::from_utf8_lossy(&ret.message).into_owned();
            if code == protocol::r#return::ResponseCode::ContractExeError {
                return Err(Error::ContractReverted {
                    message,
                    data: txext
                        .constant_result
                        .into_iter()
                        .next()
                        .unwrap_or_default(),
                });
            }
            return Err(Error::NodeRejected {
                code: code.into(),
                message,
            });
        }

        let txext: domain::transaction::TransactionExtention =
            txext.try_into()?;

        if let Some(contract_ret) = txext
            .transaction
            .as_ref()
            .and_then(|t| t.result.first())
            .map(|r| r.contract_ret.clone())
            && !matches!(
                contract_ret,
                ContractResult::Success | ContractResult::Default
            )
        {
            return Err(Error::ContractReverted {
                message: format!("{contract_ret:?}"),
                data: txext
                    .constant_result
                    .into_iter()
                    .next()
                    .unwrap_or_default(),
            });
        }

        Ok(txext)
    }

    async fn get_now_block(&self) -> Result<domain::block::BlockExtention> {
        let now_block = self
            .retry_grpc("get_now_block", || {
                let mut node = self.wallet_client();

                async move {
                    node.get_now_block2(protocol::EmptyMessage::default()).await
                }
            })
            .await?
            .into_inner();

        Ok(now_block.try_into()?)
    }

    async fn get_block_by_number(
        &self,
        block_num: i64,
    ) -> Result<domain::block::BlockExtention> {
        let block = self
            .retry_grpc("get_block_by_number", || {
                let mut node = self.wallet_client();

                async move {
                    node.get_block_by_num2(protocol::NumberMessage {
                        num: block_num,
                    })
                    .await
                }
            })
            .await?
            .into_inner();

        Ok(block.try_into()?)
    }

    async fn get_transaction_info_by_block_num(
        &self,
        block_num: i64,
    ) -> Result<
        Vec<
            std::result::Result<
                domain::transaction::TransactionInfo,
                TxInfoConvError,
            >,
        >,
    > {
        let list = self
            .retry_grpc("get_transaction_info_by_block_num", || {
                let mut node = self.wallet_client();

                async move {
                    node.get_transaction_info_by_block_num(
                        protocol::NumberMessage { num: block_num },
                    )
                    .await
                }
            })
            .await?
            .into_inner();

        Ok(list
            .transaction_info
            .into_iter()
            .map(|ti| {
                let txid: Hash32 = ti.id.clone().try_into().unwrap_or_default();
                domain::transaction::TransactionInfo::try_from(ti)
                    .map_err(|source| TxInfoConvError { txid, source })
            })
            .collect())
    }

    async fn get_solid_block_number(&self) -> Result<i64> {
        if let Some(solidity) = self.solidity_channel.clone() {
            let block = self
                .retry_grpc("get_solid_block_number", || {
                    let mut node = WalletSolidityClient::new(solidity.clone());
                    async move {
                        node.get_now_block2(protocol::EmptyMessage::default())
                            .await
                    }
                })
                .await?
                .into_inner();

            return block
                .block_header
                .and_then(|h| h.raw_data)
                .map(|raw| raw.number)
                .ok_or_else(|| {
                    Error::InvalidResponse(
                        "solidity node returned a block without header".into(),
                    )
                });
        }

        let info = self
            .retry_grpc("get_node_info", || {
                let mut node = self.wallet_client();

                async move {
                    node.get_node_info(protocol::EmptyMessage::default()).await
                }
            })
            .await?
            .into_inner();

        parse_solidity_block_num(&info.solidity_block)
    }

    async fn account_permission_update(
        &self,
        contract: domain::contract::AccountPermissionUpdateContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract: protocol::AccountPermissionUpdateContract =
            contract.into();

        let txext = self
            .retry_grpc("account_permission_update", || {
                let mut node = self.wallet_client();
                let contract = contract.clone();

                async move { node.account_permission_update(contract).await }
            })
            .await?
            .into_inner();

        Self::return_to_result(txext.result.clone())?;
        Ok(txext.try_into()?)
    }

    async fn get_transaction_by_id(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::Transaction> {
        let txid = protocol::BytesMessage::from(txid);

        let transaction = self
            .retry_grpc("get_transaction_by_id", || {
                let mut node = self.wallet_client();
                let txid = txid.clone();

                async move { node.get_transaction_by_id(txid).await }
            })
            .await?
            .into_inner();

        Ok(transaction.try_into()?)
    }

    async fn get_transaction_info(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::TransactionInfo> {
        let txid = protocol::BytesMessage::from(txid);

        let transaction = self
            .retry_grpc("get_transaction_info", || {
                let mut node = self.wallet_client();
                let txid = txid.clone();

                async move { node.get_transaction_info_by_id(txid).await }
            })
            .await?
            .into_inner();

        Ok(transaction.try_into()?)
    }

    async fn chain_parameters(&self) -> Result<HashMap<String, i64>> {
        let chain_parameters = self
            .retry_grpc("chain_parameters", || {
                let mut node = self.wallet_client();

                async move {
                    node.get_chain_parameters(protocol::EmptyMessage::default())
                        .await
                }
            })
            .await?
            .into_inner()
            .chain_parameter
            .into_iter()
            .map(|ch_p| (ch_p.key, ch_p.value))
            .collect::<HashMap<_, _>>();

        Ok(chain_parameters)
    }

    async fn freeze_balance(
        &self,
        contract: domain::contract::FreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract: protocol::FreezeBalanceV2Contract = contract.into();

        let txext = self
            .retry_grpc("freeze_balance", || {
                let mut node = self.wallet_client();
                let contract = contract.clone();

                async move { node.freeze_balance_v2(contract).await }
            })
            .await?
            .into_inner();

        Self::return_to_result(txext.result.clone())?;
        Ok(txext.try_into()?)
    }

    async fn unfreeze_balance(
        &self,
        contract: domain::contract::UnfreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract: protocol::UnfreezeBalanceV2Contract = contract.into();

        let txext = self
            .retry_grpc("unfreeze_balance", || {
                let mut node = self.wallet_client();
                let contract = contract.clone();

                async move { node.unfreeze_balance_v2(contract).await }
            })
            .await?
            .into_inner();

        Self::return_to_result(txext.result.clone())?;
        Ok(txext.try_into()?)
    }

    async fn get_reward(&self, address: TronAddress) -> Result<Trx> {
        let message = protocol::BytesMessage {
            value: address.as_bytes().to_vec(),
        };

        let number = self
            .retry_grpc("get_reward", || {
                let mut node = self.wallet_client();
                let message = message.clone();

                async move { node.get_reward_info(message).await }
            })
            .await?
            .into_inner();

        Ok(number.num.into())
    }

    async fn get_delegated_resource(
        &self,
        from_address: TronAddress,
        to_address: TronAddress,
    ) -> Result<Vec<domain::account::DelegatedResource>> {
        let message = protocol::DelegatedResourceMessage {
            from_address: from_address.as_bytes().to_vec(),
            to_address: to_address.as_bytes().to_vec(),
        };

        let list = self
            .retry_grpc("get_delegated_resource", || {
                let mut node = self.wallet_client();
                let message = message.clone();

                async move { node.get_delegated_resource_v2(message).await }
            })
            .await?
            .into_inner()
            .delegated_resource;

        Ok(list
            .into_iter()
            .map(TryInto::<domain::account::DelegatedResource>::try_into)
            .collect::<std::result::Result<Vec<_>, _>>()?)
    }

    async fn get_delegated_resource_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::DelegatedResourceAccountIndex> {
        let message = protocol::BytesMessage {
            value: address.as_bytes().to_vec(),
        };

        let index = self
            .retry_grpc("get_delegated_resource_account", || {
                let mut node = self.wallet_client();
                let message = message.clone();

                async move {
                    node.get_delegated_resource_account_index_v2(message).await
                }
            })
            .await?
            .into_inner();

        Ok(index.try_into()?)
    }
    async fn get_contract(
        &self,
        address: TronAddress,
    ) -> Result<domain::contract::SmartContract> {
        let message = protocol::BytesMessage {
            value: address.as_bytes().to_vec(),
        };

        let contract = self
            .retry_grpc("get_contract", || {
                let mut node = self.wallet_client();
                let message = message.clone();
                async move { node.get_contract(message).await }
            })
            .await?
            .into_inner();

        if contract.origin_address.is_empty() {
            return Err(Error::NotFound(format!(
                "contract {address} not found"
            )));
        }

        Ok(contract.try_into()?)
    }
}

pub mod middleware {
    use http::HeaderName;
    use secrecy::SecretString;
    use tonic::transport::Channel;
    use tower::ServiceBuilder;

    pub use service::AuthChannel;

    #[derive(Clone)]
    pub struct BHAuth {
        pub bearer_name: HeaderName,
        pub bearer_secret: SecretString,
    }

    pub fn auth_channel(channel: Channel, auth: Option<BHAuth>) -> AuthChannel {
        ServiceBuilder::new()
            .layer(service::AuthChLayer::new(auth))
            .service(channel)
    }

    mod service {
        use http::{HeaderValue, Request, Response};
        use secrecy::ExposeSecret;
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use tonic::body::Body;
        use tonic::transport::Channel;
        use tower::{Layer, Service};

        use super::BHAuth;

        #[derive(Clone)]
        pub struct AuthChannel {
            inner: Channel,
            info: Option<BHAuth>,
        }

        pub struct AuthChLayer {
            info: Option<BHAuth>,
        }

        impl AuthChLayer {
            pub fn new(info: Option<BHAuth>) -> AuthChLayer {
                Self { info }
            }
        }

        impl Layer<Channel> for AuthChLayer {
            type Service = AuthChannel;
            fn layer(&self, inner: Channel) -> Self::Service {
                AuthChannel {
                    inner,
                    info: self.info.clone(),
                }
            }
        }

        impl Service<Request<Body>> for AuthChannel {
            type Response = Response<Body>;
            type Error = Box<dyn std::error::Error + Send + Sync>;
            type Future = Pin<
                Box<
                    dyn Future<Output = Result<Self::Response, Self::Error>>
                        + Send,
                >,
            >;

            fn poll_ready(
                &mut self,
                cx: &mut Context<'_>,
            ) -> Poll<Result<(), Self::Error>> {
                self.inner.poll_ready(cx).map_err(Into::into)
            }

            fn call(&mut self, mut req: Request<Body>) -> Self::Future {
                // See: https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
                let clone = self.inner.clone();
                let mut inner = std::mem::replace(&mut self.inner, clone);
                let info = self.info.clone();

                Box::pin(async move {
                    if let Some(BHAuth {
                        bearer_name,
                        bearer_secret,
                    }) = info
                    {
                        match HeaderValue::from_str(
                            bearer_secret.expose_secret(),
                        ) {
                            Ok(secret) => {
                                req.headers_mut().insert(bearer_name, secret);
                            }
                            Err(e) => tracing::error!(
                                "failed to authorize grpc request: {e}"
                            ),
                        }
                    }
                    let response = inner.call(req).await?;
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(response)
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_to_result_maps_every_response_code() {
        use protocol::r#return::ResponseCode as P;

        let codes = [
            P::Success,
            P::Sigerror,
            P::ContractValidateError,
            P::ContractExeError,
            P::BandwithError,
            P::DupTransactionError,
            P::TaposError,
            P::TooBigTransactionError,
            P::TransactionExpirationError,
            P::ServerBusy,
            P::NoConnection,
            P::NotEnoughEffectiveConnection,
            P::BlockUnsolidified,
            P::OtherError,
        ];

        for code in codes {
            let ret = protocol::Return {
                result: false,
                code: code as i32,
                message: b"boom".to_vec(),
            };
            let err = GrpcProvider::return_to_result(Some(ret))
                .expect_err("result=false must be an error");
            match err {
                Error::NodeRejected {
                    code: domain_code,
                    message,
                } => {
                    assert_eq!(
                        domain_code,
                        crate::domain::ResponseCode::from(code),
                        "code mismatch for {code:?}"
                    );
                    assert_eq!(message, "boom");
                }
                other => panic!("expected NodeRejected, got {other:?}"),
            }
        }
    }

    #[test]
    fn return_to_result_ok_cases() {
        assert!(GrpcProvider::return_to_result(None).is_ok());
        assert!(
            GrpcProvider::return_to_result(Some(protocol::Return {
                result: true,
                ..Default::default()
            }))
            .is_ok()
        );
    }

    #[test]
    fn node_rejected_display_is_compatible() {
        let err = Error::NodeRejected {
            code: crate::domain::ResponseCode::DupTransactionError,
            message: "dup".into(),
        };
        let s = err.to_string();
        assert!(s.starts_with("failed: dup, code:"), "got: {s}");
        assert!(s.contains("DupTransactionError"), "got: {s}");
    }

    #[test]
    fn parses_solidity_block_num() {
        assert_eq!(
            parse_solidity_block_num(
                "Num:12345,ID:00000000000030390000aabbccdd"
            )
            .unwrap(),
            12345
        );
        assert_eq!(parse_solidity_block_num("Num: 77 , ID:aa").unwrap(), 77);
        assert!(matches!(
            parse_solidity_block_num("garbage"),
            Err(Error::InvalidResponse(_))
        ));
        assert!(matches!(
            parse_solidity_block_num("Num:notanumber,ID:aa"),
            Err(Error::InvalidResponse(_))
        ));
    }
}
