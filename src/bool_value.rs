import_stdlib!();

use crate::{CBOR, Simple, CBORDecodable, CBORError, CBORCase};

use anyhow::bail;

impl From<bool> for CBOR {
    fn from(value: bool) -> Self {
        match value {
            false => CBORCase::Simple(Simple::False).into(),
            true => CBORCase::Simple(Simple::True).into(),
        }
    }
}

impl CBORDecodable for bool {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor.case() {
            CBORCase::Simple(Simple::False) => Ok(false),
            CBORCase::Simple(Simple::True) => Ok(true),
            _ => bail!(CBORError::WrongType),
        }
    }
}

impl From<CBOR> for bool {
    fn from(value: CBOR) -> Self {
        Self::from_cbor(&value).unwrap()
    }
}
