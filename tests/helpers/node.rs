use k256::ecdsa::SigningKey;
use tronic::domain::address::TronAddress;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::runtime::Runtime;
use tronic::client::pending::AutoSigning;
use tronic::client::Client;
use tronic::provider::grpc::GrpcProvider;
use tronic::signer::LocalSigner;
use tronic::trx;

#[static_init::dynamic(drop)]
pub static mut NODE: Node = Node::start("test_tron_container");

pub struct Node {
    container_name: String,
    grpc_port: u16,
    tx: tokio::sync::mpsc::Sender<tokio::sync::oneshot::Sender<LocalSigner>>,
    zion_addr: TronAddress,
}

impl Node {
    fn start(container_name: &'static str) -> Self {
        let grpc_port = portpicker::pick_unused_port().unwrap();

        // Create the runtime first
        let rt = Runtime::new().unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel::<tokio::sync::oneshot::Sender<LocalSigner>>(1000);
        
        // Use the runtime to block on async operations
        std::thread::spawn(move || {
            rt.block_on(async {
                // Start the Tron Quickstart container
                Command::new("docker")
                    .args(&[
                        "run",
                        "-d",
                        "-it",
                        "-p",
                        &format!("{grpc_port}:50051"),
                        "-v",
                        "./tests/conf/tron.conf:/java-tron/conf/tron.conf",
                        "--rm",
                        "--name",
                        container_name,
                        "custom-tron",
                        "-jvm",
                        "{-Xmx10g -Xms10g}",
                        "-c",
                        "/java-tron/conf/tron.conf",
                        "--es",
                        "-w",
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .await
                    .unwrap();

                tokio::time::sleep(Duration::from_secs(8)).await;

                let signing_key = SigningKey::from_slice(&hex::decode("da146374a75310b9666e834ee4ad0866d6f4035967bfc76217c5a495fff9f0d0").unwrap()).unwrap();

                let provider = GrpcProvider::new(
                    format!("http://localhost:{}", grpc_port).parse().unwrap(),
                    tronic::client::Auth::None,
                ).await.unwrap();
                
                let zion = Client::builder()
                    .provider(provider)
                    .signer(LocalSigner::from(signing_key))
                    .build();

                while let Some(tx) = rx.recv().await {
                    let signer = LocalSigner::rand();
                    let _ = zion
                        .send_trx()
                        .to(signer.address())
                        .amount(trx!(100_000.0 TRX))
                        .build::<AutoSigning>()
                        .await
                        .unwrap()
                        .broadcast(&())
                        .await
                        .unwrap();
                    tx.send(signer).unwrap();
                }
            });
        });

        // Wait node to initialize
        std::thread::sleep(Duration::from_secs(10));

        Self {
            container_name: container_name.into(),
            grpc_port,
            tx, 
            zion_addr: LocalSigner::from(SigningKey::from_slice(&hex::decode("da146374a75310b9666e834ee4ad0866d6f4035967bfc76217c5a495fff9f0d0").unwrap()).unwrap()).address()
        }
    }

    pub fn grpc_addr(&self) -> http::Uri {
        format!("http://localhost:{}", self.grpc_port)
            .parse()
            .unwrap()
    }

    pub fn zion_addr(&self) -> TronAddress {
        self.zion_addr
    }

    pub async fn new_account(&self) -> LocalSigner {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx.send(tx).await.unwrap();
        rx.await.unwrap()
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let container_name = self.container_name.clone();
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                Command::new("docker")
                    .args(&["stop", &container_name])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .await
                    .unwrap();
            });
        }).join().unwrap();
    }
}
