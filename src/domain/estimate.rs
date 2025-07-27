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
pub struct ResourceRequirements {
    pub bandwidth: i64,
    pub energy: i64,
    pub trx: Trx,
}

#[derive(Debug)]
pub enum ResourceState {
    Sufficient {
        will_consume: ResourceRequirements,
        remaining: ResourceRequirements,
    },
    Insufficient {
        missing: Vec<MissingResource>,
        /// Suggested TRX fee to cover deficits
        suggested_trx_topup: Vec<(MissingResource, Trx)>,
        account_balance: Trx,
    },
}

impl ResourceState {
    pub async fn estimate<P, S>(
        client: &Client<P, S>,
        resources: &AccountResourceUsage,
        required: ResourceRequirements,
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

        if available_energy < required.energy {
            missing.push(MissingResource::Energy {
                available: available_energy,
                required: required.energy,
            });
        }

        if balance < required.trx {
            missing.push(MissingResource::Trx {
                available: balance,
                required: required.trx,
            });
        }

        if missing.is_empty() {
            Ok(ResourceState::Sufficient {
                will_consume: required,
                remaining: ResourceRequirements {
                    bandwidth: staked_b.max(free_b) - required.bandwidth,
                    energy: available_energy - required.energy,
                    trx: balance - required.trx,
                },
            })
        } else {
            let energy_price = client.energy_price().await?;
            let bandwidth_price = client.bandwidth_price().await?;
            Ok(ResourceState::Insufficient {
                suggested_trx_topup: calculate_topup(
                    &missing,
                    energy_price,
                    bandwidth_price,
                ),
                missing,
                account_balance: balance,
            })
        }
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
                available: _,
            } => {
                // let deficit = required.saturating_sub(*available);
                // (res.clone(), energy_price * deficit)
                (res.clone(), energy_price * required)
            }
            MissingResource::Bandwidth {
                required,
                available: _,
            } => {
                // let deficit = required.saturating_sub(*available);
                // (res.clone(), bandwidth_price * deficit)
                (res.clone(), bandwidth_price * required)
            }
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
