//! Tests for CBOR bignum (tags 2 and 3) support with `num-bigint`.
#![cfg(feature = "num-bigint")]

use dcbor::prelude::*;
use num_bigint::{BigInt, BigUint};

// ============================================================================
// RFC 8949 Appendix A test vectors
// These are the exact test vectors from the RFC specification.
// ============================================================================

/// RFC 8949 Appendix A: 18446744073709551616 (2^64) = 0xc249010000000000000000
#[test]
fn rfc8949_test_vector_2_pow_64() {
    // 2^64 = 18446744073709551616
    let expected_hex = "c249010000000000000000";
    let expected_value = BigUint::from(u64::MAX) + 1u32;

    // Encoding test
    let cbor = CBOR::from(expected_value.clone());
    assert_eq!(hex::encode(cbor.to_cbor_data()), expected_hex);

    // Decoding test
    let decoded_cbor = CBOR::try_from_hex(expected_hex).unwrap();
    let decoded: BigUint = decoded_cbor.try_into().unwrap();
    assert_eq!(decoded, expected_value);
}

/// RFC 8949 Appendix A: -18446744073709551617 (-2^64 - 1) =
/// 0xc349010000000000000000
#[test]
fn rfc8949_test_vector_neg_2_pow_64_minus_1() {
    // -18446744073709551617 = -(2^64 + 1)
    let expected_hex = "c349010000000000000000";
    let big_2_64: BigInt = BigInt::from(1) << 64;
    let sum: BigInt = big_2_64 + 1;
    let expected_value = -sum;

    // Encoding test
    let cbor = CBOR::from(expected_value.clone());
    assert_eq!(hex::encode(cbor.to_cbor_data()), expected_hex);

    // Decoding test
    let decoded_cbor = CBOR::try_from_hex(expected_hex).unwrap();
    let decoded: BigInt = decoded_cbor.try_into().unwrap();
    assert_eq!(decoded, expected_value);
}

