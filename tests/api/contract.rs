use std::time::Duration;

use alloy_primitives::U256;
use tronic::client::pending::AutoSigning;
use tronic::contracts::token::usdt::Usdt;
use tronic::contracts::trc20::Trc20Contract;
use tronic::domain::transaction::TxCode;
use tronic::provider::TronProvider;

use crate::helpers::Tronic;

const MYCOIN_CONTRACT: &str = include_str!("../../tests/assets/MyCoin.json");

#[tokio::test]
async fn deploy_trc20_contract() {
    let tronic = Tronic::new().await;
    let initial_supply = Usdt::from_decimal(10_000_000.0).unwrap();
    let txid = tronic
        .create_contract(MYCOIN_CONTRACT.into())
        .params(vec![
            &Into::<U256>::into(initial_supply),
            &alloy_primitives::Address::ZERO,
        ])
        .consume_user_resource_percent(100)
        .origin_energy_limit(10_000_000)
        .can_spend_trx_for_fee(true)
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast(&())
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(7)).await;
    let txinfo = tronic.provider().get_transaction_info(txid).await.unwrap();
    assert_eq!(txinfo.result, TxCode::Sucess);

    let upload_contract_addr = txinfo.contract_address;
    let owner_balance = tronic
        .trc20_balance_of()
        .contract(Trc20Contract::<Usdt>::new(upload_contract_addr))
        .get()
        .await
        .unwrap();
    assert_eq!(owner_balance, initial_supply);
}
