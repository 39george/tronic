use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
};

pub trait BlockCache: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;

    fn store_latest_seen_block(
        &self,
        block_number: i64,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    fn load_latest_seen_block(
        &self,
    ) -> impl Future<Output = std::result::Result<Option<i64>, Self::Error>> + Send;
}

#[derive(Clone)]
pub struct InMemoryBlockCache {
    latest: Arc<AtomicI64>,
}

impl Default for InMemoryBlockCache {
    fn default() -> Self {
        Self {
            latest: Arc::new(AtomicI64::new(-1)),
        }
    }
}

impl BlockCache for InMemoryBlockCache {
    type Error = crate::error::Error;

    fn store_latest_seen_block(
        &self,
        block_number: i64,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        let latest = self.latest.clone();

        async move {
            latest.store(block_number, Ordering::Release);
            Ok(())
        }
    }

    fn load_latest_seen_block(
        &self,
    ) -> impl Future<Output = std::result::Result<Option<i64>, Self::Error>> + Send
    {
        let latest = self.latest.clone();

        async move {
            let block_number = latest.load(Ordering::Acquire);
            Ok((block_number >= 0).then_some(block_number))
        }
    }
}
