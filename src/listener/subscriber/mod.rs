use std::future::Future;

use crate::domain::block::BlockExtention;

pub mod filters;
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
