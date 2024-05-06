use crate::CBOR;

pub trait CBOREncodable: Into<CBOR> {
    fn into_cbor(self) -> CBOR {
        self.into()
    }

    fn into_cbor_data(self) -> Vec<u8> {
        self.into().cbor_data()
    }
}

impl<T> CBOREncodable for T where T: Into<CBOR> { }

pub trait CBORDecodable: TryFrom<CBOR> { }

impl<T> CBORDecodable for T where T: TryFrom<CBOR> { }

/// A type that can be encoded to or decoded from CBOR.
pub trait CBORCodable { }

impl<T> CBORCodable for T where T: CBORDecodable + CBOREncodable { }