/// RFC 8949 §3.4.3 example: The number 18446744073709551616 (2^64)
/// "is represented as 0b110_00010 (major type 6, tag number 2), followed by
/// 0b010_01001 (major type 2, length 9), followed by 0x010000000000000000"
#[test]
fn rfc8949_example_bignum_encoding() {
    // Verify our encoding matches the RFC example byte-by-byte
    let value = BigUint::from(1u128) << 64;
    let cbor = CBOR::from(value);
    let bytes = cbor.to_cbor_data();

    // 0xc2 = 0b110_00010 = major type 6 (tag), tag number 2
    assert_eq!(bytes[0], 0xc2);
    // 0x49 = 0b010_01001 = major type 2 (byte string), length 9
    assert_eq!(bytes[1], 0x49);
    // 0x010000000000000000 = 9 bytes of content
    assert_eq!(
        &bytes[2..],
        &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
}

// ============================================================================
// Round-trip tests for BigUint
// ============================================================================

#[test]
fn biguint_roundtrip_zero() {
    let big = BigUint::from(0u32);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_one() {
    let big = BigUint::from(1u32);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'01')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_255() {
    let big = BigUint::from(255u32);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'ff')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_256() {
    let big = BigUint::from(256u32);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'0100')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_u64_max() {
    let big = BigUint::from(u64::MAX);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'ffffffffffffffff')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_over_u64() {
    // 2^64 = 18446744073709551616
    let big = BigUint::from(u64::MAX) + 1u32;
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic_flat(), "2(h'010000000000000000')");
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn biguint_roundtrip_large() {
    // 2^200
    let big: BigUint = BigUint::from(1u32) << 200;
    let cbor = CBOR::from(big.clone());
    let decoded: BigUint = cbor.clone().try_into().unwrap();
    assert_eq!(decoded, big);
    // Verify it's encoded as tag 2
    assert!(cbor.diagnostic_flat().starts_with("2(h'"));
}

// ============================================================================
// Round-trip tests for BigInt
// ============================================================================

#[test]
fn bigint_roundtrip_zero() {
    let big = BigInt::from(0);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_positive_one() {
    let big = BigInt::from(1);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'01')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_positive_256() {
    let big = BigInt::from(256);
    let cbor = CBOR::from(big.clone());
    assert_eq!(cbor.diagnostic(), "2(h'0100')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_negative_one() {
    let big = BigInt::from(-1);
    let cbor = CBOR::from(big.clone());
    // -1 -> magnitude = 1, n = 0 -> 0x00
    assert_eq!(cbor.diagnostic(), "3(h'00')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_negative_two() {
    let big = BigInt::from(-2);
    let cbor = CBOR::from(big.clone());
    // -2 -> magnitude = 2, n = 1 -> 0x01
    assert_eq!(cbor.diagnostic(), "3(h'01')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_negative_256() {
    let big = BigInt::from(-256);
    let cbor = CBOR::from(big.clone());
    // -256 -> magnitude = 256, n = 255 -> 0xff
    assert_eq!(cbor.diagnostic(), "3(h'ff')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_negative_257() {
    let big = BigInt::from(-257);
    let cbor = CBOR::from(big.clone());
    // -257 -> magnitude = 257, n = 256 -> 0x0100
    assert_eq!(cbor.diagnostic(), "3(h'0100')");
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, big);
}

#[test]
fn bigint_roundtrip_large_positive() {
    // 2^200
    let big: BigInt = BigInt::from(1) << 200;
    let cbor = CBOR::from(big.clone());
    let decoded: BigInt = cbor.clone().try_into().unwrap();
    assert_eq!(decoded, big);
    assert!(cbor.diagnostic_flat().starts_with("2(h'"));
}

#[test]
fn bigint_roundtrip_large_negative() {
    // -(2^200)
    let shifted: BigInt = BigInt::from(1) << 200;
    let big = -shifted;
    let cbor = CBOR::from(big.clone());
    let decoded: BigInt = cbor.clone().try_into().unwrap();
    assert_eq!(decoded, big);
    assert!(cbor.diagnostic_flat().starts_with("3(h'"));
}

// ============================================================================
// Decoding plain CBOR integers to bignums
// ============================================================================

#[test]
fn decode_plain_unsigned_to_biguint() {
    let cbor = CBOR::from(12345u64);
    let big: BigUint = cbor.try_into().unwrap();
    assert_eq!(big, BigUint::from(12345u64));
}

#[test]
fn decode_plain_unsigned_to_bigint() {
    let cbor = CBOR::from(12345u64);
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(12345));
}

#[test]
fn decode_plain_negative_to_bigint() {
    let cbor = CBOR::from(-12345i64);
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(-12345));
}

#[test]
fn decode_plain_max_u64_to_biguint() {
    let cbor = CBOR::from(u64::MAX);
    let big: BigUint = cbor.try_into().unwrap();
    assert_eq!(big, BigUint::from(u64::MAX));
}

#[test]
fn decode_plain_min_i64_to_bigint() {
    // i64::MIN = -9223372036854775808
    let cbor = CBOR::from(i64::MIN);
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(i64::MIN));
}

// ============================================================================
// Sign/range checks - negative values to BigUint must fail
// ============================================================================

#[test]
fn decode_plain_negative_to_biguint_fails() {
    let cbor = CBOR::from(-1i64);
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_tag3_to_biguint_fails() {
    let big = BigInt::from(-1);
    let cbor = CBOR::from(big);
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_large_negative_bigint_to_biguint_fails() {
    let shifted: BigInt = BigInt::from(1) << 200;
    let big = -shifted;
    let cbor = CBOR::from(big);
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

// ============================================================================
// Float rejection tests
// ============================================================================

#[test]
fn decode_float_to_biguint_fails() {
    // Use a non-integral float that dCBOR cannot reduce to an integer
    let cbor = CBOR::from(1.5f64);
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_float_to_bigint_fails() {
    // Use a non-integral float that dCBOR cannot reduce to an integer
    let cbor = CBOR::from(1.5f64);
    let result: Result<BigInt, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_integral_float_to_biguint_succeeds() {
    // dCBOR reduces integral floats to integers, so this should succeed
    // because 42.0 becomes the integer 42
    let cbor = CBOR::from(42.0f64);
    // Verify it was reduced to an integer
    assert_eq!(cbor.diagnostic(), "42");
    let result: BigUint = cbor.try_into().unwrap();
    assert_eq!(result, BigUint::from(42u32));
}

#[test]
fn decode_integral_float_to_bigint_succeeds() {
    // dCBOR reduces integral floats to integers, so this should succeed
    let cbor = CBOR::from(42.0f64);
    let result: BigInt = cbor.try_into().unwrap();
    assert_eq!(result, BigInt::from(42));
}

#[test]
fn decode_negative_float_to_bigint_fails() {
    let cbor = CBOR::from(-1.5f64);
    let result: Result<BigInt, _> = cbor.try_into();
    assert!(result.is_err());
}

// ============================================================================
// Wrong type rejection tests
// ============================================================================

#[test]
fn decode_string_to_biguint_fails() {
    let cbor = CBOR::from("hello");
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_string_to_bigint_fails() {
    let cbor = CBOR::from("hello");
    let result: Result<BigInt, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_array_to_biguint_fails() {
    let cbor: CBOR = vec![1u64, 2, 3].into();
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_bytestring_to_biguint_fails() {
    // A raw byte string (not tagged) should fail
    let cbor = CBOR::to_byte_string([1u8, 2, 3]);
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_wrong_tag_to_biguint_fails() {
    // Tag 42 with a byte string should fail
    let cbor = CBOR::to_tagged_value(42u64, CBOR::to_byte_string([1u8]));
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

// ============================================================================
// Non-canonical encoding rejection tests
// ============================================================================

#[test]
fn decode_tag2_with_leading_zero_fails() {
    // Tag 2 with h'0001' (leading zero) should fail
    // This represents 1, but with a non-canonical encoding
    let cbor = CBOR::to_tagged_value(2u64, CBOR::to_byte_string([0u8, 1u8]));
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_tag2_with_leading_zeros_fails() {
    // Tag 2 with h'000001' (multiple leading zeros) should fail
    let cbor =
        CBOR::to_tagged_value(2u64, CBOR::to_byte_string([0u8, 0u8, 1u8]));
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_tag3_with_leading_zero_fails() {
    // Tag 3 with h'0001' (leading zero) should fail
    // This represents -2 but with a non-canonical encoding
    let cbor = CBOR::to_tagged_value(3u64, CBOR::to_byte_string([0u8, 1u8]));
    let result: Result<BigInt, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_tag3_empty_byte_string_fails() {
    // Tag 3 with empty byte string should fail
    // (tag 2 with empty is valid for zero, but tag 3 needs at least 0x00)
    let cbor = CBOR::to_tagged_value(3u64, CBOR::to_byte_string([]));
    let result: Result<BigInt, _> = cbor.try_into();
    assert!(result.is_err());
}

#[test]
fn decode_tag2_empty_is_zero() {
    // Tag 2 with empty byte string is canonical zero
    let cbor = CBOR::to_tagged_value(2u64, CBOR::to_byte_string([]));
    let big: BigUint = cbor.clone().try_into().unwrap();
    assert_eq!(big, BigUint::from(0u32));
    let big_int: BigInt = cbor.try_into().unwrap();
    assert_eq!(big_int, BigInt::from(0));
}

#[test]
fn decode_tag3_single_zero_is_negative_one() {
    // Tag 3 with h'00' represents -1 (n=0, value = -1 - 0 = -1)
    let cbor = CBOR::to_tagged_value(3u64, CBOR::to_byte_string([0u8]));
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(-1));
}

// ============================================================================
// Reference conversion tests (From<&BigInt> and From<&BigUint>)
// ============================================================================

#[test]
fn from_ref_biguint() {
    let big = BigUint::from(256u32);
    let cbor = CBOR::from(&big);
    assert_eq!(cbor.diagnostic(), "2(h'0100')");
}

#[test]
fn from_ref_bigint() {
    let big = BigInt::from(-256);
    let cbor = CBOR::from(&big);
    assert_eq!(cbor.diagnostic(), "3(h'ff')");
}

// ============================================================================
// Edge cases around dCBOR integer range
// ============================================================================

#[test]
fn bigint_around_i64_bounds() {
    // Values at i64 boundaries still encode as bignums
    let i64_max = BigInt::from(i64::MAX);
    let cbor = CBOR::from(i64_max.clone());
    assert!(cbor.diagnostic().starts_with("2(h'"));
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, i64_max);

    let i64_min = BigInt::from(i64::MIN);
    let cbor = CBOR::from(i64_min.clone());
    assert!(cbor.diagnostic().starts_with("3(h'"));
    let decoded: BigInt = cbor.try_into().unwrap();
    assert_eq!(decoded, i64_min);
}

#[test]
fn biguint_around_u64_bounds() {
    let u64_max = BigUint::from(u64::MAX);
    let cbor = CBOR::from(u64_max.clone());
    assert!(cbor.diagnostic().starts_with("2(h'"));
    let decoded: BigUint = cbor.try_into().unwrap();
    assert_eq!(decoded, u64_max);
}

// ============================================================================
// Additional RFC 8949 compliance tests
// ============================================================================

/// Per RFC 8949 §3.4.3: "The preferred serialization of the byte string is to
/// leave out any leading zeroes (note that this means the preferred
/// serialization for n = 0 is the empty byte string)"
#[test]
fn rfc8949_preferred_serialization_zero() {
    let zero = BigUint::from(0u32);
    let cbor = CBOR::from(zero);
    // Should be tag 2 with empty byte string
    assert_eq!(cbor.diagnostic(), "2(h'')");
    // Hex should be c240 (tag 2, empty byte string)
    assert_eq!(hex::encode(cbor.to_cbor_data()), "c240");
}

/// Per RFC 8949 §3.4.3: Decoders MUST be able to decode bignums with leading
/// zeroes, but dCBOR requires canonical encoding so we reject them.
#[test]
fn rfc8949_noncanonical_leading_zeros_rejected() {
    // Tag 2 with h'00' should fail (non-canonical zero)
    let cbor = CBOR::try_from_hex("c24100").unwrap(); // tag 2, 1-byte bstr h'00'
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());

    // Tag 2 with h'0001' should fail (non-canonical 1)
    let cbor = CBOR::try_from_hex("c2420001").unwrap(); // tag 2, 2-byte bstr
    let result: Result<BigUint, _> = cbor.try_into();
    assert!(result.is_err());
}

/// Test decoding from raw CBOR hex strings
#[test]
fn decode_from_hex_positive_bignum() {
    // Tag 2 with h'0100' = 256
    let cbor = CBOR::try_from_hex("c2420100").unwrap();
    let big: BigUint = cbor.try_into().unwrap();
    assert_eq!(big, BigUint::from(256u32));
}

/// Test decoding from raw CBOR hex strings
#[test]
fn decode_from_hex_negative_bignum() {
    // Tag 3 with h'00' = -1 (n=0, value = -1 - 0 = -1)
    let cbor = CBOR::try_from_hex("c34100").unwrap();
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(-1));

    // Tag 3 with h'ff' = -256 (n=255, value = -1 - 255 = -256)
    let cbor = CBOR::try_from_hex("c341ff").unwrap();
    let big: BigInt = cbor.try_into().unwrap();
    assert_eq!(big, BigInt::from(-256));
}

/// Verify the encoded byte length matches expectations
#[test]
fn encoding_length_efficiency() {
    // Small values still get full bignum encoding (no numeric reduction)
    let one = BigUint::from(1u32);
    let cbor = CBOR::from(one);
    // c2 41 01 = tag 2, 1-byte bstr, 0x01
    assert_eq!(cbor.to_cbor_data().len(), 3);

    // 2^64 should be tag + 9-byte length prefix + 9 bytes = 11 bytes
    let big = BigUint::from(1u128) << 64;
    let cbor = CBOR::from(big);
    assert_eq!(cbor.to_cbor_data().len(), 11);
}

// ============================================================================
// Tag summarizer tests
// These tests verify that the registered tag summarizers produce correct
// diagnostic output for bignum values.
// ============================================================================

/// Test that positive bignum summarizer displays the decimal value
#[test]
fn summarizer_positive_bignum() {
    // Register the bignum tags in the global store
    dcbor::register_tags();

    let big = BigUint::from(256u32);
    let cbor = CBOR::from(big);

    // With summarizer using global tags, shows: bignum(256)
    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(256)");
}

/// Test that negative bignum summarizer displays the decimal value
#[test]
fn summarizer_negative_bignum() {
    dcbor::register_tags();

    let big = BigInt::from(-256);
    let cbor = CBOR::from(big);

    // With summarizer, shows: bignum(-256)
    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(-256)");
}

/// Test summarizer for zero (tag 2 with empty byte string)
#[test]
fn summarizer_zero() {
    dcbor::register_tags();

    let big = BigUint::from(0u32);
    let cbor = CBOR::from(big);

    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(0)");
}

/// Test summarizer for -1 (tag 3 with h'00')
#[test]
fn summarizer_negative_one() {
    dcbor::register_tags();

    let big = BigInt::from(-1);
    let cbor = CBOR::from(big);

    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(-1)");
}

/// Test summarizer for large positive value (2^64)
#[test]
fn summarizer_large_positive() {
    dcbor::register_tags();

    let big = BigUint::from(1u128) << 64;
    let cbor = CBOR::from(big);

    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(18446744073709551616)");
}

/// Test summarizer for large negative value (-(2^64 + 1))
#[test]
fn summarizer_large_negative() {
    dcbor::register_tags();

    let big_2_64: BigInt = BigInt::from(1) << 64;
    let sum: BigInt = big_2_64 + 1;
    let big = -sum;
    let cbor = CBOR::from(big);

    let opts = DiagFormatOpts::default()
        .summarize(true)
        .tags(TagsStoreOpt::Global);
    let diag = cbor.diagnostic_opt(&opts);
    assert_eq!(diag, "bignum(-18446744073709551617)");
}
