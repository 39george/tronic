use crate::{
    client::{Client, TronProvider},
    domain::block::BlockExtention,
    signer::PrehashSigner,
};
use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{Duration, sleep};

#[derive(Clone)]
pub enum Message {
    Block(BlockExtention),
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
    pub fn into_stream(self) -> impl Stream<Item = Message> {
        ListenerStream {
            listener: self,
            delay: Box::pin(sleep(Duration::from_secs(0))),
            fut: None,
        }
    }
}

struct ListenerStream<P, S> {
    listener: Listener<P, S>,
    delay: Pin<Box<tokio::time::Sleep>>,
    fut: Option<Pin<Box<dyn Future<Output = Option<(i64, Message)>> + Send>>>,
}

impl<P, S> Unpin for ListenerStream<P, S> {}

impl<P, S> Stream for ListenerStream<P, S>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    type Item = Message;

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

                    Some((number, Message::Block(b)))
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
