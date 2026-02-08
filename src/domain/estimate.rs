use crate::{
    client::Client, domain::account::AccountResourceUsage,
    provider::TronProvider, signer::PrehashSigner,
};

use super::trx::Trx;

#[derive(Debug, Clone, thiserror::Error)]
pub enum MissingResource {
    #[error(
        "Insufficient bandwidth (available: {available}, required: {required})"
    )]
    Bandwidth { available: i64, required: i64 },
    #[error(
        "Insufficient energy (available: {available}, required: {required})"
    )]
    Energy { available: i64, required: i64 },
    #[error(
        "Insufficient TRX (available: {available} SUN, required: {required})"
    )]
    Trx { available: Trx, required: Trx },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Resource {
    pub bandwidth: i64,
    pub energy: i64,
    pub trx: Trx,
}

#[derive(Debug)]
pub struct ResourceState {
    pub will_consume: Resource,
    pub remaining: Resource,
    pub insufficient: Option<InsufficientResource>,
}

#[derive(Debug)]
pub struct InsufficientResource {
    pub missing: Vec<MissingResource>,
    /// Suggested TRX fee to cover deficits
    pub suggested_trx_topup: Vec<(MissingResource, Trx)>,
    pub account_balance: Trx,
}

impl ResourceState {
    pub async fn estimate<P, S>(
        client: &Client<P, S>,
        resources: &AccountResourceUsage,
        required: Resource,
        balance: Trx,
    ) -> crate::Result<Self>
    where
        P: TronProvider,
        S: PrehashSigner,
    {
        let mut missing = Vec::new();

        // Note: Bandwidth (free and staked) cannot be used for the same transaction,
        // with Bandwidth "From staked TRX/From others' delegation" consumed first.
        let (free_b, staked_b) = (
            resources.free_net_limit - resources.free_net_used,
            resources.net_limit - resources.net_used,
        );

        let available_energy = resources.energy_limit - resources.energy_used;

        // Validate each resource
        if staked_b < required.bandwidth && free_b < required.bandwidth {
            missing.push(MissingResource::Bandwidth {
                available: staked_b.max(free_b),
                required: required.bandwidth,
            });
        }

        let sufficient_energy = available_energy >= required.energy;
        if !sufficient_energy {
            missing.push(MissingResource::Energy {
                available: available_energy,
                required: required.energy,
            });
        }
        let sufficient_balance = balance >= required.trx;
        if !sufficient_balance {
            missing.push(MissingResource::Trx {
                available: balance,
                required: required.trx,
            });
        }

        if missing.is_empty() {
            Ok(ResourceState {
                will_consume: required,
                remaining: Resource {
                    bandwidth: staked_b.max(free_b) - required.bandwidth,
                    energy: available_energy - required.energy,
                    trx: balance - required.trx,
                },
                insufficient: None,
            })
        } else {
            let (energy_price, bandwidth_price) = tokio::try_join!(
                client.energy_price(),
                client.bandwidth_price()
            )?;
            let remaining_energy = if sufficient_energy {
                available_energy - required.energy
            } else {
                available_energy
            };
            let remaining_balance = if sufficient_balance {
                balance - required.trx
            } else {
                balance
            };
            Ok(ResourceState {
                will_consume: required,
                remaining: Resource {
                    bandwidth: staked_b.max(free_b),
                    energy: remaining_energy,
                    trx: remaining_balance,
                },
                insufficient: Some(InsufficientResource {
                    suggested_trx_topup: calculate_topup(
                        &missing,
                        energy_price,
                        bandwidth_price,
                    ),
                    missing,
                    account_balance: balance,
                }),
            })
        }
    }

    pub fn trx_required(&self) -> Trx {
        let mut total_trx = self.will_consume.trx;
        if let Some(ref insufficinet) = self.insufficient {
            total_trx += insufficinet
                .suggested_trx_topup
                .iter()
                .map(|(_, trx)| *trx)
                .sum();
        }
        total_trx
    }
}

/// Calculates per-resource TRX top-up suggestion
pub(crate) fn calculate_topup(
    missing: &[MissingResource],
    energy_price: Trx,
    bandwidth_price: Trx,
) -> Vec<(MissingResource, Trx)> {
    missing
        .iter()
        .map(|res| match res {
            MissingResource::Energy {
                required,
                available,
            } => {
                let deficit = required.saturating_sub(*available);
                (res.clone(), energy_price * deficit)
            }
            // Warn: we can't calculate deficit if we selected free_b here
            MissingResource::Bandwidth {
                required,
                available: _,
            } => (res.clone(), bandwidth_price * required),
            MissingResource::Trx {
                required,
                available,
            } => {
                let deficit = *required - *available;
                (res.clone(), deficit)
            }
        })
        .collect()
}
