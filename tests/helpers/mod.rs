use std::ops::Deref;

use alloy_primitives::U256;

use tronic::client::pending::AutoSigning;
use tronic::domain::Hash32;
use tronic::domain::transaction::TransactionInfo;
use tronic::provider::TronProvider;
use tronic::{
    client::Client, contracts::token::usdt::Usdt, provider::grpc::GrpcProvider,
    signer::LocalSigner,
};

pub use node::NODE;

mod node;

const MYCOIN_CONTRACT: &str = include_str!("../../tests/assets/MyCoin.json");

pub struct Tronic {
    client: Client<GrpcProvider, LocalSigner>,
    signer: LocalSigner,
}

impl Deref for Tronic {
    type Target = Client<GrpcProvider, LocalSigner>;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Tronic {
    pub async fn new() -> Self {
        let node = NODE.read();
        let acc = node.new_account().await;
        let signer: LocalSigner = acc.into();
        let client = Client::builder()
            .provider(
                GrpcProvider::builder()
                    .connect(node.grpc_addr())
                    .await
                    .unwrap(),
            )
            .signer(signer.clone())
            .build();
        Self { client, signer }
    }

    pub fn signer(&self) -> &LocalSigner {
        &self.signer
    }

    pub async fn deploy_mycoin(
        &self,
        user_resource_percent: i64,
        origin_energy_limit: i64,
    ) -> TransactionInfo {
        let initial_supply = Usdt::from_decimal(10_000_000.0).unwrap();
        let txid = self
            .create_contract(MYCOIN_CONTRACT.into())
            .params(vec![
                &Into::<U256>::into(initial_supply),
                &alloy_primitives::Address::ZERO,
            ])
            .consume_user_resource_percent(user_resource_percent)
            .origin_energy_limit(origin_energy_limit)
            .can_spend_trx_for_fee(true)
            .build::<AutoSigning>()
            .await
            .unwrap()
            .set_expiration(time::Duration::seconds(100))
            .await
            .unwrap()
            .broadcast(&())
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(7)).await;
        self.provider().get_transaction_info(txid).await.unwrap()
    }
}
