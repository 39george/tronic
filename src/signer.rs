use k256::ecdsa::{RecoveryId, Signature, SigningKey, signature::Signer};

use crate::domain::{Hash32, address::TronAddress};

#[async_trait::async_trait]
pub trait PrehashSigner {
    type Ctx;
    type Error;
    async fn sign(
        &self,
        txid: &Hash32,
        ctx: &Self::Ctx,
    ) -> Result<Signature, Self::Error>;
    fn recovery_id(
        &self,
        txid: &Hash32,
        signature: &Signature,
    ) -> Result<RecoveryId, Self::Error>;
    fn address(&self) -> Option<TronAddress> {
        None
    }
}

#[derive(Clone)]
pub struct LocalSigner {
    signing_key: SigningKey,
}

impl LocalSigner {
    pub fn rand() -> Self {
        let mut rng = k256::elliptic_curve::rand_core::OsRng;
        LocalSigner {
            signing_key: SigningKey::random(&mut rng),
        }
    }
    pub fn from_bytes(buf: &[u8]) -> crate::Result<Self> {
        Ok(LocalSigner {
            signing_key: SigningKey::from_slice(buf)?,
        })
    }
    pub fn tron_address(&self) -> TronAddress {
        self.signing_key
            .verifying_key()
            .try_into()
            .expect("valid key must produce Tron address")
    }
}

impl From<SigningKey> for LocalSigner {
    fn from(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }
}

#[async_trait::async_trait]
impl PrehashSigner for LocalSigner {
    type Error = k256::ecdsa::signature::Error;
    type Ctx = ();
    async fn sign(
        &self,
        txid: &Hash32,
        _: &Self::Ctx,
    ) -> Result<Signature, Self::Error> {
        let s = self.signing_key.sign(&Vec::<u8>::from(*txid));
        Ok(s)
    }
    fn recovery_id(
        &self,
        txid: &Hash32,
        signature: &Signature,
    ) -> Result<RecoveryId, Self::Error> {
        let verifying_key = self.signing_key.verifying_key();
        let recovery_id = k256::ecdsa::RecoveryId::trial_recovery_from_prehash(
            verifying_key,
            &Vec::<u8>::from(*txid),
            signature,
        )?;
        Ok(recovery_id)
    }
    fn address(&self) -> Option<TronAddress> {
        Some(self.tron_address())
    }
}
