use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, oneshot};
use tokio::time::timeout;
use tronic::client::Client;
use tronic::client::pending::{AutoSigning, ManualSigning, PendingTransaction};
use tronic::contracts::TryFromData;
use tronic::contracts::token::InMemoryTokenRegistry;
use tronic::contracts::token::usdt::Usdt;
use tronic::contracts::trc20::{Trc20Call, Trc20Calls, Trc20Contract};
use tronic::domain::Hash32;
use tronic::domain::account::AccountStatus;
use tronic::domain::address::TronAddress;
use tronic::domain::estimate::MissingResource;
use tronic::domain::transaction::{TransactionExtention, TxCode};
use tronic::extractor::DynamicTrc20Extractor;
use tronic::listener::subscriber::filters::AddressFilter;
use tronic::listener::subscriber::tx_sub::TxSubscriber;
use tronic::provider::TronProvider;
use tronic::provider::grpc::GrpcProvider;
use tronic::signer::LocalSigner;
use tronic::trx;

use crate::helpers::{NODE, Tronic};

#[tokio::test]
async fn deploy_trc20_contract() {
    let tronic = Tronic::new().await;
    let initial_supply = Usdt::from_decimal(10_000_000.0).unwrap();
    let txinfo = tronic.deploy_mycoin(100, 10_000_000).await;
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

#[tokio::test]
async fn listener_catches_trc20_transfer() {
    let tronic = Tronic::new().await;
    let txinfo = tronic.deploy_mycoin(100, 10_000_000).await;

    let registry = InMemoryTokenRegistry::from(
        [(
            txinfo.contract_address,
            tronic::contracts::token::TokenKind::Usdt,
        )]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    );

    let watched = tronic.signer_address().unwrap();
    let filter = AddressFilter::new(move || {
        let watched = watched.clone();
        async move {
            let mut hs = std::collections::HashSet::new();
            hs.insert(watched);
            hs
        }
    })
    .with_extractor::<DynamicTrc20Extractor>()
    .with_registry(registry);

    let (tx, rx) =
        oneshot::channel::<(Hash32, TronAddress, TronAddress, Usdt)>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let listener = tronic.listener(Duration::from_millis(200)).await;
    let subscriber = TxSubscriber::new(&*tronic, move |res| {
        let tx = tx.clone();
        async move {
            if let Ok(txext) = res {
                // decode transfer, if matches -> send + ignore errors
                if let Some((txid, from, to, amount)) =
                    decode_usdt_transfer(&txext)
                {
                    if let Some(sender) = tx.lock().await.take() {
                        let _ = sender.send((txid, from, to, amount));
                    }
                }
            }
        }
    })
    .with_filter(filter);

    listener.subscribe(subscriber);

    let recipient = NODE.read().new_account(trx!(100_000.0 TRX)).await;
    tronic
        .trc20_transfer()
        .contract(Trc20Contract::<Usdt>::new(txinfo.contract_address))
        .to(recipient.address())
        .amount(Usdt::from_decimal(123.0).unwrap())
        .can_spend_trx_for_fee(true)
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast(&())
        .await
        .unwrap();

    let (txid, from, to, amount) =
        timeout(Duration::from_secs(10), rx).await.unwrap().unwrap();
    assert_eq!(from, watched);
    assert_eq!(to, recipient.address());
    assert_eq!(amount, Usdt::from_decimal(123.0).unwrap());
    assert_ne!(txid, Hash32::default());
}

#[tokio::test]
async fn activation_fee_rechecked_if_tx_broadcast_later() {
    let tronic = Tronic::new().await;
    let deploy_info = tronic.deploy_mycoin(100, 10_000_000).await;
    assert_eq!(deploy_info.result, TxCode::Sucess);

    let token_contract =
        Trc20Contract::<Usdt>::new(deploy_info.contract_address);

    tronic
        .freeze_balance()
        .amount(trx!(100.0 TRX))
        .resource(tronic::domain::contract::ResourceCode::Energy)
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast_get_receipt(&(), 1)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(7)).await;

    let recipient = LocalSigner::rand();
    let to = recipient.address();

    let st = tronic.check_account(to).await.unwrap();
    assert!(matches!(st, AccountStatus::NotExists));

    let owner = tronic.signer_address().unwrap();
    let zion_addr = NODE.read().zion_addr();

    let bal = tronic.trx_balance().address(owner).get().await.unwrap();
    let keep = trx!(0.09 TRX);

    if bal > keep {
        let drain = bal - keep;
        tronic
            .send_trx()
            .to(zion_addr)
            .amount(drain)
            .build::<AutoSigning>()
            .await
            .unwrap()
            .broadcast_get_receipt(&(), 1)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(7)).await;
    }

    let bal2 = tronic.trx_balance().address(owner).get().await.unwrap();
    assert!(bal2 < trx!(0.1 TRX));

    let ptx = tronic
        .trc20_transfer()
        .contract(token_contract.clone())
        .to(to)
        .amount(Usdt::from_decimal(1.0).unwrap())
        .build::<ManualSigning>()
        .await
        .unwrap()
        .set_expiration(time::Duration::seconds(300))
        .await
        .unwrap();

    let before = ptx.estimate_transaction().await.unwrap();

    assert_eq!(before.will_consume.trx, trx!(0.1 TRX));

    let missing = before.insufficient.as_ref().unwrap().missing.clone();
    assert!(
        missing
            .iter()
            .any(|m| matches!(m, MissingResource::Trx { .. })),
        "should be MissingResource::Trx because of activation fee"
    );

    let bytes = ptx.serialize();

    let grpc_addr = NODE.read().grpc_addr();
    let gas_signer = NODE.read().new_account(trx!(10.0 TRX)).await;

    let gas_client = Client::builder()
        .provider(GrpcProvider::builder().connect(grpc_addr).await.unwrap())
        .signer(gas_signer)
        .build();

    gas_client
        .send_trx()
        .to(to)
        .amount(trx!(1.0 TRX))
        .build::<AutoSigning>()
        .await
        .unwrap()
        .broadcast_get_receipt(&(), 1)
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(7)).await;

    let st2 = tronic.check_account(to).await.unwrap();
    assert!(
        !matches!(st2, AccountStatus::NotExists),
        "after activation address should exist"
    );

    let mut ptx = PendingTransaction::try_deserialize(&tronic, &bytes).unwrap();

    let after = ptx.estimate_transaction().await.unwrap();
    assert_eq!(after.will_consume.trx, trx!(0.0 TRX));

    if let Some(ins) = after.insufficient.as_ref() {
        assert!(
            !ins.missing
                .iter()
                .any(|m| matches!(m, MissingResource::Trx { .. })),
            "after activation MissingResource::Trx should not be the case"
        );
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    ptx.reset_estimates().await.unwrap();

    ptx.sign(tronic.signer(), &()).await.unwrap();
    let txinfo = ptx.broadcast_get_receipt(1).await.unwrap();

    assert_eq!(txinfo.result, TxCode::Sucess);

    let got = tronic
        .trc20_balance_of()
        .contract(token_contract)
        .owner(to)
        .get()
        .await
        .unwrap();

    assert_eq!(got, Usdt::from_decimal(1.0).unwrap());
}

pub fn decode_usdt_transfer(
    txext: &TransactionExtention,
) -> Option<(Hash32, TronAddress, TronAddress, Usdt)> {
    let txid: Hash32 = txext.txid.clone().try_into().ok()?;

    let tx = txext.transaction.as_ref()?;
    let contract = tx.get_contract()?;
    let trg = contract.trigger_smart_contract()?;
    let call =
        Trc20Call::<Usdt>::try_from_data(&trg.data.to_bytes_vec()).ok()?;

    match call {
        Trc20Call::Transfer(t) => {
            Some((txid, trg.owner_address.clone(), t.recipient, t.amount))
        }
        _ => None,
    }
}
