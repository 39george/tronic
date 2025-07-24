use std::collections::HashMap;

use tronic::{
    client::Client,
    contracts::{
        TryFromData,
        token::{InMemoryTokenRegistry, usdt::Usdt},
        trc20::Trc20Call,
    },
    domain::transaction::Transaction,
    extractor::DynamicTrc20Extractor,
    listener::subscriber::{filters::AddressFilter, tx_sub::TxSubscriber},
    providers::grpc::GrpcProvider,
    signer::LocalSigner,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .provider(
            GrpcProvider::new(
                "http://grpc.trongrid.io:50051".parse().unwrap(),
                tronic::client::Auth::None,
            )
            .await?,
        )
        .signer(LocalSigner::rand())
        .build();
    let listener_handle = client.listener().await;
    let registry = InMemoryTokenRegistry::from(
        vec![(
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".parse().unwrap(),
            tronic::contracts::token::TokenKind::Usdt,
        )]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    );
    let filter = AddressFilter::new(|| async move {
        Some(
            vec!["TDqSquXBgUCLYvYC4XZgrprLK589dkhSCf".parse().unwrap()]
                .into_iter()
                .collect(),
        )
    })
    .with_extractor::<DynamicTrc20Extractor>()
    .with_registry(registry);
    let subscriber = TxSubscriber::new(&client, |t: Transaction| async move {
        if let Some(c) = t.get_contract() {
            if let Some(trg) = c.trigger_smart_contract()
                && let Ok(trc20) =
                    Trc20Call::<Usdt>::try_from_data(&trg.data.to_vec())
            {
                if let Trc20Call::Transfer(transfer_call) = trc20 {
                    let message = t
                        .raw
                        .and_then(|r| (!r.data.is_empty()).then_some(r.data))
                        .unwrap_or_else(|| "None".into());

                    println!(
                        "\n  USDT Transfer Detected:\n\
                              ├─ From:     {}\n\
                              ├─ To:       {}\n\
                              ├─ Amount:   {}\n\
                              ├─ Contract: {}\n\
                              └─ Message:  {}",
                        trg.owner_address,
                        transfer_call.recipient,
                        transfer_call.amount,
                        trg.contract_address,
                        message
                    );
                }
            } else {
                println!("\nTransaction: {t:?}");
            }
        }
    })
    .with_filter(filter);
    listener_handle.subscribe(subscriber);
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("\nShutting down gracefully...");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {err}");
        }
    }
    Ok(())
}
