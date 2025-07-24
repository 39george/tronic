use std::marker::PhantomData;
use std::{collections::HashSet, future::Future};

use crate::domain::address::TronAddress;
use crate::domain::contract::{Contract, TriggerSmartContract};
use crate::domain::transaction::TransactionExtention;
use crate::{AddressExtractor, Filter};

#[derive(Clone)]
pub struct AddressFilter<F, E> {
    fetch_addrs: F,
    extractor: PhantomData<E>,
}

impl<F> AddressFilter<F, ()> {
    pub fn new(fetch_addrs: F) -> AddressFilter<F, ()> {
        AddressFilter::<F, ()> {
            fetch_addrs,
            extractor: Default::default(),
        }
    }
    pub fn with_extractor<NewE>(self) -> AddressFilter<F, NewE> {
        AddressFilter::<F, NewE> {
            fetch_addrs: self.fetch_addrs,
            extractor: Default::default(),
        }
    }
}

#[async_trait::async_trait]
impl<F, Fut, E> Filter<TransactionExtention> for AddressFilter<F, E>
where
    F: FnOnce() -> Fut + Send + Sync + Clone,
    Fut: Future<Output = Option<HashSet<TronAddress>>> + Send,
    E: AddressExtractor<TriggerSmartContract> + Send + Sync + Clone,
{
    async fn filter(&self, txext: TransactionExtention) -> bool {
        if let Some(contract) = txext.get_contract() {
            let addrs = (self.fetch_addrs.clone())().await;
            let mut found = false;
            if let Some(ref addrs) = addrs {
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
                    let address = E::extract(contract);
                    found =
                        address.is_some_and(|a| addrs.contains(&a)) || found;
                }
                found = check_owner(&contract)
                    || check_to(&contract)
                    || check_contract(&contract)
                    || found;
            }
            found || addrs.is_none()
        } else {
            false
        }
    }
}
