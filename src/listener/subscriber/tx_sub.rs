use std::{collections::HashSet, future::Future};

use crate::Filter;
use crate::client::{Client, TronProvider};
use crate::domain::contract::Contract;
use crate::domain::{
    address::TronAddress,
    block::BlockExtention,
    transaction::{Transaction, TransactionExtention},
};

use super::BlockSubscriber;

#[async_trait::async_trait]
impl<F, Fut> Filter<TransactionExtention> for F
where
    F: Fn(TransactionExtention) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = bool> + Send,
{
    async fn filter(&self, by: TransactionExtention) -> bool {
        self(by).await
    }
}

// Default filter type that always returns true
#[derive(Clone)]
pub struct DefaultFilter;

#[async_trait::async_trait]
impl Filter<TransactionExtention> for DefaultFilter {
    async fn filter(&self, _: TransactionExtention) -> bool {
        true
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
        NewF: Filter<TransactionExtention>,
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
    F: Filter<TransactionExtention> + Send + Sync + Clone,
    H: FnOnce(Transaction) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = ()> + Send,
    P: TronProvider + Sync,
    S: Sync,
{
    async fn handle(&self, msg: BlockExtention) {
        for txext in msg.transactions {
            if (self.filter.clone()).filter(txext.clone()).await
                && let Some(tx) = txext.transaction
            {
                (self.handler.clone())(tx).await;
            }
        }
    }
}
