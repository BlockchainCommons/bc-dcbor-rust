//! CBOR bignum (tags 2 and 3) support for `num-bigint` types.
//!
//! This module provides conversion between [`CBOR`] and the `num-bigint`
//! crate's [`BigInt`] and [`BigUint`] types, implementing RFC 8949 §3.4.3
//! (Bignums) with dCBOR/CDE canonical encoding rules.
//!
//! # Encoding
//!
//! - [`BigUint`] always encodes as tag 2 (positive bignum) with a byte string
//!   content.
//! - [`BigInt`] encodes as tag 2 for non-negative values or tag 3 (negative
//!   bignum) for negative values.
//! - No numeric reduction is performed: values are always encoded as bignums,
//!   even if they would fit in normal CBOR integers.
//!
//! # Decoding
//!
//! - Accepts CBOR integers (major types 0 and 1) and converts them to bignums.
//! - Accepts tag 2 (positive bignum) and tag 3 (negative bignum) with byte
//!   string content.
//! - Enforces shortest-form canonical representation for bignum magnitudes.
//! - Rejects floating-point values.
//!
//! # Examples
//!
//! ```
//! use dcbor::prelude::*;
//! use num_bigint::{BigInt, BigUint};
//!
//! // Encode a BigUint
//! let big = BigUint::from(256u32);
//! let cbor = CBOR::from(big.clone());
//! assert_eq!(cbor.diagnostic(), "2(h'0100')");
//!
//! // Decode back to BigUint
//! let decoded: BigUint = cbor.try_into().unwrap();
//! assert_eq!(decoded, big);
//!
//! // Encode a negative BigInt
//! let neg = BigInt::from(-1);
//! let cbor = CBOR::from(neg.clone());
//! assert_eq!(cbor.diagnostic(), "3(h'00')");
//!
//! // Decode back to BigInt
//! let decoded: BigInt = cbor.try_into().unwrap();
//! assert_eq!(decoded, neg);
//! ```

import_stdlib!();

pub use num_bigint::{BigInt, BigUint, Sign};

use crate::{
    CBOR, CBORCase, Error, Result, TAG_NEGATIVE_BIGNUM, TAG_POSITIVE_BIGNUM,
    Tag,
};

/// Validates that a bignum magnitude byte string is in shortest canonical form.
///
/// Rules:
/// - For positive bignums (tag 2): empty byte string represents zero; non-empty
///   must not have leading zero bytes.
/// - For negative bignums (tag 3): byte string must not be empty (magnitude
///   zero is encoded as `0x00`); must not have leading zero bytes except when
///   the magnitude is zero (single `0x00`).
fn validate_bignum_magnitude(bytes: &[u8], is_negative: bool) -> Result<()> {
    if is_negative {
        // Tag 3: byte string must not be empty
        if bytes.is_empty() {
            return Err(Error::NonCanonicalNumeric);
        }
        // No leading zeros unless the entire magnitude is zero (single 0x00
        // byte)
        if bytes.len() > 1 && bytes[0] == 0 {
            return Err(Error::NonCanonicalNumeric);
        }
    } else {
        // Tag 2: empty byte string is valid (represents zero)
        // Non-empty must not have leading zeros
        if !bytes.is_empty() && bytes[0] == 0 {
            return Err(Error::NonCanonicalNumeric);
        }
    }
    Ok(())
}

/// Strips leading zero bytes from a byte slice, returning the minimal
/// representation.
fn strip_leading_zeros(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    &bytes[start..]
}

impl From<BigUint> for CBOR {
    /// Converts a [`BigUint`] to CBOR as a tag 2 (positive bignum).
    ///
    /// The value is always encoded as a bignum regardless of size.
    /// Zero is encoded as tag 2 with an empty byte string.
    fn from(value: BigUint) -> Self {
        let bytes = value.to_bytes_be();
        let stripped = strip_leading_zeros(&bytes);
        let byte_string = CBOR::to_byte_string(stripped);
        CBOR::to_tagged_value(Tag::with_value(TAG_POSITIVE_BIGNUM), byte_string)
    }
}

impl From<&BigUint> for CBOR {
    fn from(value: &BigUint) -> Self { value.clone().into() }
}

