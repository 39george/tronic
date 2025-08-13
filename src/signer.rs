use std::sync::Arc;

use k256::ecdsa::SigningKey;

use crate::domain::{Hash32, RecoverableSignature, address::TronAddress};

#[async_trait::async_trait]
pub trait PrehashSigner {
    type Ctx;
    type Error;
    async fn sign_recoverable(
        &self,
        txid: &Hash32,
        ctx: &Self::Ctx,
    ) -> Result<RecoverableSignature, Self::Error>;
    fn address(&self) -> Option<TronAddress> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct LocalSigner {
    signing_key: Arc<SigningKey>,
}

impl LocalSigner {
    pub fn rand() -> Self {
        let mut rng = k256::elliptic_curve::rand_core::OsRng;
        LocalSigner {
            signing_key: Arc::new(SigningKey::random(&mut rng)),
        }
    }
    pub fn from_bytes(buf: &[u8]) -> crate::Result<Self> {
        Ok(LocalSigner {
            signing_key: Arc::new(SigningKey::from_slice(buf)?),
        })
    }
    pub fn address(&self) -> TronAddress {
        self.signing_key
            .verifying_key()
            .try_into()
            .expect("valid key must produce Tron address")
    }
    pub fn secret_key(&self) -> [u8; 32] {
        self.signing_key.to_bytes().to_vec().try_into().unwrap()
    }
}

impl From<SigningKey> for LocalSigner {
    fn from(signing_key: SigningKey) -> Self {
        Self {
            signing_key: Arc::new(signing_key),
        }
    }
}

#[async_trait::async_trait]
impl PrehashSigner for LocalSigner {
    type Error = k256::ecdsa::signature::Error;
    type Ctx = ();
    async fn sign_recoverable(
        &self,
        txid: &Hash32,
        _: &Self::Ctx,
    ) -> Result<RecoverableSignature, Self::Error> {
        let (signature, recovery_id) = self
            .signing_key
            .sign_prehash_recoverable(&Vec::<u8>::from(*txid))?;
        Ok(RecoverableSignature::new(signature, recovery_id))
    }
    fn address(&self) -> Option<TronAddress> {
        Some(self.address())
    }
}
