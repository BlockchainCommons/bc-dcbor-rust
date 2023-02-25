use crate::{CBOREncodable, CBOR, Simple, varint::{EncodeVarInt, MajorType}, CBORDecodable, CBORCodable};
use half::f16;

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
        n.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f64 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                Ok(Box::new(*n as f64))
            },
            CBOR::Negative(n) => {
                Ok(Box::new(*n as f64))
            },
            CBOR::Simple(simple) => {
                match simple {
                    Simple::Float(n) => Ok(Box::new(*n)),
                    _ => Err(crate::DecodeError::WrongType)
                }
            },
            _ => Err(crate::DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f64 {}

pub(crate) fn validate_canonical_f64(n: f64) -> Result<(), crate::DecodeError> {
    if n == n as f32 as f64 {
        return Err(crate::DecodeError::NonCanonicalFloat);
    }
    if n == n as i64 as f64 {
        return Err(crate::DecodeError::NonCanonicalFloat);
    }
    Ok(())
}

impl TryFrom<&CBOR> for f64 {
    type Error = crate::DecodeError;

    fn try_from(cbor: &CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Unsigned(n) => {
                Ok(*n as f64)
            },
            CBOR::Negative(n) => {
                Ok(*n as f64)
            },
            CBOR::Simple(simple) => {
                match simple {
                    Simple::Float(n) => Ok(*n),
                    _ => Err(crate::DecodeError::WrongType)
                }
            },
            _ => Err(crate::DecodeError::WrongType)
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
        n.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f32 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                Ok(Box::new(*n as f32))
            },
            CBOR::Negative(n) => {
                Ok(Box::new(*n as f32))
            },
            CBOR::Simple(simple) => {
                match simple {
                    Simple::Float(n) => Ok(Box::new(*n as f32)),
                    _ => Err(crate::DecodeError::WrongType)
                }
            },
            _ => Err(crate::DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f32 {}

pub(crate) fn validate_canonical_f32(n: f32) -> Result<(), crate::DecodeError> {
    if n == f16::from_f32(n).to_f32() {
        return Err(crate::DecodeError::NonCanonicalFloat);
    }
    if n == n as i32 as f32 {
        return Err(crate::DecodeError::NonCanonicalFloat);
    }
    Ok(())
}

impl TryFrom<&CBOR> for f32 {
    type Error = crate::DecodeError;

    fn try_from(cbor: &CBOR) -> Result<Self, Self::Error> {
        match cbor {
            CBOR::Unsigned(n) => {
                Ok(*n as f32)
            },
            CBOR::Negative(n) => {
                Ok(*n as f32)
            },
            CBOR::Simple(simple) => {
                match simple {
                    Simple::Float(n) => Ok(*n as f32),
                    _ => Err(crate::DecodeError::WrongType)
                }
            },
            _ => Err(crate::DecodeError::WrongType)
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
        self.to_bits().encode_varint(MajorType::Simple)
    }
}

impl CBORDecodable for f16 {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::DecodeError> {
        match cbor {
            CBOR::Unsigned(n) => {
                Ok(Box::new(f16::from_f64(*n as f64)))
            },
            CBOR::Negative(n) => {
                Ok(Box::new(f16::from_f64(*n as f64)))
            },
            CBOR::Simple(simple) => {
                match simple {
                    Simple::Float(n) => Ok(Box::new(f16::from_f64(*n))),
                    _ => Err(crate::DecodeError::WrongType)
                }
            },
            _ => Err(crate::DecodeError::WrongType)
        }
    }
}

impl CBORCodable for f16 {}

pub(crate) fn validate_canonical_f16(n: f16) -> Result<(), crate::DecodeError> {
    let f = n.to_f64();
    if f == f as i64 as f64 {
        return Err(crate::DecodeError::NonCanonicalFloat);
    }
    Ok(())
}
