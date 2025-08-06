use std::time::Duration;

use tronic::client::pending::{AutoSigning, ManualSigning, PendingTransaction};
use tronic::domain::permission::{Key, Ops, PermissionParams};
use tronic::domain::transaction::TxCode;
use tronic::provider::TronProvider;
use tronic::trx;

use crate::helpers::{NODE, Tronic};

#[tokio::test]
async fn multisig_trx_transfer() {
    let tronic = Tronic::new().await;
    let zion = NODE.read().zion_addr();
    let multisig_participant = NODE.read().new_account().await;

    // Configure permissions
    let mut permissions = tronic
        .account_permissions(tronic.signer_address().unwrap())
        .await
        .unwrap();
    permissions
        .set_actives(vec![PermissionParams {
            permission_name: "CustomPermissions".try_into().unwrap(),
            threshold: 2,
            parent_id: 0,
            operations: vec![Ops::TransferContract],
            keys: vec![
                Key {
                    address: multisig_participant.address(),
                    weight: 1,
                },
                Key {
                    address: tronic.signer_address().unwrap(),
                    weight: 2,
                },
            ],
        }])
        .unwrap();
    let txid = permissions
        .update_permission::<AutoSigning>()
        .await
        .unwrap()
        .broadcast(&())
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(7)).await;
    let txinfo = tronic.provider().get_transaction_info(txid).await.unwrap();
    assert_eq!(txinfo.result, TxCode::Sucess);

    // Make multisig transaction
    let mut tx = tronic
        .send_trx()
        .amount(trx!(1.0 TRX))
        .to(zion)
        .build::<ManualSigning>()
        .await
        .unwrap();
    tx.set_permission(2).await.unwrap();
    let tx = tx.serialize();

    let mut tx = PendingTransaction::try_deserialize(&tronic, tx).unwrap();
    tx.sign(tronic.signer(), &()).await.unwrap();
    tx.sign(&multisig_participant, &()).await.unwrap();

    let txid = tx.broadcast().await.unwrap();

    tokio::time::sleep(Duration::from_secs(7)).await;
    let txinfo = tronic.provider().get_transaction_info(txid).await.unwrap();
    assert_eq!(txinfo.result, TxCode::Sucess);
}
