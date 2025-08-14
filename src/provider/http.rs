use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;
use crate::domain::{self, Hash32};
use crate::provider::TronProvider;
use crate::{Result, client::Auth};
use anyhow::Context;
use http::HeaderName;
use reqwest::{
    Client, Url,
    header::{HeaderMap, HeaderValue},
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct HttpProvider {
    client: Client,
    base: Url,
}

impl HttpProvider {
    pub fn new(base: Url, auth: Auth) -> Result<Self> {
        let mut headers = HeaderMap::new();
        if let Auth::Bearer { name, secret } = auth {
            headers.insert(
                HeaderName::from_str(&name)
                    .context("failed to parse bearer header")?,
                HeaderValue::from_str(&secret.expose_secret())?,
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .tcp_nodelay(true)
            .pool_max_idle_per_host(8)
            .http2_adaptive_window(true)
            .build()?;

        Ok(Self { client, base })
    }

    fn url(&self, path: &str) -> Result<Url> {
        Ok(self.base.join(path).context("failed to get call path")?)
    }

    async fn post_json<
        TReq: Serialize + ?Sized,
        TResp: for<'de> Deserialize<'de>,
    >(
        &self,
        path: &str,
        body: &TReq,
    ) -> Result<TResp> {
        let url = self.url(path)?;
        let resp = self
            .client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    async fn get_json<TResp: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<TResp> {
        let url = self.url(path)?;
        let resp = self.client.get(url).send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }
}

#[async_trait::async_trait]
impl TronProvider for HttpProvider {
    async fn trasnfer_contract(
        &self,
        owner: TronAddress,
        to: TronAddress,
        amount: Trx,
    ) -> Result<domain::transaction::TransactionExtention> {
        // REST: /wallet/createtransaction возвращает TransactionExtention
        // self.post_json("/wallet/createtransaction", &req).await
        todo!()
    }
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention> {
        // self.post_json("/wallet/triggersmartcontract", &req).await
        todo!()
    }
    async fn broadcast_transaction(
        &self,
        transaction: domain::transaction::Transaction,
    ) -> Result<()> {
        let () = self
            .post_json(
                "wallet/broadcasttransaction",
                &serde_json::json!({"raw_data": transaction.raw}),
            )
            .await?;
        Ok(())
    }
    async fn estimate_energy(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<i64> {
        todo!()
    }
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account> {
        todo!()
    }
    async fn get_account_resources(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage> {
        todo!()
    }
    async fn trigger_constant_contract(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        todo!()
    }
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention> {
        todo!()
    }
    async fn account_permission_update(
        &self,
        contract: domain::contract::AccountPermissionUpdateContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        todo!()
    }
    async fn get_transaction_by_id(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::Transaction> {
        todo!()
    }
    async fn get_transaction_info(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::TransactionInfo> {
        todo!()
    }
    async fn chain_parameters(&self) -> Result<HashMap<String, i64>> {
        todo!()
    }
    async fn freeze_balance(
        &self,
        contract: domain::contract::FreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        todo!()
    }
    async fn unfreeze_balance(
        &self,
        contract: domain::contract::UnfreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        todo!()
    }
    async fn get_reward(&self, address: TronAddress) -> Result<Trx> {
        todo!()
    }
    async fn get_delegated_resource(
        &self,
        from_address: TronAddress,
        to_address: TronAddress,
    ) -> Result<Vec<domain::account::DelegatedResource>> {
        todo!()
    }
    async fn get_delegated_resource_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::DelegatedResourceAccountIndex> {
        todo!()
    }

    // async fn calculate_fee(&self, transaction: &Transaction) -> Result<Fee>;

    //     async fn get_contract_abi(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractAbi>;

    // async fn get_contract_info(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractInfo>;

    // async fn get_block_by_number(&self, block_num: i64) -> Result<Block>;

    // async fn get_block_by_id(&self, block_id: [u8; 32]) -> Result<Block>;

    // async fn get_token_info(&self, token_id: String) -> Result<TokenInfo>;

    // async fn get_token_list(&self) -> Result<Vec<TokenInfo>>;

    // async fn get_chain_parameters(&self) -> Result<Vec<ChainParameter>>;

    // async fn get_node_info(&self) -> Result<NodeInfo>;
}
