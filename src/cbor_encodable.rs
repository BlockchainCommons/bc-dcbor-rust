use crate::{CBOR, varint::{MajorType, EncodeVarInt}};

/// A type that can be encoded as CBOR.
pub trait CBOREncodable {
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
        match self {
            CBOR::Unsigned(x) => x.cbor_data(),
            CBOR::Negative(x) => {
                assert!(x < &0);
                x.cbor_data()
            },
            CBOR::Bytes(x) => x.cbor_data(),
            CBOR::Text(x) => x.cbor_data(),
            CBOR::Array(x) => x.cbor_data(),
            CBOR::Map(x) => x.cbor_data(),
            CBOR::Tagged(tag, item) => {
                let mut buf = tag.value().encode_varint(MajorType::Tagged);
                buf.extend(item.cbor_data());
                buf
            },
            CBOR::Simple(x) => x.cbor_data(),
        }
    }
}

impl<T> CBOREncodable for &T where T: CBOREncodable {
    fn cbor(&self) -> CBOR {
        (*self).cbor()
    }
}

// impl<T> CBOREncodable for Rc<T> where T: CBOREncodable {
//     fn cbor(&self) -> CBOR {
//         (**self).cbor()
//     }
// }
