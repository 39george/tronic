use std::io::{self, Write};

use tronic::{
    client::{Client, pending::AutoSigning},
    domain::{address::TronAddress, trx::Trx},
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

    // Prompt for private key
    print!("Enter sender private key (hex format): ");
    let signer_str = read_data()?;
    let signer = LocalSigner::from_bytes(&hex::decode(signer_str.trim())?)?;
    let sender_address = signer.address();
    println!("Sender address: {sender_address}");

    // Create client
    let client = Client::builder()
        .provider(
            GrpcProvider::builder()
                .connect("http://grpc.trongrid.io:50051")
                .await?,
        )
        .signer(signer)
        .build();

    // Prompt for recipient address
    print!("Enter recipient address: ");
    let recipient: TronAddress = read_data()?.trim().parse()?;

    // Prompt for amount (in TRX)
    print!("Enter amount to send (TRX): ");
    let amount: Trx = read_data()?.trim().parse::<f64>()?.into();

    println!("Sending {amount} to {recipient}...",);

    // Send transaction
    let txid = client
        .send_trx()
        .to(recipient)
        .amount(amount)
        .can_spend_trx_for_fee(true)
        .build::<AutoSigning>() // Uses automatic signing strategy
        .await?
        .broadcast(&())
        .await?;

    println!("Transaction sent successfully!");
    println!("Transaction hash: {txid:?}",);

    Ok(())
}
