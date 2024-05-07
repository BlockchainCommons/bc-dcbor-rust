use bytes::Bytes;

use anyhow::{bail, Error, Result};

use crate::{CBOR, CBORCase};

impl From<Bytes> for CBOR {
    fn from(value: Bytes) -> Self {
        CBORCase::ByteString(value.clone()).into()
    }
}

impl TryFrom<CBOR> for Bytes {
    type Error = Error;

    fn try_from(value: CBOR) -> Result<Self> {
        match value.into_case() {
            CBORCase::ByteString(b) => Ok(b),
            _ => bail!("Cannot convert CBOR value to Bytes")
        }
    }
}

impl TryFrom<&CBOR> for Bytes {
    type Error = Error;

    fn try_from(value: &CBOR) -> Result<Self> {
        value.clone().try_into()
    }
}
