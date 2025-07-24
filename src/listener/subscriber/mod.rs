use std::{collections::HashSet, future::Future};

use crate::{
    client::{Client, TronProvider},
    domain::{
        address::TronAddress,
        block::BlockExtention,
        transaction::{Transaction, TransactionExtention},
    },
};

pub mod tx_sub;

// TODO: implement for fnonce
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
