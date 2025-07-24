use std::{collections::HashSet, future::Future};

use crate::client::{Client, TronProvider};
use crate::domain::contract::Contract;
use crate::domain::{
    address::TronAddress,
    block::BlockExtention,
    transaction::{Transaction, TransactionExtention},
};

use super::BlockSubscriber;

#[async_trait::async_trait]
pub trait Filter {
    async fn filter(&self, txext: TransactionExtention) -> bool;
}

#[async_trait::async_trait]
impl<F, Fut> Filter for F
where
    F: Fn(TransactionExtention) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = bool> + Send,
{
    async fn filter(&self, txext: TransactionExtention) -> bool {
        self(txext).await
    }
}

// Default filter type that always returns true
#[derive(Clone)]
pub struct DefaultFilter;

#[async_trait::async_trait]
impl Filter for DefaultFilter {
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
        NewF: Filter,
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
    F: Filter + Send + Sync + Clone,
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

#[derive(Default, Clone)]
pub enum AddressFilterKind {
    #[default]
    Account,
    Contract,
    Both,
}

#[derive(Clone)]
pub struct AddressFilter<F> {
    fetch_addrs: F,
    kind: AddressFilterKind,
}

impl<F> AddressFilter<F> {
    pub fn new(fetch_addrs: F) -> Self {
        Self {
            fetch_addrs,
            kind: Default::default(),
        }
    }
    pub fn with_kind(mut self, kind: AddressFilterKind) -> Self {
        self.kind = kind;
        self
    }
}

#[async_trait::async_trait]
impl<F, Fut> Filter for AddressFilter<F>
where
    F: FnOnce() -> Fut + Send + Sync + Clone,
    Fut: Future<Output = HashSet<TronAddress>> + Send,
{
    async fn filter(&self, txext: TransactionExtention) -> bool {
        if let Some(contract) = txext.get_contract() {
            let addrs = (self.fetch_addrs.clone())().await;
            let check_owner = |contract: &Contract| {
                contract.owner_address().is_some_and(|a| addrs.contains(&a))
            };

            let check_to = |contract: &Contract| {
                contract.to_address().is_some_and(|a| addrs.contains(&a))
            };

            let check_contract = |contract: &Contract| {
                contract
                    .contract_address()
                    .is_some_and(|a| addrs.contains(&a))
            };

            match self.kind {
                AddressFilterKind::Account => {
                    check_owner(&contract) || check_to(&contract)
                }
                AddressFilterKind::Contract => check_contract(&contract),
                AddressFilterKind::Both => {
                    check_owner(&contract)
                        || check_to(&contract)
                        || check_contract(&contract)
                }
            }
        } else {
            false
        }
    }
}
