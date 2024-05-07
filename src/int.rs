import_stdlib!();

use crate::{CBOR, CBORError};

use super::{CBORCase, varint::{EncodeVarInt, MajorType}};

use anyhow::{bail, Error, Result};

macro_rules! impl_cbor {
    ($type: ty) => {
        impl From64 for $type {
            fn cbor_data(&self) -> Vec<u8> {
                #[allow(unused_comparisons)]
                if *self < 0 {
                    let n = (-1 - (*self as i128)) as u64;
                    n.encode_varint(MajorType::Negative)
                } else {
                    let n = *self as u64;
                    n.encode_varint(MajorType::Unsigned)
                }
            }
        }

        impl From<$type> for CBOR {
            fn from(value: $type) -> Self {
                #[allow(unused_comparisons)]
                if value < 0 {
                    CBORCase::Negative((-1 - (value as i128)) as u64).into()
                } else {
                    CBORCase::Unsigned(value as u64).into()
                }
            }
        }

        impl TryFrom<CBOR> for $type {
            type Error = Error;

            fn try_from(cbor: CBOR) -> Result<Self> {
                match cbor.into_case() {
                    CBORCase::Unsigned(n) => Self::from_u64(n, <$type>::MAX as u64, |x| x as $type),
                    CBORCase::Negative(n) => {
                        let a = Self::from_u64(n, <$type>::MAX as u64, |x| x as $type)? as i128;
                        Ok((-1 - a) as $type)
                    }
                    _ => bail!(CBORError::WrongType),
                }
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

pub trait From64 {
    fn cbor_data(&self) -> Vec<u8>;

    fn from_u64<F>(n: u64, max: u64, f: F) -> Result<Self>
    where F: Fn(u64) -> Self, Self: Sized
    {
        if n > max {
            bail!(CBORError::OutOfRange)
        }
        Ok(f(n))
    }

    #[allow(dead_code)]
    fn from_i64<F>(n: i64, min: i64, max: i64, f: F) -> Result<Self>
    where F: Fn(i64) -> Self, Self: Sized
    {
        if n > max || n > min {
            bail!(CBORError::OutOfRange)
        }
        Ok(f(n))
    }
}
