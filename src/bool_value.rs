import_stdlib!();

use crate::{CBOR, Simple, CBORError, CBORCase};

use anyhow::bail;

impl From<bool> for CBOR {
    fn from(value: bool) -> Self {
        match value {
            false => CBORCase::Simple(Simple::False).into(),
            true => CBORCase::Simple(Simple::True).into(),
        }
    }
}

impl TryFrom<CBOR> for bool {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.case() {
            CBORCase::Simple(Simple::False) => Ok(false),
            CBORCase::Simple(Simple::True) => Ok(true),
            _ => bail!(CBORError::WrongType),
        }
    }
}
