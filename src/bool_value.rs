use crate::{CBOREncodable, CBOR, Simple};

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
