use std::{collections::HashSet, future::Future};

use crate::Filter;
use crate::domain::address::TronAddress;
use crate::domain::contract::Contract;
use crate::domain::transaction::TransactionExtention;

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
impl<F, Fut> Filter<TransactionExtention> for AddressFilter<F>
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
                contract.to_address().is_some_and(|a| {
                    println!("to is {}", a);
                    addrs.contains(&a)
                })
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
