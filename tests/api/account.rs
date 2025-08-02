use tronic::client::pending::AutoSigning;
use tronic::{provider::TronProvider, trx};

use crate::helpers::Tronic;

fn available_bandwidth(staked: i64, total_staked: i64) -> i64 {
    (staked / total_staked) * 43_200_000_000
}

fn available_energy(staked: i64, total_staked: i64) -> i64 {
    (staked / total_staked) * 180_000_000_000
}

#[tokio::test]
async fn freeze_bandwidth() {
    let tronic = Tronic::new().await;
    let address = tronic.signer_address().unwrap();
    let _ = tronic
        .freeze_balance()
        .amount(trx!(100.0 TRX))
        .resource(tronic::domain::contract::ResourceCode::Bandwidth)
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast(&())
        .await
        .unwrap();

    let resources = tronic
        .provider()
        .get_account_resources(address)
        .await
        .unwrap();
    let bandwidth =
        available_bandwidth(resources.net_limit, resources.total_net_limit);
    assert_eq!(bandwidth, 43_200_000_000);
}

#[tokio::test]
async fn freeze_energy() {
    let tronic = Tronic::new().await;
    let address = tronic.signer_address().unwrap();
    let _ = tronic
        .freeze_balance()
        .amount(trx!(100.0 TRX))
        .resource(tronic::domain::contract::ResourceCode::Energy)
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast(&())
        .await
        .unwrap();

    let resources = tronic
        .provider()
        .get_account_resources(address)
        .await
        .unwrap();
    let bandwidth =
        available_energy(resources.energy_limit, resources.total_energy_limit);
    assert_eq!(bandwidth, 180_000_000_000);
}
