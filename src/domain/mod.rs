use anyhow::{Context, anyhow};
use k256::ecdsa::{RecoveryId, Signature};

pub mod account;
pub mod address;
pub mod block;
pub mod contract;
pub mod transaction;
pub mod trx;

#[derive(Clone, PartialEq)]
pub struct IdHash(pub Vec<u8>);

impl From<Vec<u8>> for IdHash {
    fn from(buf: Vec<u8>) -> Self {
        Self(buf)
    }
}

#[derive(Clone, PartialEq)]
pub struct RecoverableSignature {
    signature: k256::ecdsa::Signature,
    recovery_id: k256::ecdsa::RecoveryId,
}

impl RecoverableSignature {
    pub fn new(
        signature: k256::ecdsa::Signature,
        recovery_id: k256::ecdsa::RecoveryId,
    ) -> Self {
        RecoverableSignature {
            signature,
            recovery_id,
        }
    }
}

impl From<RecoverableSignature> for Vec<u8> {
    fn from(s: RecoverableSignature) -> Self {
        // Append recovery_id (1 byte) to the signature (64 bytes) directly
        let mut signature_bytes = Vec::with_capacity(65);
        signature_bytes.extend_from_slice(&s.signature.to_vec()); // 64 bytes
        signature_bytes.push(s.recovery_id.to_byte()); // 1 byte
        signature_bytes
    }
}

impl TryFrom<&[u8]> for RecoverableSignature {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 65 {
            return Err(anyhow!(
                "bad signature length: {}, should be 65",
                value.len()
            ));
        }
        let recovery_byte = value.last().unwrap();
        let recovery_id = RecoveryId::from_byte(*recovery_byte)
            .context("recovery byte exceedes up limit")?;
        Ok(RecoverableSignature {
            signature: Signature::from_slice(&value[..64])
                .expect("bad signature"),
            recovery_id,
        })
    }
}
