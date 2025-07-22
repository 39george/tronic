use futures::{Stream, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

use crate::client::TronProvider;
use crate::domain::block::BlockExtention;
use crate::signer::PrehashSigner;

use super::Message;

#[async_trait::async_trait]
pub trait BlockSubscriber {
    async fn handle(&self, msg: Message);
}

#[async_trait::async_trait]
impl<F, Fut> BlockSubscriber for F
where
    F: Fn(Message) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
{
    async fn handle(&self, msg: Message) {
        self(msg).await
    }
}
