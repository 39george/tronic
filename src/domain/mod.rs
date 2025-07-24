use std::ops::Deref;

use anyhow::{Context, anyhow};
use k256::ecdsa::{RecoveryId, Signature};

pub mod account;
pub mod address;
pub mod block;
pub mod contract;
pub mod transaction;
pub mod trx;

macro_rules! define_fixed_hash {
    ($name:ident, $len:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub [u8; $len]);

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "0x{}", hex::encode(&self.0))
            }
        }

        impl TryFrom<Vec<u8>> for $name {
            type Error = String;
            fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
                value.try_into().map(Self).map_err(|v: Vec<u8>| {
                    format!(
                        "invalid {} length: got {}, expected {}",
                        stringify!($name),
                        v.len(),
                        $len
                    )
                })
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(value: $name) -> Self {
                value.0.to_vec()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                tracing::warn!("default {} value", stringify!($name));
                Self([0u8; $len])
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}

define_fixed_hash!(RefBlockBytes, 2, "2-byte reference to block number");
define_fixed_hash!(RefBlockHash, 8, "8-byte truncated block hash");
define_fixed_hash!(Hash32, 32, "32-byte block or transaction hash");
define_fixed_hash!(AccountStateRoot, 32, "32-byte root hash of account state");
define_fixed_hash!(TxTrieRoot, 32, "32-byte transaction trie root");
define_fixed_hash!(ParentHash, 32, "32-byte parent block hash");

#[derive(Debug, Clone, PartialEq)]
pub struct HexMessage(String);

impl From<Vec<u8>> for HexMessage {
    fn from(value: Vec<u8>) -> Self {
        HexMessage(hex::encode(value))
    }
}

impl From<HexMessage> for Vec<u8> {
    fn from(value: HexMessage) -> Self {
        hex::decode(value.0).unwrap_or_default()
    }
}

impl HexMessage {
    pub fn to_vec(self) -> Vec<u8> {
        Vec::<u8>::from(self)
    }
}

impl Deref for HexMessage {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Message(String);

impl From<Vec<u8>> for Message {
    fn from(value: Vec<u8>) -> Self {
        Message(String::from_utf8_lossy(&value).into())
    }
}

impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Message(value.into())
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        value.0.as_bytes().to_vec()
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for Message {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        // if value.len() != 65 {
        //     return Err(anyhow!(
        //         "bad signature length: {}, should be 65",
        //         value.len()
        //     ));
        // }
        let value = if value.len() == 68 {
            &value[..65]
        } else if value.len() == 65 {
            value
        } else {
            return Err(anyhow!(
                "Bad signature length: {}, should be 65 or 68",
                value.len()
            ));
        };
        let recovery_byte = *value.last().unwrap();

        let recovery_byte_normalized = match recovery_byte {
            0..=3 => recovery_byte,        // some direct recovery IDs
            27..=30 => recovery_byte - 27, // Per [TIP 120](https://github.com/tronprotocol/tips/issues/120)
            31..=34 => recovery_byte - 31, // older/broken clients that added 4
            _ => return Err(anyhow!("Invalid recovery byte: {recovery_byte}")),
        };
        let recovery_id = RecoveryId::from_byte(recovery_byte_normalized)
            .context(format!("can't parse recovery byte: {recovery_byte}"))?;

        Ok(RecoverableSignature {
            signature: Signature::from_slice(&value[..64])
                .expect("bad signature"),
            recovery_id,
        })
    }
}
