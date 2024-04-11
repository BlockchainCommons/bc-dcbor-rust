import_stdlib!();

use bytes::Bytes;

use crate::{CBOR, varint::{MajorType, EncodeVarInt}, CBORCase};

/// A type that can be encoded as CBOR.
pub trait CBOREncodable: Into<CBOR> {
    /// Returns the value in CBOR symbolic representation.
    fn cbor(&self) -> CBOR;
    /// Returns the value in CBOR binary representation.
    fn cbor_data(&self) -> Vec<u8> {
        self.cbor().cbor_data()
    }
}

impl CBOREncodable for CBOR {
    fn cbor(&self) -> CBOR {
        self.clone()
    }

    fn cbor_data(&self) -> Vec<u8> {
        match self.case() {
            CBORCase::Unsigned(x) => x.cbor_data(),
            CBORCase::Negative(x) => x.encode_varint(MajorType::Negative),
            CBORCase::ByteString(x) => {
                let mut buf = x.len().encode_varint(MajorType::Bytes);
                buf.extend(x);
                buf
            },
            CBORCase::Text(x) => x.cbor_data(),
            CBORCase::Array(x) => x.cbor_data(),
            CBORCase::Map(x) => x.cbor_data(),
            CBORCase::Tagged(tag, item) => {
                let mut buf = tag.value().encode_varint(MajorType::Tagged);
                buf.extend(item.cbor_data());
                buf
            },
            CBORCase::Simple(x) => x.cbor_data(),
        }
    }
}

impl<T> CBOREncodable for &T where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        (*self).cbor()
    }
}

impl<T> From<&T> for CBOR where T: CBOREncodable {
    fn from(value: &T) -> Self {
        value.cbor()
    }
}

impl CBOREncodable for Box<CBOR> {
    fn cbor(&self) -> CBOR {
        (**self).cbor()
    }
}

impl From<Box<CBOR>> for CBOR {
    fn from(value: Box<CBOR>) -> Self {
        value.cbor()
    }
}

impl CBOREncodable for Bytes {
    fn cbor(&self) -> CBOR {
        CBORCase::ByteString(self.clone()).into()
    }
}
