import_stdlib!();

use half::f16;
use anyhow::bail;

use crate::{int::From64, CBORCase, CBORError, ExactFrom, Simple, CBOR};

use super::varint::{EncodeVarInt, MajorType};

static CBOR_NAN: [u8; 3] = [0xf9, 0x7e, 0x00];

impl From<f64> for CBOR {
    fn from(value: f64) -> Self {
        let n = value;
        if n < 0.0f64 {
            if let Some(n) = i128::exact_from_f64(n) {
                if let Some(i) = u64::exact_from_i128(-1 - n) {
                    return CBORCase::Negative(i).into();
                }
            }
        }
        if let Some(i) = u64::exact_from_f64(n) {
            return i.into();
        }
        CBORCase::Simple(Simple::Float(n)).into()
    }
}

pub fn f64_cbor_data(value: f64) -> Vec<u8> {
    let n = value;
    let f = n as f32;
    if f as f64 == n {
        return f32_cbor_data(f);
    }
    if n < 0.0f64 {
        if let Some(n) = i128::exact_from_f64(n) {
            if let Some(i) = u64::exact_from_i128(-1 - n) {
                let cbor: CBOR = CBORCase::Negative(i).into();
                return cbor.cbor_data();
            }
        }
    }
    if let Some(i) = u64::exact_from_f64(n) {
        return i.cbor_data();
    }
    if value.is_nan() {
        return CBOR_NAN.to_vec();
    }
    n.to_bits().encode_int(MajorType::Simple)
}

pub(crate) fn validate_canonical_f64(n: f64) -> Result<(), CBORError> {
    if
        n == n as f32 as f64 ||
        n == n as i64 as f64 ||
        n.is_nan()
    {
        return Err(CBORError::NonCanonicalNumeric);
    }
    Ok(())
}

impl TryFrom<CBOR> for f64 {
    type Error = anyhow::Error;
    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Negative(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    Ok(-1f64 - f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Simple(Simple::Float(n)) => Ok(n),
            _ => bail!(CBORError::WrongType)
        }
    }
}

impl From<f32> for CBOR {
    fn from(value: f32) -> Self {
        let n = value;
        if n < 0.0f32 {
            if let Some(i) = u64::exact_from_f32(-1f32 - n) {
                return CBORCase::Negative(i).into();
            }
        }
        if let Some(i) = u32::exact_from_f32(n) {
            return i.into();
        }
        CBORCase::Simple(Simple::Float(n as f64)).into()
    }
}

pub fn f32_cbor_data(value: f32) -> Vec<u8> {
    let n = value;
    let f = f16::from_f32(n);
    if f.to_f32() == n {
        return f16_cbor_data(f);
    }
    if n < 0.0f32 {
        if let Some(i) = u64::exact_from_f32(-1f32 - n) {
            let cbor: CBOR = CBORCase::Negative(i).into();
            return cbor.cbor_data();
        }
    }
    if let Some(i) = u32::exact_from_f32(n) {
        return i.cbor_data();
    }
    if value.is_nan() {
        return CBOR_NAN.to_vec();
    }
    n.to_bits().encode_int(MajorType::Simple)
}

pub(crate) fn validate_canonical_f32(n: f32) -> Result<(), CBORError> {
    if
        n == f16::from_f32(n).to_f32() ||
        n == n as i32 as f32 ||
        n.is_nan()
    {
        return Err(CBORError::NonCanonicalNumeric);
    }
    Ok(())
}

impl TryFrom<CBOR> for f32 {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f32::exact_from_u64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Negative(n) => {
                if let Some(f) = f32::exact_from_u64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Simple(Simple::Float(n)) => {
                if let Some(f) = f32::exact_from_f64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            _ => bail!(CBORError::WrongType)
        }
    }
}

impl From<f16> for CBOR {
    fn from(value: f16) -> Self {
        let n = value.to_f64();
        if n < 0.0 {
            if let Some(i) = u64::exact_from_f64(-1f64 - n) {
                return CBORCase::Negative(i).into();
            }
        }
        if let Some(i) = u16::exact_from_f64(n) {
            return i.into();
        }
        CBORCase::Simple(Simple::Float(n)).into()
    }
}

pub fn f16_cbor_data(value: f16) -> Vec<u8> {
    let n = value.to_f64();
    if n < 0.0 {
        if let Some(i) = u64::exact_from_f64(-1f64 - n) {
            let cbor: CBOR = CBORCase::Negative(i).into();
            return cbor.cbor_data();
        }
    }
    if let Some(i) = u16::exact_from_f64(n) {
        return i.cbor_data();
    }
    if value.is_nan() {
        return CBOR_NAN.to_vec();
    }
    value.to_bits().encode_int(MajorType::Simple)
}

impl TryFrom<CBOR> for f16 {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f16::exact_from_u64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Negative(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    if let Some(b) = f16::exact_from_f64(-1f64 - f) {
                        Ok(b)
                    } else {
                        bail!(CBORError::OutOfRange);
                    }
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            CBORCase::Simple(Simple::Float(n)) => {
                if let Some(f) = f16::exact_from_f64(n) {
                    Ok(f)
                } else {
                    bail!(CBORError::OutOfRange);
                }
            },
            _ => bail!(CBORError::WrongType)
        }
    }
}

pub(crate) fn validate_canonical_f16(n: f16) -> Result<(), CBORError> {
    let f = n.to_f64();
    if
        f == f as i64 as f64 ||
        n.is_nan() && n.to_bits() != 0x7e00
    {
        return Err(CBORError::NonCanonicalNumeric);
    }
    Ok(())
}