impl From<BigInt> for CBOR {
    /// Converts a [`BigInt`] to CBOR as a tag 2 or tag 3 bignum.
    ///
    /// - Non-negative values use tag 2 (positive bignum).
    /// - Negative values use tag 3 (negative bignum), where the encoded
    ///   magnitude is `|value| - 1` per RFC 8949.
    fn from(value: BigInt) -> Self {
        let (sign, magnitude) = value.into_parts();
        match sign {
            Sign::NoSign | Sign::Plus => {
                // Zero or positive: use tag 2
                CBOR::from(magnitude)
            }
            Sign::Minus => {
                // Negative: use tag 3 with magnitude = |value| - 1
                // For value = -1, magnitude = 1, so n = 0 -> encode as 0x00
                // For value = -2, magnitude = 2, so n = 1 -> encode as 0x01
                let n = magnitude - 1u32;
                let bytes = n.to_bytes_be();
                let stripped = strip_leading_zeros(&bytes);
                // For n = 0 (value = -1), to_bytes_be() returns empty, but we
                // need 0x00
                let content = if stripped.is_empty() {
                    CBOR::to_byte_string([0u8])
                } else {
                    CBOR::to_byte_string(stripped)
                };
                CBOR::to_tagged_value(
                    Tag::with_value(TAG_NEGATIVE_BIGNUM),
                    content,
                )
            }
        }
    }
}

impl From<&BigInt> for CBOR {
    fn from(value: &BigInt) -> Self { value.clone().into() }
}

/// Decodes a [`BigUint`] from an untagged CBOR byte string.
///
/// This function is intended for use in tag summarizers where the tag has
/// already been stripped. It expects a CBOR byte string representing the
/// big-endian magnitude of a positive bignum (tag 2 content).
///
/// Enforces canonical encoding: no leading zero bytes (except empty for zero).
pub fn biguint_from_untagged_cbor(cbor: CBOR) -> Result<BigUint> {
    let bytes = cbor.try_into_byte_string()?;
    validate_bignum_magnitude(&bytes, false)?;
    Ok(BigUint::from_bytes_be(&bytes))
}

/// Decodes a [`BigInt`] from an untagged CBOR byte string for a negative
/// bignum.
///
/// This function is intended for use in tag summarizers where the tag has
/// already been stripped. It expects a CBOR byte string representing `n` where
/// the actual value is `-1 - n` (tag 3 content per RFC 8949).
///
/// Enforces canonical encoding: no leading zero bytes (except single `0x00`
/// for -1).
pub fn bigint_from_negative_untagged_cbor(cbor: CBOR) -> Result<BigInt> {
    let bytes = cbor.try_into_byte_string()?;
    validate_bignum_magnitude(&bytes, true)?;
    let n = BigUint::from_bytes_be(&bytes);
    let magnitude = n + 1u32;
    Ok(BigInt::from_biguint(Sign::Minus, magnitude))
}

impl TryFrom<CBOR> for BigUint {
    type Error = Error;

    /// Converts CBOR to a [`BigUint`].
    ///
    /// Accepts:
    /// - Major type 0 (unsigned integer)
    /// - Tag 2 (positive bignum) with canonical byte string
    ///
    /// Rejects:
    /// - Major type 1 (negative integer)
    /// - Tag 3 (negative bignum)
    /// - Floating-point values
    /// - Non-canonical bignum encodings
    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => Ok(BigUint::from(n)),
            CBORCase::Negative(_) => Err(Error::OutOfRange),
            CBORCase::Tagged(tag, inner) => {
                let tag_value = tag.value();
                if tag_value == TAG_POSITIVE_BIGNUM {
                    // Tag 2: positive bignum
                    let bytes = inner.try_into_byte_string()?;
                    validate_bignum_magnitude(&bytes, false)?;
                    Ok(BigUint::from_bytes_be(&bytes))
                } else if tag_value == TAG_NEGATIVE_BIGNUM {
                    // Tag 3: negative bignum - not allowed for BigUint
                    Err(Error::OutOfRange)
                } else {
                    Err(Error::WrongType)
                }
            }
            CBORCase::Simple(_) => {
                // This covers floats (Simple::Float) - reject them
                Err(Error::WrongType)
            }
            _ => Err(Error::WrongType),
        }
    }
}

impl TryFrom<CBOR> for BigInt {
    type Error = Error;

