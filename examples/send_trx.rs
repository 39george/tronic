use std::io::{self, Write};
use tronic::{
    client::{Client, pending::AutoSigning},
    domain::{address::TronAddress, trx::Trx},
    provider::grpc::GrpcProvider,
    signer::LocalSigner,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Using tron nile testnet (grpc.nile.trongrid.io:50051)");
    // Prompt for private key
    print!("Enter sender private key (hex format): ");
    io::stdout().flush()?;
    let mut private_key = String::new();
    io::stdin().read_line(&mut private_key)?;
    let private_key = private_key.trim();

    // Create signer from private key
    let signer = LocalSigner::from_bytes(&hex::decode(private_key)?)?;
    let sender_address = signer.address();

    println!("Sender address: {sender_address}");

    // Prompt for recipient address
    print!("Enter recipient address: ");
    io::stdout().flush()?;
    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient)?;
    let recipient: TronAddress = recipient.trim().parse()?;

    // Prompt for amount (in TRX)
    print!("Enter amount to send (TRX): ");
    io::stdout().flush()?;
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    let amount: Trx = amount.trim().parse::<f64>()?.into();

    // Create client
    let client = Client::builder()
        .provider(
            GrpcProvider::new(
                "http://grpc.nile.trongrid.io:50051".parse()?,
                tronic::client::Auth::None,
            )
            .await?,
        )
        .signer(signer)
        .build();

    println!("Sending {amount} TRX to {recipient}...",);

    // Send transaction
    let txid = client
        .send_trx()
        .to(recipient)
        .amount(amount)
        .build::<AutoSigning>() // Denote signing approach
        .await?
        .broadcast(&())
        .await?;

    println!("Transaction sent successfully!");
    println!("Transaction hash: {txid:?}",);

    Ok(())
}
