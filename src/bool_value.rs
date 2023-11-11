use crate::{CBOREncodable, CBOR, Simple, CBORDecodable, CBORCodable, CBORError, CBORCase};

use anyhow::bail;

impl CBOREncodable for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBORCase::Simple(Simple::False).into(),
            true => CBORCase::Simple(Simple::True).into(),
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self {
            false => Simple::False.cbor_data(),
            true => Simple::True.cbor_data()
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

impl CBORCodable for bool { }

impl From<bool> for CBOR {
    fn from(value: bool) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for bool {
    fn from(value: CBOR) -> Self {
        Self::from_cbor(&value).unwrap()
    }
}
