use std::io::{self, Write};

use tronic::{
    client::{Client, pending::ManualSigning},
    contracts::{
        token::usdt::Usdt,
        trc20::{Trc20Calls, Trc20Contract},
    },
    domain::address::TronAddress,
    provider::grpc::GrpcProvider,
    signer::LocalSigner,
};

pub fn read_data() -> io::Result<String> {
    io::stdout().flush()?;
    let mut data = String::new();
    io::stdin().read_line(&mut data)?;
    Ok(data)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Using tron nile testnet (grpc.nile.trongrid.io:50051)");

    print!("Enter owner account private key (hex format): ");
    let owner = LocalSigner::from_bytes(&hex::decode(read_data()?.trim())?)?;

    // Create client
    let client = Client::builder()
        .provider(
            GrpcProvider::builder()
                .connect("http://grpc.trongrid.io:50051")
                .await?,
        )
        .signer(owner.clone())
        .build();
    print!("Enter recipient address: ");

    let recipient: TronAddress = read_data()?.trim().parse()?;

    // Prompt for amount (in USDT)
    print!("Enter amount to send (USDT): ");
    let amount = Usdt::from_decimal(read_data()?.trim().parse::<f64>()?)?;

    // Call trc20 transfer method
    let mut tx = client
        .trc20_transfer()
        .contract(Trc20Contract::new(
            "TXYZopYRdj2D9XRtbG411XZZ3kM5VkAeBf".parse().unwrap(),
        ))
        .to(recipient)
        .can_spend_trx_for_fee(true)
        .amount(amount)
        .build::<ManualSigning>() // Uses manual signing strategy
        .await?;

    print!("Enter integer permission id to use: ");
    let permission_id: i32 = read_data()?.trim().parse()?;

    tx.set_permission(permission_id).await?;

    // Estimate transaction
    let estimation = tx.estimate_transaction().await?;
    println!("Transaction estimation: {estimation:#?}");
    println!("Total trx required: {}", estimation.trx_required());

    // Sign by owner
    tx.sign(&owner, &()).await?;
    println!("Transaction signed by owner");

    print!("Enter participant account private key (hex format): ");
    let participant =
        LocalSigner::from_bytes(&hex::decode(read_data()?.trim())?)?;
    tx.sign(&participant, &()).await?;

    println!("Sending {amount} to {recipient}...",);

    // Broadcast
    let txid = tx.broadcast().await?;

    println!("Transaction sent successfully!");
    println!("Transaction hash: {txid:?}",);

    Ok(())
}
