use std::collections::HashMap;

use anyhow::anyhow;
use http::Uri;

use crate::Result;
use crate::client::Auth;
use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::trx;
use crate::domain::{self, Hash32};
use crate::error::Error;
use crate::protocol;
use crate::protocol::wallet_client::WalletClient;
use crate::provider::grpc::middleware::auth_channel;

#[derive(Clone)]
pub struct GrpcProvider {
    channel: middleware::AuthChannel,
}

impl GrpcProvider {
    pub async fn new(node_uri: Uri, auth: Auth) -> Result<Self> {
        let channel = tonic::transport::Channel::builder(node_uri)
            .connect()
            .await?;
        let channel = auth_channel(
            channel,
            match auth {
                Auth::Bearer { name, secret } => Some(middleware::BHAuth {
                    bearer_name: name.parse()?,
                    bearer_secret: secret,
                }),
                Auth::None => None,
            },
        );
        Ok(Self { channel })
    }
    pub fn wallet_client(&self) -> WalletClient<middleware::AuthChannel> {
        WalletClient::new(self.channel.clone())
    }
    pub fn return_to_result(ret: Option<protocol::Return>) -> Result<()> {
        if let Some(protocol::Return {
            result: false,
            code: _,
            message,
        }) = ret
        {
            Err(anyhow!("failed: {}", String::from_utf8_lossy(&message)).into())
        } else {
            Ok(())
        }
    }
}

#[async_trait::async_trait]
impl crate::provider::TronProvider for GrpcProvider {
    async fn trasnfer_contract(
        &self,
        owner: domain::address::TronAddress,
        to: domain::address::TronAddress,
        amount: trx::Trx,
    ) -> Result<domain::transaction::TransactionExtention> {
        let grpc_transfer_contract = protocol::TransferContract {
            owner_address: owner.as_bytes().to_vec(),
            to_address: to.as_bytes().to_vec(),
            amount: amount.to_sun(),
        };
        let mut node = self.wallet_client();
        let txext = node
            .create_transaction2(grpc_transfer_contract)
            .await?
            .into_inner();
        if txext.txid.is_empty() {
            if let Some(ref r) = txext.result
                && r.message == b"Contract validate error : Validate TransferContract error, no OwnerAccount."
            {
                return Err(Error::NoAccount(owner))
            }
            Err(Error::Unexpected(anyhow!(
                "txid is empty: {}",
                String::from_utf8_lossy(
                    &txext.result.unwrap_or_default().message
                )
            )))
        } else {
            Ok(txext.into())
        }
    }
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract = protocol::TriggerSmartContract {
            owner_address: owner.as_bytes().to_vec(),
            contract_address: contract.as_bytes().to_vec(),
            data: call.encode(),
            ..Default::default()
        };

        let mut node = self.wallet_client();
        let reply = node
            .trigger_contract(contract)
            .await
            .map(|r| r.into_inner())?;
        Self::return_to_result(reply.result.clone())?;
        Ok(reply.into())
    }
    async fn broadcast_transaction(
        &self,
        transaction: domain::transaction::Transaction,
    ) -> Result<()> {
        let mut node = WalletClient::new(self.channel.clone());
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
        let mut node = self.wallet_client();
        let contract: protocol::TriggerSmartContract = contract.into();
        let msg = node.estimate_energy(contract).await?.into_inner();
        Self::return_to_result(msg.result.clone())?;
        Ok(msg.energy_required)
    }
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account> {
        let mut node = self.wallet_client();
        let account = protocol::Account {
            address: address.as_bytes().to_vec(),
            ..Default::default()
        };
        let account: domain::account::Account =
            node.get_account(account).await?.into_inner().into();
        Ok(account)
    }
    async fn get_account_resources(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage> {
        let mut node = self.wallet_client();
        let account = protocol::Account {
            address: address.as_bytes().to_vec(),
            ..Default::default()
        };
        let account_resource =
            node.get_account_resource(account).await?.into_inner();
        Ok(account_resource.into())
    }
    async fn trigger_constant_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention> {
        let contract = protocol::TriggerSmartContract {
            owner_address: owner.as_bytes().to_vec(),
            contract_address: contract.as_bytes().to_vec(),
            data: call.encode(),
            ..Default::default()
        };

        let mut node = self.wallet_client();
        let txext = node
            .trigger_constant_contract(contract)
            .await
            .map(|r| r.into_inner())?;
        Self::return_to_result(txext.result.clone())?;
        Ok(txext.into())
    }
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention> {
        let mut node = self.wallet_client();
        let now_block = node
            .get_now_block2(protocol::EmptyMessage::default())
            .await?
            .into_inner();
        Ok(now_block.into())
    }
    async fn account_permission_update(
        &self,
        contract: domain::contract::AccountPermissionUpdateContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        let mut node = self.wallet_client();
        let contract: protocol::AccountPermissionUpdateContract =
            contract.into();
        let txext =
            node.account_permission_update(contract).await?.into_inner();
        Self::return_to_result(txext.result.clone())?;
        Ok(txext.into())
    }
    async fn get_transaction_by_id(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::Transaction> {
        let mut node = self.wallet_client();
        let transaction = node
            .get_transaction_by_id(protocol::BytesMessage::from(txid))
            .await?
            .into_inner();
        Ok(transaction.into())
    }
    async fn get_transaction_info(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::TransactionInfo> {
        let mut node = self.wallet_client();
        let transaction = node
            .get_transaction_info_by_id(protocol::BytesMessage::from(txid))
            .await?
            .into_inner();
        Ok(transaction.into())
    }
    async fn chain_parameters(&self) -> Result<HashMap<String, i64>> {
        let mut node = WalletClient::new(self.channel.clone());
        let chain_parameters = node
            .get_chain_parameters(protocol::EmptyMessage::default())
            .await?
            .into_inner()
            .chain_parameter
            .into_iter()
            .map(|ch_p| (ch_p.key, ch_p.value))
            .collect::<HashMap<_, _>>();
        Ok(chain_parameters)
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
