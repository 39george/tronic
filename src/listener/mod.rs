use futures::stream::FuturesOrdered;
use futures::{Stream, StreamExt, TryStreamExt};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

use crate::Result;
use crate::client::Client;
use crate::domain::block::BlockExtention;
use crate::listener::block_cache::{BlockCache, InMemoryBlockCache};
use crate::provider::TronProvider;
use crate::{listener::subscriber::BlockSubscriber, signer::PrehashSigner};

pub mod block_cache;
pub mod subscriber;

const MAX_BLOCKS_PER_FETCH: i64 = 100;

#[derive(Clone, Debug)]
pub struct ListenerError(Arc<crate::error::Error>);

impl std::ops::Deref for ListenerError {
    type Target = crate::error::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<crate::error::Error> for ListenerError {
    fn from(e: crate::error::Error) -> Self {
        Self(Arc::new(e))
    }
}

impl std::fmt::Display for ListenerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.0, f)
    }
}

impl std::error::Error for ListenerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

pub type ListenerMsg =
    std::result::Result<crate::domain::block::BlockExtention, ListenerError>;

pub struct ListenerHandle {
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    rx: tokio::sync::broadcast::Receiver<ListenerMsg>,
}

impl ListenerHandle {
    pub fn subscribe<S: BlockSubscriber + Send + Sync + 'static>(
        &self,
        subscriber: S,
    ) {
        let mut rx = self.rx.resubscribe();

        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => subscriber.handle(msg).await,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(
                        n,
                    )) => {
                        tracing::warn!("subscriber lagged by {n} messages");
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });
    }
}

impl Drop for ListenerHandle {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

pub struct Listener<P, S, BC = InMemoryBlockCache> {
    client: Client<P, S>,
    block_cache: BC,
    interval: Duration,
}

impl<P, S> Listener<P, S, InMemoryBlockCache>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    pub fn new(client: Client<P, S>, block_poll_interval: Duration) -> Self {
        Self {
            client,
            block_cache: InMemoryBlockCache::default(),
            interval: block_poll_interval,
        }
    }
}

impl<P, S, BC> Listener<P, S, BC>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
    BC: BlockCache + Clone,
{
    pub fn new_with_block_cache(
        client: Client<P, S>,
        block_poll_interval: Duration,
        block_cache: BC,
    ) -> Self {
        Self {
            client,
            block_cache,
            interval: block_poll_interval,
        }
    }

    pub fn with_block_cache<BC2>(self, block_cache: BC2) -> Listener<P, S, BC2>
    where
        BC2: BlockCache + Clone,
    {
        Listener {
            client: self.client,
            block_cache,
            interval: self.interval,
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
                        if let Err(e) = tx.send(msg) {
                            tracing::error!("failed to send block msg: {e}");
                        }
                    }
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

    pub(crate) fn block_stream(self) -> impl Stream<Item = ListenerMsg> {
        BlockStream {
            listener: self,
            delay: Box::pin(sleep(Duration::from_secs(0))),
            fut: None,
            pending_blocks: VecDeque::new(),
        }
    }
}

struct BlockStream<P, S, BC> {
    listener: Listener<P, S, BC>,
    delay: Pin<Box<tokio::time::Sleep>>,
    fut: Option<
        Pin<Box<dyn Future<Output = Result<Vec<BlockExtention>>> + Send>>,
    >,
    pending_blocks: VecDeque<BlockExtention>,
}

impl<P, S, BC> Unpin for BlockStream<P, S, BC> {}

impl<P, S, BC> Stream for BlockStream<P, S, BC>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
    BC: BlockCache + Clone,
{
    type Item = ListenerMsg;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some(block) = self.pending_blocks.pop_front() {
            return Poll::Ready(Some(Ok(block)));
        }

        if self.delay.as_mut().poll(cx).is_pending() {
            return Poll::Pending;
        }

        if self.fut.is_none() {
            let client = self.listener.client.clone();
            let block_cache = self.listener.block_cache.clone();

            self.fut = Some(Box::pin(async move {
                let last_block =
                    match block_cache.load_latest_seen_block().await {
                        Ok(Some(block_number)) => block_number,
                        Ok(None) => -1,
                        Err(e) => return Err(e.into()),
                    };

                let latest_block = client.provider.get_now_block().await?;
                let latest_number = latest_block.block_header.raw_data.number;

                let blocks_to_send = if last_block == -1 {
                    vec![latest_block]
                } else if latest_number <= last_block {
                    Vec::new()
                } else {
                    let first_needed = last_block + 1;
                    let batch_end_number =
                        (first_needed + MAX_BLOCKS_PER_FETCH - 1)
                            .min(latest_number);

                    let futures: FuturesOrdered<_> =
                        (first_needed..=batch_end_number)
                            .map(|num| {
                                let provider = client.provider();
                                async move {
                                    provider.get_block_by_number(num).await
                                }
                            })
                            .collect();

                    futures.try_collect().await?
                };

                if let Some(last_block) = blocks_to_send.last() {
                    let block_number = last_block.block_header.raw_data.number;

                    if let Err(e) =
                        block_cache.store_latest_seen_block(block_number).await
                    {
                        return Err(e.into());
                    }
                }

                Ok(blocks_to_send)
            }));
        }

        let interval = self.listener.interval;

        let poll_result = {
            let fut = self.fut.as_mut().expect("future must exist");
            fut.as_mut().poll(cx)
        };

        match poll_result {
            Poll::Ready(Ok(blocks)) => {
                self.fut = None;
                self.delay
                    .as_mut()
                    .reset(tokio::time::Instant::now() + interval);

                if blocks.is_empty() {
                    return Poll::Pending;
                }

                self.pending_blocks.extend(blocks);

                let block = self
                    .pending_blocks
                    .pop_front()
                    .expect("pending blocks must not be empty");

                Poll::Ready(Some(Ok(block)))
            }
            Poll::Ready(Err(e)) => {
                self.fut = None;
                self.delay
                    .as_mut()
                    .reset(tokio::time::Instant::now() + interval);

                Poll::Ready(Some(Err(ListenerError::from(e))))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
