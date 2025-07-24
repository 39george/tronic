pub mod trc20;

pub trait TryFromData: Sized {
    type Error;
    fn try_from_data(data: &[u8]) -> Result<Self, Self::Error>;
}

pub trait AbiEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait AbiDecode: Sized {
    type Error;
    fn decode(data: &[u8]) -> Result<Self, Self::Error>;
}
