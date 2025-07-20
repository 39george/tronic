use core::fmt::{Debug, Display};
use core::str::FromStr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("coordinate not found")]
    NoCoordinate,
}

crate::impl_debug!(Error);

/// Account address struct
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TronAddress([u8; 21]);

// TODO: refine error handling
impl TronAddress {
    pub const ZERO: TronAddress = TronAddress([
        0x41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    /// Construct new address from bytes (expected with 0x41 prefix)
    pub fn new(bytes: [u8; 21]) -> Result<Self, anyhow::Error> {
        if bytes[0] == 0x41 {
            Ok(Self(bytes))
        } else {
            Err(anyhow!("bad address"))
        }
    }

    pub fn rand() -> Self {
        use k256::ecdsa::SigningKey;
        use k256::elliptic_curve::SecretKey;

        // Generate random secret key
        let mut rng = k256::elliptic_curve::rand_core::OsRng;
        let sk = SecretKey::random(&mut rng);
        let sk = SigningKey::from(sk);
        sk.verifying_key()
            .try_into()
            .expect("valid key must produce Tron address")
    }

    pub fn from_pk(pk_bytes: &[u8]) -> Result<Self, anyhow::Error> {
        let verifying_key =
            &k256::ecdsa::VerifyingKey::from_sec1_bytes(pk_bytes)?;
        let addr = verifying_key.try_into()?;
        Ok(addr)
    }

    /// Get base58 representation
    pub fn as_base58(&self) -> String {
        bs58::encode(&self.0).with_check().into_string()
    }

    /// Get hex representation
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Get raw address bytes (including 0x41 prefix)
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Default for TronAddress {
    fn default() -> Self {
        TronAddress::ZERO
    }
}

/// Parse address from base58 or hex string
impl FromStr for TronAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s)
            .with_check(None)
            .into_vec()
            .or_else(|_| hex::decode(s))
            .map_err(|_| anyhow!("bad address"))?;
        Ok(Self(bytes.try_into().map_err(|_| anyhow!("bad address"))?))
    }
}

impl TryFrom<&k256::ecdsa::VerifyingKey> for TronAddress {
    type Error = Error;
    fn try_from(
        verifying_key: &k256::ecdsa::VerifyingKey,
    ) -> Result<Self, Self::Error> {
        let point = verifying_key.to_encoded_point(false);
        let x = point.x().ok_or(Error::NoCoordinate)?;
        let y = point.y().ok_or(Error::NoCoordinate)?;
        let mut p_bytes = Vec::new();
        p_bytes.extend_from_slice(&[0x04]);
        p_bytes.extend_from_slice(x.as_slice());
        p_bytes.extend_from_slice(y.as_slice());
        if p_bytes.len() == 65 {
            p_bytes.remove(0);
        }
        let hash = sha3::Keccak256::digest(&p_bytes);
        let mut addr = [0x41; 21];
        addr[1..].copy_from_slice(&hash.as_slice()[hash.len() - 20..]);

        Ok(TronAddress(addr))
    }
}

impl TryFrom<&[u8]> for TronAddress {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let bytes: [u8; 21] = slice.try_into().map_err(|_| {
            anyhow!("slice length must be 21 bytes, got: {}", slice.len())
        })?;
        Self::new(bytes)
    }
}

impl TryFrom<&Vec<u8>> for TronAddress {
    type Error = anyhow::Error;

    fn try_from(vec: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(vec.as_slice())
    }
}

impl From<TronAddress> for alloy_primitives::Address {
    fn from(tron_address: TronAddress) -> Self {
        alloy_primitives::Address::from_slice(&tron_address.as_bytes()[1..21])
    }
}

impl Debug for TronAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_base58())
    }
}

impl Display for TronAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_base58())
    }
}

// TODO: make serde feature?
impl Serialize for TronAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_base58())
    }
}

impl<'de> Deserialize<'de> for TronAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use k256::ecdsa::VerifyingKey;

    use super::*;

    #[test]
    fn test_address_full() {
        let hex = "418840E6C55B9ADA326D211D818C34A994AECED808";
        let b58 = "TNPeeaaFB7K9cmo4uQpcU32zGK8G1NYqeL";
        let bytes = hex::decode(hex).unwrap();

        let a1 =
            TronAddress::new(bytes.try_into().unwrap()).expect("Address::new");
        let a2: TronAddress = b58.parse().expect("b58 parse");
        let a3: TronAddress = hex.parse().expect("hex parse");

        assert!(a1 == a2 && a2 == a3, "address mismatch");
        assert_eq!(a1.as_base58(), b58, "b58 mismatch");
        assert_eq!(a1.as_hex().to_ascii_uppercase(), hex, "hex mismatch");
    }

    #[test]
    fn test_from_verifying_key() {
        let addr = "TPgzPCV8abFBaWEyh7kBeDdiMFcKEX2CJE";
        let pk_bytes = hex::decode(
            "41b66b36c9d31903a170f664fdd6fc8e7a575b3e753a61a609b0170561c29f95",
        )
        .unwrap();
        let signing_key =
            k256::ecdsa::SigningKey::from_slice(&pk_bytes).unwrap();
        let verifying_key = signing_key.verifying_key();
        let tron_addr: Result<TronAddress, _> = verifying_key.try_into();
        assert!(tron_addr.is_ok());
        assert_eq!(tron_addr.unwrap().as_base58(), addr);
    }

    #[test]
    fn test_from_prv_key() {
        let bytes = hex::decode(
        "0291e9cba70e8124c5ddc47d4e6f54e9ed7f942995d3302086aec70b2eae13794b"
    )
    .unwrap();
        let verifying_key = VerifyingKey::from_sec1_bytes(&bytes).unwrap();
        let tron_addr: Result<TronAddress, _> = (&verifying_key).try_into();
        println!("tron addr is: {}", tron_addr.unwrap());
    }
}
