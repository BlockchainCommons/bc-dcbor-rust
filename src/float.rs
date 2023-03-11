use crate::{CBOREncodable, CBOR, Simple, varint::{EncodeVarInt, MajorType}, CBORDecodable, CBORCodable, DecodeError};
use half::f16;

static CBOR_NAN: [u8; 3] = [0xf9, 0x7e, 0x00];

impl CBOREncodable for f64 {
    fn cbor(&self) -> CBOR {
        let n = *self;
        if n < 0.0f64 {
            let i = n as i64;
            if i as f64 == n {
                return i.cbor();
            }
        }
        let i = n as u64;
        if i as f64 == n {
            return i.cbor();
        }
        CBOR::Simple(Simple::Float(n))
    }

    fn cbor_data(&self) -> Vec<u8> {
        let n = *self;
        let f = n as f32;
        if f as f64 == n {
            return f.cbor_data();
        }
        if n < 0.0f64 {
            let i = n as i64;
            if i as f64 == n {
                return i.cbor_data();
            }
        }
        let i = n as u64;
        if i as f64 == n {
            return i.cbor_data();
        }
        if self.is_nan() {
            return CBOR_NAN.to_vec();
        }
        n.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f64 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                let f = *n as f64;
                if f as u64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Negative(n) => {
                let f = *n as f64;
                if f as i64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Simple(Simple::Float(n)) => Ok(Box::new(*n)),
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f64 {}

pub(crate) fn validate_canonical_f64(n: f64) -> Result<(), DecodeError> {
    if
        n == n as f32 as f64 ||
        n == n as i64 as f64 ||
        n.is_nan()
    {
        return Err(DecodeError::NonCanonicalNumeric);
    }
    Ok(())
}

impl From<&f64> for CBOR {
    fn from(value: &f64) -> Self {
        value.cbor()
    }
}

impl From<f64> for CBOR {
    fn from(value: f64) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for f64 {
    fn from(value: CBOR) -> Self {
        *Self::from_cbor(&value).unwrap()
    }
}

impl TryFrom<&CBOR> for f64 {
    type Error = DecodeError;

    fn try_from(cbor: &CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Unsigned(n) => Ok(*n as f64),
            CBOR::Negative(n) => Ok(*n as f64),
            CBOR::Simple(Simple::Float(n)) => Ok(*n),
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl CBOREncodable for f32 {
    fn cbor(&self) -> CBOR {
        let n = *self;
        if n < 0.0f32 {
            let i = n as i32;
            if i as f32 == n {
                return i.cbor();
            }
        }
        let i = n as u32;
        if i as f32 == n {
            return i.cbor();
        }
        CBOR::Simple(Simple::Float(n as f64))
    }

    fn cbor_data(&self) -> Vec<u8> {
        let n = *self;
        let f = f16::from_f32(*self);
        if f.to_f32() == n {
            return f.cbor_data();
        }
        if n < 0.0f32 {
            let i = n as i32;
            if i as f32 == n {
                return i.cbor_data();
            }
        }
        let i = n as u32;
        if i as f32 == n {
            return i.cbor_data();
        }
        if self.is_nan() {
            return CBOR_NAN.to_vec();
        }
        n.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f32 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                let f = *n as f32;
                if f as u64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Negative(n) => {
                let f = *n as f32;
                if f as i64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Simple(Simple::Float(n)) => {
                let f = *n as f32;
                if f as f64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f32 {}

pub(crate) fn validate_canonical_f32(n: f32) -> Result<(), DecodeError> {
    if
        n == f16::from_f32(n).to_f32() ||
        n == n as i32 as f32 ||
        n.is_nan()
    {
        return Err(DecodeError::NonCanonicalNumeric);
    }
    Ok(())
}

impl From<&f32> for CBOR {
    fn from(value: &f32) -> Self {
        value.cbor()
    }
}

impl From<f32> for CBOR {
    fn from(value: f32) -> Self {
        value.cbor()
    }
}

impl From<CBOR> for f32 {
    fn from(value: CBOR) -> Self {
        *Self::from_cbor(&value).unwrap()
    }
}

impl TryFrom<&CBOR> for f32 {
    type Error = DecodeError;

    fn try_from(cbor: &CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Unsigned(n) => Ok(*n as f32),
            CBOR::Negative(n) => Ok(*n as f32),
            CBOR::Simple(Simple::Float(n)) => Ok(*n as f32),
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl CBOREncodable for f16 {
    fn cbor(&self) -> CBOR {
        let n = self.to_f64();
        if n < 0.0 {
            let i = n as i16;
            if i as f64 == n {
                return i.cbor();
            }
        }
        let i = n as u16;
        if i as f64 == n {
            return i.cbor();
        }
        CBOR::Simple(Simple::Float(n))
    }

    fn cbor_data(&self) -> Vec<u8> {
        let n = self.to_f64();
        if n < 0.0 {
            let i = n as i16;
            if i as f64 == n {
                return i.cbor_data();
            }
        }
        let i = n as u16;
        if i as f64 == n {
            return i.cbor_data();
        }
        if self.is_nan() {
            return CBOR_NAN.to_vec();
        }
        self.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f16 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                let f = f16::from_f64(*n as f64);
                if f.to_f64() as u64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Negative(n) => {
                let f = f16::from_f64(*n as f64);
                if f.to_f64() as i64 == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            CBOR::Simple(Simple::Float(n)) => {
                let f = f16::from_f64(*n);
                if f.to_f64() == *n {
                    Ok(Box::new(f))
                } else {
                    Err(DecodeError::OutOfRange)
                }
            },
            _ => Err(DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f16 {}

pub(crate) fn validate_canonical_f16(n: f16) -> Result<(), DecodeError> {
    let f = n.to_f64();
    if
        f == f as i64 as f64 ||
        n.is_nan() && n.to_bits() != 0x7e00
    {
        return Err(DecodeError::NonCanonicalNumeric);
    }
    Ok(())
}
