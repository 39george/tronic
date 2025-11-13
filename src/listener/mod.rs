use futures::stream::FuturesOrdered;
use futures::{Stream, StreamExt, TryStreamExt};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

use crate::Result;
use crate::client::Client;
use crate::domain::block::BlockExtention;
use crate::provider::TronProvider;
use crate::{listener::subscriber::BlockSubscriber, signer::PrehashSigner};

pub mod subscriber;

const MAX_BLOCKS_PER_FETCH: i64 = 100;

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
    pub fn new(client: Client<P, S>, block_poll_interval: Duration) -> Self {
        Self {
            client,
            last_block_number: -1,
            interval: block_poll_interval,
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
    pub(crate) fn block_stream(self) -> impl Stream<Item = BlockExtention> {
        BlockStream {
            listener: self,
            delay: Box::pin(sleep(Duration::from_secs(0))),
            fut: None,
            pending_blocks: VecDeque::new(),
        }
    }
}

struct BlockStream<P, S> {
    listener: Listener<P, S>,
    delay: Pin<Box<tokio::time::Sleep>>,
    fut: Option<
        Pin<Box<dyn Future<Output = Result<Vec<BlockExtention>>> + Send>>,
    >,
    pending_blocks: VecDeque<BlockExtention>,
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
        if let Some(block) = self.pending_blocks.pop_front() {
            return Poll::Ready(Some(block));
        }

        if self.delay.as_mut().poll(cx).is_pending() {
            return Poll::Pending;
        }

        // If we don't already have a future, build one
        if self.fut.is_none() {
            let client = self.listener.client.clone();
            let last_block = self.listener.last_block_number;

            self.fut = Some(Box::pin(async move {
                let latest_block = client.provider.get_now_block().await?;
                let latest_number = latest_block.block_header.raw_data.number;

                if last_block == -1 {
                    return Ok(vec![latest_block]);
                }

                if latest_number <= last_block {
                    return Ok(Vec::new());
                }

                let first_needed = last_block + 1;
                let batch_end_number = (first_needed + MAX_BLOCKS_PER_FETCH
                    - 1)
                .min(latest_number);

                let futures: FuturesOrdered<_> = (first_needed
                    ..=batch_end_number)
                    .map(|num| {
                        let provider = client.provider();
                        async move { provider.get_block_by_number(num).await }
                    })
                    .collect();

                let blocks_to_send: Vec<_> = futures.try_collect().await?;

                Ok(blocks_to_send)
            }));
        }

        let interval = self.listener.interval;

        let fut = self.fut.as_mut().unwrap();
        match fut.as_mut().poll(cx) {
            Poll::Ready(Ok(blocks)) => {
                self.fut = None;
                self.delay
                    .as_mut()
                    .reset(tokio::time::Instant::now() + interval);
                if !blocks.is_empty() {
                    self.listener.last_block_number =
                        blocks.last().unwrap().block_header.raw_data.number;
                    self.pending_blocks.extend(blocks);
                    let block = self.pending_blocks.pop_front();
                    Poll::Ready(block)
                } else {
                    Poll::Pending
                }
            }
            Poll::Ready(Err(e)) => {
                tracing::error!("failed to fetch blocks in listener: {e:#?}");
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
