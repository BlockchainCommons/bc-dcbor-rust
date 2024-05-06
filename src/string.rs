import_stdlib!();

use anyhow::bail;

use crate::{CBOR, CBORError, CBORCase};

impl From<&str> for CBOR {
    fn from(value: &str) -> Self {
        CBORCase::Text(value.to_string()).into()
    }
}

impl From<String> for CBOR {
    fn from(value: String) -> Self {
        CBORCase::Text(value.clone()).into()
    }
}

impl TryFrom<CBOR> for String {
    type Error = anyhow::Error;
    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Text(s) => Ok(s),
            _ => bail!(CBORError::WrongType),
        }
    }
}
