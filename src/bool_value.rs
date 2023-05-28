use std::rc::Rc;

use crate::{CBOREncodable, CBOR, Simple, CBORDecodable, CBORCodable, Error};

impl CBOREncodable for bool {
    fn cbor(&self) -> CBOR {
        match self {
            false => CBOR::Simple(Simple::False),
            true => CBOR::Simple(Simple::True),
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
    fn from_cbor(cbor: &CBOR) -> Result<Rc<Self>, Error> {
        match cbor {
            CBOR::Simple(Simple::False) => Ok(Rc::new(false)),
            CBOR::Simple(Simple::True) => Ok(Rc::new(true)),
            _ => Err(Error::WrongType),
        }
    }
}

impl CBORCodable for bool { }

impl From<bool> for CBOR {
    fn from(value: bool) -> Self {
        value.cbor()
    }
}

impl From<&bool> for CBOR {
    fn from(value: &bool) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for bool {
    fn from(value: CBOR) -> Self {
        *Self::from_cbor(&value).unwrap()
    }
}

impl TryFrom<&CBOR> for bool {
    type Error = Error;

    fn try_from(value: &CBOR) -> Result<Self, Self::Error> {
        Self::from_cbor(value).map(|x| *x)
    }
}
