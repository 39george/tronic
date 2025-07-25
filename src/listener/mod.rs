use futures::{Stream, StreamExt};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

use crate::client::Client;
use crate::domain::block::BlockExtention;
use crate::provider::TronProvider;
use crate::{listener::subscriber::BlockSubscriber, signer::PrehashSigner};

pub mod subscriber;

pub struct ListenerHandle {
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    rx: tokio::sync::broadcast::Receiver<BlockExtention>,
}

impl ListenerHandle {
    pub fn subscribe<S: BlockSubscriber + Send + Sync + 'static>(
        &self,
        subscriber: S,
    ) {
        let mut rx = self.rx.resubscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                subscriber.handle(msg).await;
            }
        });
    }
}

impl Drop for ListenerHandle {
    fn drop(&mut self) {
        let _ = self.shutdown.take().unwrap().send(());
    }
}

pub struct Listener<P, S> {
    client: Client<P, S>,
    last_block_number: i64,
    interval: Duration,
}

impl<P, S> Listener<P, S>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    pub fn new(client: Client<P, S>) -> Self {
        Self {
            client,
            last_block_number: -1,
            interval: Duration::from_secs(3),
        }
    }
    pub async fn run(self) -> ListenerHandle {
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
        let (tx, rx) = tokio::sync::broadcast::channel(128);
        let mut block_stream = self.block_stream();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = block_stream.next() => {
                        if let Err(e) = tx.send(msg.clone()) {
                            tracing::error!("failed to send block msg: {e}");
                        }
                    }
                    //  WARN: attempting to .await it after it has already completed will panic
                    _ = &mut shutdown_rx => {
                        tracing::info!("exiting from listener");
                        break;
                    }
                }
            }
        });
        ListenerHandle {
            shutdown: Some(shutdown_tx),
            rx,
        }
    }
    fn block_stream(self) -> impl Stream<Item = BlockExtention> {
        BlockStream {
            listener: self,
            delay: Box::pin(sleep(Duration::from_secs(0))),
            fut: None,
        }
    }
}

struct BlockStream<P, S> {
    listener: Listener<P, S>,
    delay: Pin<Box<tokio::time::Sleep>>,
    fut: Option<
        Pin<Box<dyn Future<Output = Option<(i64, BlockExtention)>> + Send>>,
    >,
}

impl<P, S> Unpin for BlockStream<P, S> {}

impl<P, S> Stream for BlockStream<P, S>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    type Item = BlockExtention;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.delay.as_mut().poll(cx).is_pending() {
            return Poll::Pending;
        }

        // If we don't already have a future, build one
        if self.fut.is_none() {
            let client = self.listener.client.clone();
            let last_block = self.listener.last_block_number;

            self.fut = Some(Box::pin(async move {
                let block = client.provider.get_now_block().await.ok();
                block.and_then(|b| {
                    let number = b
                        .block_header
                        .as_ref()
                        .and_then(|h| h.raw_data.as_ref())
                        .map(|rd| rd.number)?;

                    if number <= last_block {
                        return None;
                    }

                    Some((number, b))
                })
            }));
        }

        let interval = self.listener.interval;

        let fut = self.fut.as_mut().unwrap();
        match fut.as_mut().poll(cx) {
            Poll::Ready(Some((new_number, msg))) => {
                self.listener.last_block_number = new_number;
                self.delay
                    .as_mut()
                    .reset(tokio::time::Instant::now() + interval);
                self.fut = None;
                Poll::Ready(Some(msg))
            }
            Poll::Ready(None) => {
                self.delay
                    .as_mut()
                    .reset(tokio::time::Instant::now() + interval);
                self.fut = None;
                Poll::Pending
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
