use std::ops::Deref;

pub use node::NODE;
use tronic::{
    client::Client, provider::grpc::GrpcProvider, signer::LocalSigner,
};

mod node;

pub struct Tronic {
    client: Client<GrpcProvider, LocalSigner>,
}

impl Deref for Tronic {
    type Target = Client<GrpcProvider, LocalSigner>;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Tronic {
    pub async fn new() -> Self {
        let node = NODE.read();
        let acc = node.new_account().await;
        let signer = acc.into();
        let client = Client::builder()
            .provider(
                GrpcProvider::new(node.grpc_addr(), tronic::client::Auth::None)
                    .await
                    .unwrap(),
            )
            .signer(signer)
            .build();
        Self { client }
    }
}
