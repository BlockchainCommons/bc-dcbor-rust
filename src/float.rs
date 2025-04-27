import_stdlib!();

use half::f16;

use crate::{ int::From64, CBORCase, Error, Result, ExactFrom, Simple, CBOR };

use super::varint::{ EncodeVarInt, MajorType };

/// # Floating Point Number Support in dCBOR
///
/// dCBOR provides canonical encoding for floating point values through implementation of the
/// `From<T>` and `TryFrom<CBOR>` traits for `f16`, `f32`, and `f64` types.
///
/// Per the dCBOR specification, the canonical encoding rules ensure deterministic representation:
///
/// - Numeric reduction: Floating point values with zero fractional part in range [-2^63, 2^64-1]
///   are automatically encoded as integers (e.g., 42.0 becomes 42)
/// - Values are encoded in the smallest possible representation that preserves their value
/// - All NaN values are canonicalized to a single representation: 0xf97e00
/// - Positive/negative infinity are canonicalized to half-precision representations
///
/// ## Example
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create CBOR from floating point values using `into()`
/// let cbor_integer: CBOR = 42.0.into();  // Numeric reduction: encoded as integer 42
/// let cbor_float: CBOR = 3.14159.into();  // Encoded as floating point
/// let cbor_nan: CBOR = f64::NAN.into();  // Canonicalized to 0xf97e00
///
/// // Convert back to floating point
/// let value_integer: f64 = cbor_integer.try_into().unwrap();
/// assert_eq!(value_integer, 42.0);
///
/// // Maps can use numeric keys with automatic reduction
/// let mut map = Map::new();
/// map.insert(1.0, "integer key");    // 1.0 becomes 1
/// map.insert(2, "another key");      // Integer directly
///
/// // Verify numeric reduction in maps
/// let key_value: String = map.extract::<i32, String>(1).unwrap();
/// assert_eq!(key_value, "integer key");
/// ```
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

pub(crate) fn f64_cbor_data(value: f64) -> Vec<u8> {
    let n = value;
    let f = n as f32;
    if (f as f64) == n {
        return f32_cbor_data(f);
    }
    if n < 0.0f64 {
        if let Some(n) = i128::exact_from_f64(n) {
            if let Some(i) = u64::exact_from_i128(-1 - n) {
                let cbor: CBOR = CBORCase::Negative(i).into();
                return cbor.to_cbor_data();
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

pub(crate) fn validate_canonical_f64(n: f64) -> Result<()> {
    if n == (n as f32 as f64) || n == (n as i64 as f64) || n.is_nan() {
        return Err(Error::NonCanonicalNumeric);
    }
    Ok(())
}

impl TryFrom<CBOR> for f64 {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Negative(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    Ok(-1f64 - f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Simple(Simple::Float(n)) => Ok(n),
            _ => {
                return Err(Error::WrongType);
            }
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

pub(crate) fn f32_cbor_data(value: f32) -> Vec<u8> {
    let n = value;
    let f = f16::from_f32(n);
    if f.to_f32() == n {
        return f16_cbor_data(f);
    }
    if n < 0.0f32 {
        if let Some(i) = u64::exact_from_f32(-1f32 - n) {
            let cbor: CBOR = CBORCase::Negative(i).into();
            return cbor.to_cbor_data();
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

pub(crate) fn validate_canonical_f32(n: f32) -> Result<()> {
    if n == f16::from_f32(n).to_f32() || n == (n as i32 as f32) || n.is_nan() {
        return Err(Error::NonCanonicalNumeric);
    }
    Ok(())
}

impl TryFrom<CBOR> for f32 {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f32::exact_from_u64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Negative(n) => {
                if let Some(f) = f32::exact_from_u64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Simple(Simple::Float(n)) => {
                if let Some(f) = f32::exact_from_f64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            _ => {
                return Err(Error::WrongType);
            }
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

pub(crate) fn f16_cbor_data(value: f16) -> Vec<u8> {
    let n = value.to_f64();
    if n < 0.0 {
        if let Some(i) = u64::exact_from_f64(-1f64 - n) {
            let cbor: CBOR = CBORCase::Negative(i).into();
            return cbor.to_cbor_data();
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
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => {
                if let Some(f) = f16::exact_from_u64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Negative(n) => {
                if let Some(f) = f64::exact_from_u64(n) {
                    if let Some(b) = f16::exact_from_f64(-1f64 - f) {
                        Ok(b)
                    } else {
                        return Err(Error::OutOfRange);
                    }
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            CBORCase::Simple(Simple::Float(n)) => {
                if let Some(f) = f16::exact_from_f64(n) {
                    Ok(f)
                } else {
                    return Err(Error::OutOfRange);
                }
            }
            _ => {
                return Err(Error::WrongType);
            }
        }
    }
}

pub(crate) fn validate_canonical_f16(n: f16) -> Result<()> {
    let f = n.to_f64();
    if f == (f as i64 as f64) || (n.is_nan() && n.to_bits() != 0x7e00) {
        return Err(Error::NonCanonicalNumeric);
    }
    Ok(())
}
