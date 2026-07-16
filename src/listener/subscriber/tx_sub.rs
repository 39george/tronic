use std::future::Future;

use futures::{StreamExt, stream};

use crate::Filter;
use crate::client::Client;
use crate::domain::block::BlockExtension;
use crate::domain::transaction::TransactionExtension;
use crate::listener::{ListenerError, ListenerMsg};
use crate::provider::TronProvider;

use super::BlockSubscriber;

#[async_trait::async_trait]
impl<F> Filter<BlockExtension> for F
where
    F: Fn(&TransactionExtension) -> bool + Send + Sync,
{
    type Item = TransactionExtension;
    async fn filter(&self, content: BlockExtension) -> Vec<Self::Item> {
        content
            .transactions
            .into_iter()
            .filter(|tx| self(tx))
            .collect::<Vec<_>>()
    }
}

// Default filter type that returns all
#[derive(Clone)]
pub struct DefaultFilter;

#[async_trait::async_trait]
impl Filter<BlockExtension> for DefaultFilter {
    type Item = TransactionExtension;
    async fn filter(&self, content: BlockExtension) -> Vec<Self::Item> {
        content.transactions
    }
}

pub struct TxSubscriber<P, S, F, H> {
    client: Client<P, S>,
    filter: F,
    handler: H,
}

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
    pub fn with_filter<NewF>(self, filter: NewF) -> TxSubscriber<P, S, NewF, H>
    where
        NewF: Filter<BlockExtension>,
    {
        TxSubscriber {
            client: self.client,
            filter,
            handler: self.handler,
        }
    }
}

#[async_trait::async_trait]
impl<P, S, F, H, Fut> BlockSubscriber for TxSubscriber<P, S, F, H>
where
    F: Filter<BlockExtension, Item = TransactionExtension> + Send + Sync,
    H: Fn(Result<TransactionExtension, ListenerError>) -> Fut
        + Send
        + Sync
        + Clone,
    Fut: Future<Output = ()> + Send,
    P: TronProvider + Sync,
    S: Sync,
{
    async fn handle(&self, msg: ListenerMsg) {
        let block = match msg {
            Ok(be) => be,
            Err(e) => {
                (self.handler.clone())(Err(e)).await;
                return;
            }
        };
        let txs = self.filter.filter(block).await;
        stream::iter(txs)
            .for_each_concurrent(16, |txext| {
                let h = self.handler.clone();
                async move { h(Ok(txext)).await }
            })
            .await;
    }
}
