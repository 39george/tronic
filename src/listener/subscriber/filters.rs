use std::marker::PhantomData;
use std::{collections::HashSet, future::Future};

use crate::Filter;
use crate::contracts::token::{InMemoryTokenRegistry, TokenRegistry};
use crate::domain::address::TronAddress;
use crate::domain::block::BlockExtention;
use crate::domain::contract::{Contract, TriggerSmartContract};
use crate::domain::transaction::TransactionExtention;
use crate::extractor::AddressExtractor;

#[derive(Clone)]
pub struct AddressFilter<F, R, E> {
    fetch_addrs: F,
    token_registry: R,
    extractor: PhantomData<E>,
}

impl<F> AddressFilter<F, InMemoryTokenRegistry, ()> {
    pub fn new(fetch_addrs: F) -> AddressFilter<F, InMemoryTokenRegistry, ()> {
        AddressFilter::<F, InMemoryTokenRegistry, ()> {
            fetch_addrs,
            extractor: Default::default(),
            token_registry: Default::default(),
        }
    }
}

impl<F, R, E> AddressFilter<F, R, E> {
    /// Type to use as TriggerSmartContract calls extractor (from bytecode)
    pub fn with_extractor<NewE>(self) -> AddressFilter<F, R, NewE> {
        AddressFilter::<F, R, NewE> {
            fetch_addrs: self.fetch_addrs,
            token_registry: self.token_registry,
            extractor: Default::default(),
        }
    }
    pub fn with_registry<NewR>(
        self,
        registry: NewR,
    ) -> AddressFilter<F, NewR, E> {
        AddressFilter::<F, NewR, E> {
            fetch_addrs: self.fetch_addrs,
            token_registry: registry,
            extractor: Default::default(),
        }
    }
    pub fn registry_mut(&mut self) -> &mut R {
        &mut self.token_registry
    }
}

#[derive(Clone)]
pub struct FilterCtx<R> {
    pub registry: R,
    pub trigger: TriggerSmartContract,
}

#[async_trait::async_trait]
impl<F, Fut, R, E> Filter<BlockExtention> for AddressFilter<F, R, E>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = HashSet<TronAddress>> + Send,
    E: AddressExtractor<FilterCtx<R>> + Send + Sync + Clone,
    R: TokenRegistry + Send + Sync + Clone,
{
    type Item = TransactionExtention;
    async fn filter(&self, block: BlockExtention) -> Vec<Self::Item> {
        let addrs = (self.fetch_addrs)().await;

        block
            .transactions
            .into_iter()
            .filter(|tx| {
                contains_addr::<R, E>(tx, &addrs, &self.token_registry)
            })
            .collect()
    }
}

fn contains_addr<R, E>(
    txext: &TransactionExtention,
    addrs: &HashSet<TronAddress>,
    registry: &R,
) -> bool
where
    E: AddressExtractor<FilterCtx<R>> + Send + Sync + Clone,
    R: TokenRegistry + Send + Sync + Clone,
{
    if let Some(contract) = txext.get_contract() {
        let mut found = false;
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
        if let Some(contract) = contract.trigger_smart_contract() {
            let address = E::extract(FilterCtx {
                registry: registry.clone(),
                trigger: contract,
            });
            found = address.is_some_and(|a| addrs.contains(&a)) || found;
        }
        check_owner(&contract)
            || check_to(&contract)
            || check_contract(&contract)
            || found
    } else {
        false
    }
}
