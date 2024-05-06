use crate::CBOR;

pub trait CBOREncodable: Into<CBOR> {
    fn into_cbor(self) -> CBOR {
        self.into()
    }

    fn into_cbor_data(self) -> Vec<u8> {
        self.into().cbor_data()
    }
}

impl<T> CBOREncodable for T where T: Into<CBOR> { }

pub trait CBORDecodable: TryFrom<CBOR>
{
    fn from_cbor(cbor: CBOR) -> anyhow::Result<Self>
    where <Self as TryFrom<CBOR>>::Error: Into<anyhow::Error>
    {
        Self::try_from(cbor).map_err(|e| e.into())
    }

    fn from_cbor_data(data: &[u8]) -> anyhow::Result<Self>
    where
    <Self as TryFrom<CBOR>>::Error: Into<anyhow::Error> + std::error::Error + Send + Sync + 'static,
    {
        let cbor = CBOR::from_data(data)?;
        let result: Self = Self::try_from(cbor)?;
        Ok(result)
    }
}

impl<T> CBORDecodable for T where T: TryFrom<CBOR> { }

/// A type that can be encoded to or decoded from CBOR.
pub trait CBORCodable { }

impl<T> CBORCodable for T where T: CBORDecodable + CBOREncodable { }
