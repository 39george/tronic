use std::future::Future;

use crate::listener::ListenerMsg;

pub mod filters;
pub mod tx_sub;

// TODO: implement for fnonce
#[async_trait::async_trait]
pub trait BlockSubscriber {
    async fn handle(&self, msg: ListenerMsg);
}

#[async_trait::async_trait]
impl<F, Fut> BlockSubscriber for F
where
    F: Fn(ListenerMsg) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
{
    async fn handle(&self, msg: ListenerMsg) {
        self(msg).await
    }
}
