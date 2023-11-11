use bytes::Bytes;

use crate::CBOR;

impl From<Bytes> for CBOR {
    fn from(value: Bytes) -> Self {
        CBOR::ByteString(value)
    }
}

impl From<&Bytes> for CBOR {
    fn from(value: &Bytes) -> Self {
        CBOR::ByteString(value.clone())
    }
}

impl TryFrom<CBOR> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: CBOR) -> Result<Self, Self::Error> {
        match value {
            CBOR::ByteString(b) => Ok(b),
            _ => Err(anyhow::anyhow!("Cannot convert {:?} to Bytes", value))
        }
    }
}

impl TryFrom<&CBOR> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: &CBOR) -> Result<Self, Self::Error> {
        match value {
            CBOR::ByteString(b) => Ok(b.clone()),
            _ => Err(anyhow::anyhow!("Cannot convert {:?} to Bytes", value))
        }
    }
}