    /// Converts CBOR to a [`BigInt`].
    ///
    /// Accepts:
    /// - Major type 0 (unsigned integer)
    /// - Major type 1 (negative integer)
    /// - Tag 2 (positive bignum) with canonical byte string
    /// - Tag 3 (negative bignum) with canonical byte string
    ///
    /// Rejects:
    /// - Floating-point values
    /// - Non-canonical bignum encodings
    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Unsigned(n) => Ok(BigInt::from(n)),
            CBORCase::Negative(n) => {
                // CBOR negative: value = -1 - n
                // n is the raw u64 from the encoding
                let magnitude = BigUint::from(n) + 1u32;
                Ok(BigInt::from_biguint(Sign::Minus, magnitude))
            }
            CBORCase::Tagged(tag, inner) => {
                let tag_value = tag.value();
                if tag_value == TAG_POSITIVE_BIGNUM {
                    // Tag 2: positive bignum
                    let bytes = inner.try_into_byte_string()?;
                    validate_bignum_magnitude(&bytes, false)?;
                    let magnitude = BigUint::from_bytes_be(&bytes);
                    if magnitude == BigUint::ZERO {
                        Ok(BigInt::from(0))
                    } else {
                        Ok(BigInt::from_biguint(Sign::Plus, magnitude))
                    }
                } else if tag_value == TAG_NEGATIVE_BIGNUM {
                    // Tag 3: negative bignum, value = -1 - n
                    let bytes = inner.try_into_byte_string()?;
                    validate_bignum_magnitude(&bytes, true)?;
                    let n = BigUint::from_bytes_be(&bytes);
                    let magnitude = n + 1u32;
                    Ok(BigInt::from_biguint(Sign::Minus, magnitude))
                } else {
                    Err(Error::WrongType)
                }
            }
            CBORCase::Simple(_) => {
                // This covers floats (Simple::Float) - reject them
                Err(Error::WrongType)
            }
            _ => Err(Error::WrongType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biguint_zero() {
        let big = BigUint::from(0u32);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'')");
        let decoded: BigUint = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_biguint_one() {
        let big = BigUint::from(1u32);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'01')");
        let decoded: BigUint = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_biguint_255() {
        let big = BigUint::from(255u32);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'ff')");
        let decoded: BigUint = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_biguint_256() {
        let big = BigUint::from(256u32);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'0100')");
        let decoded: BigUint = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_zero() {
        let big = BigInt::from(0);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_positive() {
        let big = BigInt::from(256);
        let cbor = CBOR::from(big.clone());
        assert_eq!(cbor.diagnostic(), "2(h'0100')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_negative_one() {
        let big = BigInt::from(-1);
        let cbor = CBOR::from(big.clone());
        // -1 -> magnitude = 1, n = 0 -> 0x00
        assert_eq!(cbor.diagnostic(), "3(h'00')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_negative_two() {
        let big = BigInt::from(-2);
        let cbor = CBOR::from(big.clone());
        // -2 -> magnitude = 2, n = 1 -> 0x01
        assert_eq!(cbor.diagnostic(), "3(h'01')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_negative_256() {
        let big = BigInt::from(-256);
        let cbor = CBOR::from(big.clone());
        // -256 -> magnitude = 256, n = 255 -> 0xff
        assert_eq!(cbor.diagnostic(), "3(h'ff')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_bigint_negative_257() {
        let big = BigInt::from(-257);
        let cbor = CBOR::from(big.clone());
        // -257 -> magnitude = 257, n = 256 -> 0x0100
        assert_eq!(cbor.diagnostic(), "3(h'0100')");
        let decoded: BigInt = cbor.try_into().unwrap();
        assert_eq!(decoded, big);
    }

    #[test]
    fn test_decode_plain_unsigned_to_biguint() {
        let cbor = CBOR::from(12345u64);
        let big: BigUint = cbor.try_into().unwrap();
        assert_eq!(big, BigUint::from(12345u64));
    }

    #[test]
    fn test_decode_plain_unsigned_to_bigint() {
        let cbor = CBOR::from(12345u64);
        let big: BigInt = cbor.try_into().unwrap();
        assert_eq!(big, BigInt::from(12345));
    }

    #[test]
    fn test_decode_plain_negative_to_bigint() {
        let cbor = CBOR::from(-12345i64);
        let big: BigInt = cbor.try_into().unwrap();
        assert_eq!(big, BigInt::from(-12345));
    }

    #[test]
    fn test_decode_plain_negative_to_biguint_fails() {
        let cbor = CBOR::from(-1i64);
        let result: Result<BigUint> = cbor.try_into();
        assert!(matches!(result, Err(Error::OutOfRange)));
    }

    #[test]
    fn test_decode_tag3_to_biguint_fails() {
        let big = BigInt::from(-1);
        let cbor = CBOR::from(big);
        let result: Result<BigUint> = cbor.try_into();
        assert!(matches!(result, Err(Error::OutOfRange)));
    }
}
