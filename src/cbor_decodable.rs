use std::rc::Rc;

use crate::{CBOR, error::Error};

/// A type that can be decoded from CBOR.
pub trait CBORDecodable: 'static {
    /// Creates an instance of this type from CBOR symbolic representation.
    fn from_cbor(cbor: &CBOR) -> Result<Rc<Self>, Error>;

    /// Creates an instance of this type from encoded CBOR binary data.
    fn from_cbor_data(cbor_data: &[u8]) -> Result<Rc<Self>, Error> {
        Self::from_cbor(&CBOR::from_data(cbor_data)?)
    }
}
