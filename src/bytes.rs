use bytes::Bytes;

use crate::{CBOR, CBORCase};

impl From<Bytes> for CBOR {
    fn from(value: Bytes) -> Self {
        CBORCase::ByteString(value.clone()).into()
    }
}

impl TryFrom<CBOR> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: CBOR) -> anyhow::Result<Self> {
        match value.into_case() {
            CBORCase::ByteString(b) => Ok(b),
            _ => Err(anyhow::anyhow!("Cannot convert CBOR value to Bytes"))
        }
    }
}

impl TryFrom<&CBOR> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: &CBOR) -> anyhow::Result<Self> {
        value.clone().try_into()
    }
}
