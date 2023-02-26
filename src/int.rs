use crate::{cbor_encodable::CBOREncodable, CBORDecodable, decode_error::DecodeError, CBORCodable};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}};

macro_rules! impl_cbor {
    ($type: ty) => {
        impl From64 for $type { }

        impl CBOREncodable for $type {
            fn cbor(&self) -> CBOR {
                #[allow(unused_comparisons)]
                if *self < 0 {
                    CBOR::Negative(*self as i64)
                } else {
                    CBOR::Unsigned(*self as u64)
                }
            }

            fn cbor_data(&self) -> Vec<u8> {
                #[allow(unused_comparisons)]
                if *self < 0 {
                    let n = (!*self as u64);
                    n.encode_varint(MajorType::Negative)
                } else {
                    let n = *self as u64;
                    n.encode_varint(MajorType::Unsigned)
                }
            }
        }

        impl CBORDecodable for $type {
            fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::decode_error::DecodeError> {
                match cbor {
                    CBOR::Unsigned(n) => Self::from_u64(*n, <$type>::MAX as u64, |x| x as $type),
                    CBOR::Negative(n) => Self::from_i64(*n, 0, <$type>::MAX as i64, |x| x as $type),
                    _ => Err(DecodeError::WrongType),
                }
            }
        }

        impl CBORCodable for $type { }

        impl From<$type> for CBOR {
            fn from(value: $type) -> Self {
                value.cbor()
            }
        }

        impl From<&$type> for CBOR {
            fn from(value: &$type) -> Self {
                value.cbor()
            }
        }

        impl From<CBOR> for $type {
            fn from(value: CBOR) -> Self {
                *Self::from_cbor(&value).unwrap()
            }
        }

        impl TryFrom<&CBOR> for $type {
            type Error = crate::decode_error::DecodeError;

            fn try_from(value: &CBOR) -> Result<Self, Self::Error> {
                Self::from_cbor(value).map(|x| *x)
            }
        }
    };
}

impl_cbor!(u8);
impl_cbor!(u16);
impl_cbor!(u32);
impl_cbor!(u64);
impl_cbor!(usize);
impl_cbor!(i8);
impl_cbor!(i16);
impl_cbor!(i32);
impl_cbor!(i64);

trait From64 {
    fn from_u64<F>(n: u64, max: u64, f: F) -> Result<Box<Self>, DecodeError> where F: Fn(u64) -> Self, Self: Sized {
        if n > max {
            Err(DecodeError::IntegerOutOfRange)
        } else {
            Ok(Box::new(f(n)))
        }
    }

    fn from_i64<F>(n: i64, min: i64, max: i64, f: F) -> Result<Box<Self>, DecodeError> where F: Fn(i64) -> Self, Self: Sized {
        if n > max || n > min {
            Err(DecodeError::IntegerOutOfRange)
        } else {
            Ok(Box::new(f(n)))
        }
    }
}
