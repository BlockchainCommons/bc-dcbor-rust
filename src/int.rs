use crate::cbor_encodable::CBOREncodable;

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

impl CBOREncodable for u8 {
    fn cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.encode_varint(MajorType::Unsigned)
    }
}

impl CBOREncodable for u16 {
    fn cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.encode_varint(MajorType::Unsigned)
    }
}

impl CBOREncodable for u32 {
    fn cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.encode_varint(MajorType::Unsigned)
    }
}

impl CBOREncodable for u64 {
    fn cbor(&self) -> CBOR {
        CBOR::UInt(*self)
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.encode_varint(MajorType::Unsigned)
    }
}

impl CBOREncodable for usize {
    fn cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn cbor_data(&self) -> Vec<u8> {
        self.encode_varint(MajorType::Unsigned)
    }
}

impl CBOREncodable for i8 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i16;
            let a = (-b - 1) as u8;
            a.encode_varint(MajorType::Negative)
        } else {
            let a = *self as u8;
            a.encode_varint(MajorType::Unsigned)
        }
    }
}

impl CBOREncodable for i16 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i32;
            let a = (-b - 1) as u16;
            a.encode_varint(MajorType::Negative)
        } else {
            let a = *self as u16;
            a.encode_varint(MajorType::Unsigned)
        }
    }
}

impl CBOREncodable for i32 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i64;
            let a = (-b - 1) as u32;
            a.encode_varint(MajorType::Negative)
        } else {
            let a = *self as u32;
            a.encode_varint(MajorType::Unsigned)
        }
    }
}

impl CBOREncodable for i64 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn cbor_data(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as u64;
            let a = (-(b as i128) - 1) as u64;
            a.encode_varint(MajorType::Negative)
        } else {
            let a = *self as u64;
            a.encode_varint(MajorType::Unsigned)
        }
    }
}
