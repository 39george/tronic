use std::future::Future;

use crate::{
    client::{Client, TronProvider},
    domain::{
        block::BlockExtention,
        transaction::{Transaction, TransactionExtention},
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
    filter: F,
    handler: H,
}

// Default filter type that always returns true
#[derive(Clone)]
pub struct DefaultFilter;

impl<P, S, H> TxSubscriber<P, S, DefaultFilter, H>
where
    Client<P, S>: Clone,
{
    pub fn new(client: &Client<P, S>, handler: H) -> Self {
        Self {
            client: client.to_owned(),
            handler,
            filter: DefaultFilter,
        }
    }
}

impl<P, S, F, H> TxSubscriber<P, S, F, H> {
    pub fn with_filter<NewF, NewFut>(
        self,
        filter: NewF,
    ) -> TxSubscriber<P, S, NewF, H>
    where
        NewF: FnOnce(TransactionExtention) -> NewFut + Sync + Clone,
        NewFut: Future<Output = bool> + Send,
    {
        TxSubscriber {
            client: self.client,
            filter,
            handler: self.handler,
        }
    }
}

#[async_trait::async_trait]
impl<P, S, H, Fut> BlockSubscriber for TxSubscriber<P, S, DefaultFilter, H>
where
    H: FnOnce(Transaction) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = ()> + Send,
    P: TronProvider + Sync,
    S: Sync,
{
    async fn handle(&self, msg: BlockExtention) {
        for txext in msg.transactions {
            if let Some(tx) = txext.transaction {
                (self.handler.clone())(tx).await;
            }
        }
    }
}

#[async_trait::async_trait]
impl<P, S, F, H, Fut, FutH> BlockSubscriber for TxSubscriber<P, S, F, H>
where
    F: FnOnce(TransactionExtention) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = bool> + Send,
    H: FnOnce(Transaction) -> FutH + Send + Sync + Clone,
    FutH: Future<Output = ()> + Send,
    P: TronProvider + Sync,
    S: Sync,
{
    async fn handle(&self, msg: BlockExtention) {
        for txext in msg.transactions {
            if (self.filter.clone())(txext.clone()).await
                && let Some(tx) = txext.transaction
            {
                (self.handler.clone())(tx).await;
            }
        }
    }
}
