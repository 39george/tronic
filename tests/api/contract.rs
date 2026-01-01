use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, oneshot};
use tokio::time::timeout;
use tronic::client::pending::AutoSigning;
use tronic::contracts::TryFromData;
use tronic::contracts::token::InMemoryTokenRegistry;
use tronic::contracts::token::usdt::Usdt;
use tronic::contracts::trc20::{Trc20Call, Trc20Calls, Trc20Contract};
use tronic::domain::Hash32;
use tronic::domain::address::TronAddress;
use tronic::domain::transaction::{TransactionExtention, TxCode};
use tronic::extractor::DynamicTrc20Extractor;
use tronic::listener::subscriber::filters::AddressFilter;
use tronic::listener::subscriber::tx_sub::TxSubscriber;

use crate::helpers::Tronic;

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

    let recipient = crate::helpers::NODE.read().new_account().await;
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
