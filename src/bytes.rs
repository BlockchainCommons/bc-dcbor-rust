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
        match value.case() {
            CBORCase::ByteString(b) => Ok(b.clone()),
            _ => Err(anyhow::anyhow!("Cannot convert {:?} to Bytes", value))
        }
    }
}

impl TryFrom<&CBOR> for Bytes {
    type Error = anyhow::Error;

    fn try_from(value: &CBOR) -> anyhow::Result<Self> {
        value.clone().try_into()
    }
}
