import_stdlib!();

use anyhow::{bail, Error, Result};

use crate::{float::f64_cbor_data, CBORCase, CBORError, CBOR};

use super::varint::{EncodeVarInt, MajorType};

/// Represents CBOR simple values (major type 7).
///
/// In CBOR, simple values are a special category that includes booleans (`true` and `false`),
/// `null`, and floating point numbers. 
///
/// Per Section 2.4 of the dCBOR specification, only these specific simple values 
/// are valid in dCBOR. All other major type 7 values (such as undefined or other
/// simple values) are invalid and will be rejected by dCBOR decoders.
///
/// When encoding floating point values, dCBOR follows specific numeric reduction
/// rules detailed in Section 2.3 of the dCBOR specification, including:
/// - Integral floating point values must be reduced to integers when possible
/// - NaN values must be normalized to the canonical form `f97e00`
///
/// # Note
/// 
/// This type is primarily an implementation detail. Users should generally use
/// Rust's native types instead:
/// 
/// - Use Rust's `true` and `false` booleans directly
/// - Use the convenience methods `CBOR::r#true()`, `CBOR::r#false()`, and `CBOR::null()`
/// - Use Rust's floating point types like `f64` directly
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Use Rust's native boolean and numeric types
/// let false_cbor = CBOR::from(false);
/// let true_cbor = CBOR::from(true);
/// let float_cbor = CBOR::from(3.14159);
///
/// // Using convenience methods for common values
/// let false_value = CBOR::r#false();
/// let true_value = CBOR::r#true();
/// let null_value = CBOR::null();
///
/// // Verify they produce the same encodings
/// assert_eq!(false_cbor, false_value);
/// assert_eq!(true_cbor, true_value);
/// ```
#[derive(Clone)]
pub enum Simple {
    /// The boolean value `false`.
    /// Encoded as `0xf4` in CBOR, or `0x14` (20) with major type 7.
    False,
    
    /// The boolean value `true`.
    /// Encoded as `0xf5` in CBOR, or `0x15` (21) with major type 7.
    True,
    
    /// The value representing `null` (`None`).
    /// Encoded as `0xf6` in CBOR, or `0x16` (22) with major type 7.
    Null,
    
    /// A floating point value.
    /// 
    /// In dCBOR, floating point values follow these encoding rules:
    /// - Values are encoded in the shortest form that preserves precision
    /// - Integral floating point values are encoded as integers when in range
    /// - NaN values are normalized to `f97e00`
    Float(f64),
}

impl Simple {
    /// Returns the standard name of the simple value as a string.
    ///
    /// For `False`, `True`, and `Null`, this returns their lowercase string
    /// representation. For `Float` values, it returns their numeric representation.
    ///
    /// # Note
    ///
    /// This method is primarily used internally. Users should generally interact
    /// with Rust's native types rather than with `Simple` values directly.
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    /// Encodes the simple value to its raw CBOR byte representation.
    ///
    /// Returns the CBOR bytes that represent this simple value according to the 
    /// dCBOR deterministic encoding rules:
    /// - `False` encodes as `0xf4`
    /// - `True` encodes as `0xf5`
    /// - `Null` encodes as `0xf6`
    /// - `Float` values encode according to the IEEE 754 floating point rules,
    ///   using the shortest representation that preserves precision.
    ///
    /// # Note
    ///
    /// This method is primarily used internally. For encoding simple values,
    /// users should use the `to_cbor_data` method on CBOR values created from
    /// Rust's native types.
    pub fn cbor_data(&self) -> Vec<u8> {
        match self {
            Self::False => 20u8.encode_varint(MajorType::Simple),
            Self::True => 21u8.encode_varint(MajorType::Simple),
            Self::Null => 22u8.encode_varint(MajorType::Simple),
            Self::Float(v) => f64_cbor_data(*v),
        }
    }
}

/// Converts a `Simple` value into a CBOR representation.
///
/// This conversion allows `Simple` values to be seamlessly used where CBOR values
/// are expected, creating a CBOR value of type `CBORCase::Simple` that wraps the
/// simple value.
///
/// # Note
///
/// This conversion is primarily used internally. Users should generally prefer
/// converting from Rust's native types (bool, f64) directly to CBOR instead of
/// using the `Simple` type.
impl From<Simple> for CBOR {
    fn from(value: Simple) -> Self {
        CBORCase::Simple(value.clone()).into()
    }
}

/// Attempts to convert a CBOR value to a `Simple` value.
///
/// If the CBOR value is a `CBORCase::Simple`, this conversion will succeed.
/// For any other CBOR type, it will return a `WrongType` error.
///
/// # Note
///
/// This conversion is primarily used internally. Users should generally prefer
/// converting CBOR values to Rust's native types (bool, f64, etc.) instead of
/// to the `Simple` type.
impl TryFrom<CBOR> for Simple {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Simple(simple) => Ok(simple),
            _ => bail!(CBORError::WrongType),
        }
    }
}

/// Implements equality comparison for `Simple` values.
///
/// Two `Simple` values are equal if they're the same variant. For `Float` variants,
/// the contained floating point values are compared for equality according to
/// Rust's floating point equality rules.
impl PartialEq for Simple {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::False, Self::False) => true,
            (Self::True, Self::True) => true,
            (Self::Null, Self::Null) => true,
            (Self::Float(v1), Self::Float(v2)) => v1 == v2,
            _ => false,
        }
    }
}

/// Implements debug formatting for `Simple` values.
///
/// This is used to generate string representations for debugging purposes.
/// The format matches the standard string representations of these values:
/// - `false` for `Simple::False`
/// - `true` for `Simple::True`
/// - `null` for `Simple::Null`
/// - The debug representation of the float for `Simple::Float`
///
/// This implementation is used internally by the `name` method.
impl fmt::Debug for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{:?}", v),
        };
        f.write_str(&s)
    }
}

/// Implements string display formatting for `Simple` values.
///
/// This is used when converting a `Simple` value to a string, such as with
/// `to_string()`. The format matches the standard string representations:
/// - `false` for `Simple::False`
/// - `true` for `Simple::True`
/// - `null` for `Simple::Null`
/// - The debug representation of the float for `Simple::Float`
impl fmt::Display for Simple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::False => "false".to_owned(),
            Self::True => "true".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Float(v) => format!("{:?}", v),
        };
        f.write_str(&s)
    }
}
