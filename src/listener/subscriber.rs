use std::{collections::HashSet, future::Future};

use crate::{
    client::{Client, TronProvider},
    domain::{
        address::TronAddress, block::BlockExtention, transaction::Transaction,
    },
};

#[async_trait::async_trait]
pub trait BlockSubscriber {
    async fn handle(&self, msg: BlockExtention);
}

#[async_trait::async_trait]
impl<F, Fut> BlockSubscriber for F
where
    F: Fn(BlockExtention) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
{
    async fn handle(&self, msg: BlockExtention) {
        self(msg).await
    }
}

pub struct TxSubscriber<P, S, F, H> {
    client: Client<P, S>,
    addresses: F,
    handler: H,
}

impl<P, S, F, H> TxSubscriber<P, S, F, H>
where
    Client<P, S>: Clone,
{
    pub fn new(client: &Client<P, S>, addresses: F, handler: H) -> Self {
        Self {
            client: client.to_owned(),
            addresses,
            handler,
        }
    }
}

#[async_trait::async_trait]
impl<P, S, F, H, Fut, FutH> BlockSubscriber for TxSubscriber<P, S, F, H>
where
    F: FnOnce() -> Fut + Sync + Clone,
    Fut: Future<Output = HashSet<TronAddress>> + Send,
    H: FnOnce(Transaction) -> FutH + Sync + Clone,
    FutH: Future<Output = ()> + Send,
    P: TronProvider + Sync,
    S: Sync,
{
    async fn handle(&self, msg: BlockExtention) {
        let addrs = (self.addresses.clone())().await;
        for txext in msg.transactions {
            if let Some(contract) = txext
                .transaction
                .clone()
                .and_then(|t| t.raw)
                .as_ref()
                .and_then(|raw_tx| raw_tx.contract.as_ref())
            {
                println!("Found contract: {:?}", contract);
            }
        }
        // (self.handler.clone())(t).await;
    }
}
