//! TRON USDT Transfer Listener Example
//!
//! When running, it will:
//! - Connect to TRON Grid's public GRPC endpoint
//! - Monitor transactions for the specified address
//! - Print formatted USDT transfer details including:
//!   - Sender/Receiver addresses
//!   - Transfer amount
//!   - Optional message data
//!
//! Press Ctrl+C to exit.

use std::{collections::HashMap, time::Duration};

use tronic::{
    client::Client,
    contracts::{
        TryFromData,
        token::{InMemoryTokenRegistry, usdt::Usdt},
        trc20::Trc20Call,
    },
    domain::{Hash32, transaction::Transaction},
    extractor::DynamicTrc20Extractor,
    listener::subscriber::{filters::AddressFilter, tx_sub::TxSubscriber},
    provider::grpc::GrpcProvider,
    signer::LocalSigner,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a Tronic client with GRPC provider
    let client = Client::builder()
        .provider(
            GrpcProvider::builder()
                .connect("http://grpc.trongrid.io:50051")
                .await?,
        )
        .signer(LocalSigner::rand())
        .build();
    let listener_handle = client.listener(Duration::from_secs(2)).await;

    // Set up an in-memory token registry with USDT contract
    let registry = InMemoryTokenRegistry::from(
        vec![(
            "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".parse().unwrap(),
            tronic::contracts::token::TokenKind::Usdt,
        )]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    );

    // Configure an address filter to watch specific accounts
    let filter = AddressFilter::new(|| async move {
        Some(
            vec!["TDqSquXBgUCLYvYC4XZgrprLK589dkhSCf".parse().unwrap()]
                .into_iter()
                .collect(),
        )
    })
    .with_extractor::<DynamicTrc20Extractor>()
    .with_registry(registry);

    // Subscribe to transactions and decode TRC-20 transfers
    let subscriber =
        TxSubscriber::new(&client, |t: Transaction, txid: Hash32| async move {
            if let Some(c) = t.get_contract() {
                if let Some(trg) = c.trigger_smart_contract()
                    && let Ok(trc20) = Trc20Call::<Usdt>::try_from_data(
                        &trg.data.to_bytes_vec(),
                    )
                {
                    if let Trc20Call::Transfer(transfer_call) = trc20 {
                        let message = if !t.raw.data.is_empty() {
                            t.raw.data
                        } else {
                            "None".into()
                        };

                        println!(
                            "\n  USDT Transfer Detected:\n\
                              ├─ From:     {}\n\
                              ├─ To:       {}\n\
                              ├─ Amount:   {}\n\
                              ├─ Contract: {}\n\
                              ├─ Message:  {}\n\
                              └─ Txid:     {:?}",
                            trg.owner_address,
                            transfer_call.recipient,
                            transfer_call.amount,
                            trg.contract_address,
                            message,
                            txid
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
