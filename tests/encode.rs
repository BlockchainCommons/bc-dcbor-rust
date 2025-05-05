#[cfg(feature = "std")]
mod with_std {
    pub use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
    pub use std::fmt::{self};
}

#[cfg(feature = "no_std")]
pub mod without_std {
    extern crate alloc;
    pub use alloc::{
        fmt::{self},
        collections::{BTreeMap, VecDeque},
    };
    pub use hashbrown::{HashMap, HashSet};
}

#[cfg(feature = "std")]
use with_std::*;

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
use without_std::*;

use dcbor::prelude::*;
use half::f16;
use hex_literal::hex;

fn test_cbor(t: impl Into<CBOR>, expected_debug: &str, expected_display: &str, expected_data: &str) {
    let cbor = t.into();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.to_cbor_data();
    assert_eq!(hex::encode(&data), expected_data);
    let decoded_cbor = CBOR::try_from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}

fn test_cbor_decode(data: &str, expected_debug: &str, expected_display: &str) {
    let cbor = CBOR::try_from_hex(data).unwrap();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
}

fn test_cbor_codable<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str)
where
T: TryFrom<CBOR> + Into<CBOR>,
<T as TryFrom<dcbor::CBOR>>::Error: std::fmt::Debug
{
    let cbor = t.into();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.to_cbor_data();
    assert_eq!(hex::encode(&data), expected_data);

    let decoded_cbor = CBOR::try_from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
    let t2 = T::try_from(decoded_cbor).unwrap();

    let cbor = t2.into();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.to_cbor_data();
    assert_eq!(hex::encode(data), expected_data);
}

#[test]
fn encode_unsigned() {
    test_cbor_codable(0u8,      "unsigned(0)", "0", "00");
    test_cbor_codable(0u16,     "unsigned(0)", "0", "00");
    test_cbor_codable(0u32,     "unsigned(0)", "0", "00");
    test_cbor_codable(0u64,     "unsigned(0)", "0", "00");
    test_cbor_codable(0_usize,  "unsigned(0)", "0", "00");

    test_cbor_codable(1u8,      "unsigned(1)", "1", "01");
    test_cbor_codable(1u16,     "unsigned(1)", "1", "01");
    test_cbor_codable(1u32,     "unsigned(1)", "1", "01");
    test_cbor_codable(1u64,     "unsigned(1)", "1", "01");
    test_cbor_codable(1_usize,  "unsigned(1)", "1", "01");

    test_cbor_codable(23u8,     "unsigned(23)", "23", "17");
    test_cbor_codable(23u16,    "unsigned(23)", "23", "17");
    test_cbor_codable(23u32,    "unsigned(23)", "23", "17");
    test_cbor_codable(23u64,    "unsigned(23)", "23", "17");
    test_cbor_codable(23_usize, "unsigned(23)", "23", "17");

    test_cbor_codable(24u8,     "unsigned(24)", "24", "1818");
    test_cbor_codable(24u16,    "unsigned(24)", "24", "1818");
    test_cbor_codable(24u32,    "unsigned(24)", "24", "1818");
    test_cbor_codable(24u64,    "unsigned(24)", "24", "1818");
    test_cbor_codable(24_usize, "unsigned(24)", "24", "1818");

    test_cbor_codable(u8::MAX,          "unsigned(255)", "255", "18ff");
    test_cbor_codable(u8::MAX as u16,   "unsigned(255)", "255", "18ff");
    test_cbor_codable(u8::MAX as u32,   "unsigned(255)", "255", "18ff");
    test_cbor_codable(u8::MAX as u64,   "unsigned(255)", "255", "18ff");
    test_cbor_codable(u8::MAX as usize, "unsigned(255)", "255", "18ff");

    test_cbor_codable(u16::MAX,          "unsigned(65535)", "65535", "19ffff");
    test_cbor_codable(u16::MAX as u32,   "unsigned(65535)", "65535", "19ffff");
    test_cbor_codable(u16::MAX as u64,   "unsigned(65535)", "65535", "19ffff");
    test_cbor_codable(u16::MAX as usize, "unsigned(65535)", "65535", "19ffff");

    test_cbor_codable(65536u32,          "unsigned(65536)", "65536", "1a00010000");
    test_cbor_codable(65536u64,          "unsigned(65536)", "65536", "1a00010000");
    test_cbor_codable(65536u64 as usize, "unsigned(65536)", "65536", "1a00010000");

    test_cbor_codable(u32::MAX,          "unsigned(4294967295)", "4294967295", "1affffffff");
    test_cbor_codable(u32::MAX as u64,   "unsigned(4294967295)", "4294967295", "1affffffff");
    test_cbor_codable(u32::MAX as usize, "unsigned(4294967295)", "4294967295", "1affffffff");

    test_cbor_codable(4294967296u64, "unsigned(4294967296)", "4294967296", "1b0000000100000000");

    test_cbor_codable(u64::MAX,          "unsigned(18446744073709551615)", "18446744073709551615", "1bffffffffffffffff");
    test_cbor_codable(u64::MAX as usize, "unsigned(18446744073709551615)", "18446744073709551615", "1bffffffffffffffff");
}

#[test]
fn encode_signed() {
    test_cbor_codable(-1i8,  "negative(-1)", "-1", "20");
    test_cbor_codable(-1i16, "negative(-1)", "-1", "20");
    test_cbor_codable(-1i32, "negative(-1)", "-1", "20");
    test_cbor_codable(-1i64, "negative(-1)", "-1", "20");

    test_cbor_codable(-2i8,  "negative(-2)", "-2", "21");
    test_cbor_codable(-2i16, "negative(-2)", "-2", "21");
    test_cbor_codable(-2i32, "negative(-2)", "-2", "21");
    test_cbor_codable(-2i64, "negative(-2)", "-2", "21");

    test_cbor_codable(-127i8,  "negative(-127)", "-127", "387e");
    test_cbor_codable(-127i16, "negative(-127)", "-127", "387e");
    test_cbor_codable(-127i32, "negative(-127)", "-127", "387e");
    test_cbor_codable(-127i64, "negative(-127)", "-127", "387e");

    test_cbor_codable(i8::MIN,        "negative(-128)", "-128", "387f");
    test_cbor_codable(i8::MIN as i16, "negative(-128)", "-128", "387f");
    test_cbor_codable(i8::MIN as i32, "negative(-128)", "-128", "387f");
    test_cbor_codable(i8::MIN as i64, "negative(-128)", "-128", "387f");

    test_cbor_codable(i8::MAX,        "unsigned(127)", "127", "187f");
    test_cbor_codable(i8::MAX as i16, "unsigned(127)", "127", "187f");
    test_cbor_codable(i8::MAX as i32, "unsigned(127)", "127", "187f");
    test_cbor_codable(i8::MAX as i64, "unsigned(127)", "127", "187f");

    test_cbor_codable(i16::MIN,        "negative(-32768)", "-32768", "397fff");
    test_cbor_codable(i16::MIN as i32, "negative(-32768)", "-32768", "397fff");
    test_cbor_codable(i16::MIN as i64, "negative(-32768)", "-32768", "397fff");

    test_cbor_codable(i16::MAX,        "unsigned(32767)", "32767", "197fff");
    test_cbor_codable(i16::MAX as i32, "unsigned(32767)", "32767", "197fff");
    test_cbor_codable(i16::MAX as i64, "unsigned(32767)", "32767", "197fff");

    test_cbor_codable(i32::MIN,        "negative(-2147483648)", "-2147483648", "3a7fffffff");
    test_cbor_codable(i32::MIN as i64, "negative(-2147483648)", "-2147483648", "3a7fffffff");

    test_cbor_codable(i32::MAX,        "unsigned(2147483647)", "2147483647", "1a7fffffff");
    test_cbor_codable(i32::MAX as i64, "unsigned(2147483647)", "2147483647", "1a7fffffff");

    test_cbor_codable(i64::MIN, "negative(-9223372036854775808)", "-9223372036854775808", "3b7fffffffffffffff");

    test_cbor_codable(i64::MAX, "unsigned(9223372036854775807)", "9223372036854775807", "1b7fffffffffffffff");
}

#[test]
fn encode_bytes_1() {
    test_cbor_codable(
        ByteString::from(hex!("00112233")),
            "bytes(00112233)",
            "h'00112233'",
            "4400112233"
        );
}

#[test]
fn encode_bytes() {
    test_cbor_codable(
        ByteString::from(hex!("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7")),
        "bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
        "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
        "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
    );
    let bytes = ByteString::from([0x11, 0x22, 0x33]);
    test_cbor_codable(bytes,
    "bytes(112233)",
    "h'112233'",
    "43112233"
    )
}

#[test]
fn encode_string() {
    test_cbor_codable("Hello".to_string(), r#"text("Hello")"#, r#""Hello""#, "6548656c6c6f");
    test_cbor_codable("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
    r#"text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")"#,
    r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#,
    "7901bd4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e");
}

#[test]
fn test_normalized_string() {
    let composed_e_acute = "\u{00E9}"; // é in NFC
    let decomposed_e_acute = "\u{0065}\u{0301}"; // e followed by ́ (combining acute accent) in NFD

    // Unlike in Swift, where string comparison is aware of compositional
    // differences, in Rust string comparison is not.
    assert_ne!(composed_e_acute, decomposed_e_acute);

    // And of course they serialize differently, which is not what we
    // want for determinism.
    let utf8_1 = composed_e_acute.as_bytes();
    let utf8_2 = decomposed_e_acute.as_bytes();
    assert_ne!(utf8_1, utf8_2);

    // But serializing them as dCBOR yields the same data.
    let cbor1 = CBOR::from(composed_e_acute).to_cbor_data();
    let cbor2 = CBOR::from(decomposed_e_acute).to_cbor_data();
    assert_eq!(cbor1, cbor2);

    let cbor_data = hex!("6365cc81");
    let cbor = CBOR::try_from_data(cbor_data);
    if let Err(e) = cbor {
        assert_eq!(format!("{}", e), "a CBOR string was not encoded in Unicode Canonical Normalization Form C");
    } else {
        panic!("Expected NonCanonicalString error");
    }
}

#[test]
fn encode_array() {
    test_cbor(vec![1, 2, 3], "array([unsigned(1), unsigned(2), unsigned(3)])", "[1, 2, 3]", "83010203");
    test_cbor([1, 2, 3], "array([unsigned(1), unsigned(2), unsigned(3)])", "[1, 2, 3]", "83010203");
    test_cbor([1, -2, 3], "array([unsigned(1), negative(-2), unsigned(3)])", "[1, -2, 3]", "83012103");
}

#[test]
fn encode_heterogenous_array() {
    let array: Vec<CBOR> = vec![
        1.into(),
        "Hello".into(),
        vec![1, 2, 3].into(),
    ];

    let cbor: CBOR = array.clone().into();
    let data = cbor.to_cbor_data();
    let decoded_cbor = CBOR::try_from_data(data).unwrap();
    match decoded_cbor.into_case() {
        CBORCase::Array(a) => {
            assert_eq!(a[0], 1.into());
            assert_eq!(a[1], "Hello".into());
            assert_eq!(a[2], [1, 2, 3].into());
        },
        _ => panic!(),
    };

    let expected_debug = r#"array([unsigned(1), text("Hello"), array([unsigned(1), unsigned(2), unsigned(3)])])"#;
    let expected_display = r#"[1, "Hello", [1, 2, 3]]"#;
    let expected_data = "83016548656c6c6f83010203";
    test_cbor(array, expected_debug, expected_display, expected_data);
}

#[test]
fn encode_map() {
    let mut m = Map::new();
    m.insert(-1, 3);
    m.insert(vec![-1], 7);
    m.insert("z", 4);
    m.insert(10, 1);
    m.insert(false, 8);
    m.insert(100, 2);
    m.insert("aa", 5);
    m.insert(vec![100], 6);
    test_cbor(m.clone(),
        r#"map({0x0a: (unsigned(10), unsigned(1)), 0x1864: (unsigned(100), unsigned(2)), 0x20: (negative(-1), unsigned(3)), 0x617a: (text("z"), unsigned(4)), 0x626161: (text("aa"), unsigned(5)), 0x811864: (array([unsigned(100)]), unsigned(6)), 0x8120: (array([negative(-1)]), unsigned(7)), 0xf4: (simple(false), unsigned(8))})"#,
        r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
        "a80a011864022003617a046261610581186406812007f408");

    assert_eq!(m.get::<bool, i32>(false), Some(8));
    assert_eq!(m.get::<bool, i32>(true), None);
    assert_eq!(m.extract::<i32, i32>(-1).unwrap(), 3);
    assert_eq!(m.extract::<Vec<i32>, i32>(vec![-1]).unwrap(), 7);
    assert_eq!(m.extract::<&str, i32>("z").unwrap(), 4);
    assert!(m.extract::<&str, i32>("foo").is_err());
}

#[test]
fn encode_map_with_map_keys() {
    let mut k1 = Map::new();
    k1.insert(1, 2);

    let mut k2 = Map::new();
    k2.insert(3, 4);

    let mut m = Map::new();
    m.insert(k1, 5);
    m.insert(k2, 6);
    test_cbor(m.clone(),
        r#"map({0xa10102: (map({0x01: (unsigned(1), unsigned(2))}), unsigned(5)), 0xa10304: (map({0x03: (unsigned(3), unsigned(4))}), unsigned(6))})"#,
        r#"{{1: 2}: 5, {3: 4}: 6}"#,
        "a2a1010205a1030406");
}

#[test]
fn encode_anders_map() {
    let mut m = Map::new();
    m.insert(1, 45.7);
    m.insert(2, "Hi there!");
    assert_eq!(m.clone().cbor_data(), hex!("a201fb4046d9999999999a0269486920746865726521"));
    assert_eq!(m.extract::<i32, f64>(1).unwrap(), 45.7);
}

#[test]
fn encode_map_misordered() {
    let cbor = CBOR::try_from_hex("a2026141016142");
    if let Err(e) = cbor {
        assert_eq!(format!("{}", e), "the decoded CBOR map has keys that are not in canonical order");
    } else {
        panic!("Expected MisorderedMapKey error");
    }
}

#[test]
fn encode_tagged() {
    test_cbor(CBOR::to_tagged_value(1, "Hello"), r#"tagged(1, text("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
}

#[test]
fn encode_value() {
    test_cbor(false, "simple(false)", "false", "f4");
    test_cbor(true, "simple(true)", "true", "f5");
}

#[test]
fn encode_envelope() {
    let alice = CBOR::to_tagged_value(200, CBOR::to_tagged_value(201, "Alice"));
    let knows = CBOR::to_tagged_value(200, CBOR::to_tagged_value(201, "knows"));
    let bob = CBOR::to_tagged_value(200, CBOR::to_tagged_value(201, "Bob"));
    let knows_bob = CBOR::to_tagged_value(200, CBOR::to_tagged_value(221, [knows, bob]));
    let envelope = CBOR::to_tagged_value(200, [alice, knows_bob]);
    assert_eq!(format!("{}", envelope), r#"200([200(201("Alice")), 200(221([200(201("knows")), 200(201("Bob"))]))])"#);
    let bytes = envelope.clone().to_cbor_data();
    assert_eq!(format!("{}", hex::encode(&bytes)), "d8c882d8c8d8c965416c696365d8c8d8dd82d8c8d8c9656b6e6f7773d8c8d8c963426f62");
    let decoded_cbor = CBOR::try_from_data(&bytes).unwrap();
    assert_eq!(envelope, decoded_cbor);
}

#[test]
fn encode_float() {
    // Floating point numbers get serialized as their shortest accurate representation.
    test_cbor(1.5,              "simple(1.5)",          "1.5",          "f93e00");
    test_cbor(2345678.25,       "simple(2345678.25)",   "2345678.25",   "fa4a0f2b39");
    test_cbor(1.2,              "simple(1.2)",          "1.2",          "fb3ff3333333333333");
    test_cbor(f64::INFINITY,    "simple(inf)",          "inf",          "f97c00");

    // Floating point values that can be represented as integers get serialized as integers.
    test_cbor(42.0f32,          "unsigned(42)",         "42",           "182a");
    test_cbor(2345678.0,        "unsigned(2345678)",    "2345678",      "1a0023cace");
    test_cbor(-2345678.0,       "negative(-2345678)",   "-2345678",     "3a0023cacd");

    // Negative zero gets serialized as integer zero.
    test_cbor(-0.0,             "unsigned(0)",          "0",            "00");

    // Smallest half-precision subnormal.
    test_cbor(5.960464477539063e-8, "simple(5.960464477539063e-8)", "5.960464477539063e-8", "f90001");

    // Smallest single subnormal.
    test_cbor(1.401298464324817e-45, "simple(1.401298464324817e-45)", "1.401298464324817e-45", "fa00000001");

    // Smallest double subnormal.
    test_cbor(5e-324, "simple(5e-324)", "5e-324", "fb0000000000000001");

    // Smallest double normal.
    test_cbor(2.2250738585072014e-308, "simple(2.2250738585072014e-308)", "2.2250738585072014e-308", "fb0010000000000000");

    // Smallest half-precision normal.
    test_cbor(6.103515625e-5, "simple(6.103515625e-5)", "6.103515625e-5", "f90400");

    // Largest possible half-precision.
    test_cbor(65504.0, "unsigned(65504)", "65504", "19ffe0");

    // Exponent 24 to test single exponent boundary.
    test_cbor(33554430.0, "unsigned(33554430)", "33554430", "1a01fffffe");

    // Most negative double that converts to int64.
    test_cbor(-9223372036854774784.0, "negative(-9223372036854774784)", "-9223372036854774784", "3b7ffffffffffffbff");

    // Int64 with too much precision to be a float.
    test_cbor(-9223372036854775807i64, "negative(-9223372036854775807)", "-9223372036854775807", "3b7ffffffffffffffe");

    // Most negative encoded as 65-bit neg
    // Can only be decoded as bignum
    test_cbor_decode("3b8000000000000000", "negative(-9223372036854775809)", "-9223372036854775809");

    // Largest double that can convert to uint64, almost UINT64_MAX.
    test_cbor(18446744073709550000.0, "unsigned(18446744073709549568)", "18446744073709549568", "1bfffffffffffff800");

    // Just too large to convert to uint64, but converts to a single, just over UINT64_MAX.
    test_cbor(18446744073709552000.0, "simple(1.8446744073709552e19)", "1.8446744073709552e19", "fa5f800000");

    // Least negative float not representable as Int64
    test_cbor(-9223372036854777856.0, "negative(-9223372036854777856)", "-9223372036854777856", "3b80000000000007ff");

    // Next to most negative float encodable as 65-bit neg
    test_cbor(-18446744073709549568.0, "negative(-18446744073709549568)", "-18446744073709549568", "3bfffffffffffff7ff");

    // 65-bit neg encoded
    // not representable as double
    test_cbor_decode("3bfffffffffffffffe", "negative(-18446744073709551615)", "-18446744073709551615");

    // Most negative encodable as a 65-bit neg
    test_cbor(-18446744073709551616.0, "negative(-18446744073709551616)", "-18446744073709551616", "3bffffffffffffffff");

    // Least negative whole integer that must be encoded as float in DCBOR (there are lots of non-whole-integer floats in the range of this table that must be DCBOR encoded as floats).
    test_cbor(-18446744073709555712.0, "simple(-1.8446744073709556e19)", "-1.8446744073709556e19", "fbc3f0000000000001");

    // Large negative that converts to negative int.
    test_cbor(-18446742974197924000.0, "negative(-18446742974197923840)", "-18446742974197923840", "3bfffffeffffffffff");

    // Largest possible single.
    test_cbor(3.4028234663852886e38, "simple(3.4028234663852886e38)", "3.4028234663852886e38", "fa7f7fffff");

    // Slightly larger than largest possible single.
    test_cbor(3.402823466385289e38, "simple(3.402823466385289e38)", "3.402823466385289e38", "fb47efffffe0000001");

    // Largest double.
    test_cbor(1.7976931348623157e308, "simple(1.7976931348623157e308)", "1.7976931348623157e308", "fb7fefffffffffffff");
}

#[test]
fn int_coerced_to_float() {
    let n = 42;
    let c: CBOR = n.into();
    let f: f64 = c.clone().try_into().unwrap();
    assert_eq!(f, n as f64);
    let c2: CBOR = f.into();
    assert_eq!(c2, c);
    let i: i32 = c.try_into().unwrap();
    assert_eq!(i, n);
}

#[test]
fn fail_float_coerced_to_int() {
    // Floating point values cannot be coerced to integer types.
    let n = 42.5;
    let c: CBOR = n.into();
    let f: f64 = c.clone().try_into().unwrap();
    assert_eq!(f, n);
    let a = i32::try_from(c);
    assert!(a.is_err());
}

#[test]
fn non_canonical_float_1() {
    // Non-canonical representation of 1.5 that could be represented at a smaller width.
    if let Err(e) = CBOR::try_from_hex("FB3FF8000000000000") {
        assert_eq!(format!("{}", e), "a CBOR numeric value was encoded in non-canonical form");
    } else {
        panic!("Expected NonCanonicalNumeric error");
    }
}

#[test]
fn non_canonical_float_2() {
    // Non-canonical representation of a floating point value that could be represented as an integer.
    if let Err(e) = CBOR::try_from_hex("F94A00") {
        assert_eq!(format!("{}", e), "a CBOR numeric value was encoded in non-canonical form");
    } else {
        panic!("Expected NonCanonicalNumeric error");
    }
}

#[test]
fn unused_data() {
    if let Err(e) = CBOR::try_from_hex("0001") {
        assert_eq!(format!("{}", e), "the decoded CBOR had 1 extra bytes at the end");
    } else {
        panic!("Expected UnusedData error");
    }
}

#[test]
fn tag() {
    let tag = Tag::new(1, "A");
    assert_eq!(format!("{}", tag), "A");
    assert_eq!(format!("{:?}", tag), r#"Tag { value: 1, name: Some(Dynamic("A")) }"#);
    let tag = Tag::with_value(2);
    assert_eq!(format!("{}", tag), "2");
    assert_eq!(format!("{:?}", tag), "Tag { value: 2, name: None }");
}

#[test]
fn encode_date() {
    test_cbor_codable(
        dcbor::Date::from_timestamp(1675854714.0),
        "tagged(1, unsigned(1675854714))",
        "1(1675854714)",
        "c11a63e3837a"
    )
}

fn test_convert<T>(value: T)
where
    T: PartialEq + Clone + Into<CBOR> + TryFrom<CBOR> + fmt::Debug,
    T::Error: fmt::Debug,
{
    let cbor = value.clone().into();
    let value2 = cbor.try_into().unwrap();
    assert_eq!(value, value2);
    let cbor_2: CBOR = value.clone().into();
    let value3 = cbor_2.try_into().unwrap();
    assert_eq!(value, value3);
}

#[test]
fn convert_values() {
    test_convert(10);
    test_convert(-10i64);
    test_convert(false);
    test_convert("Hello".to_string());
    test_convert(10.0);
    test_convert(ByteString::from(hex!("001122334455")));
}

#[test]
fn convert_hash_map() {
    let mut h = HashMap::<i32, String>::new();
    h.insert(1, "A".to_string());
    h.insert(50, "B".to_string());
    h.insert(25, "C".to_string());
    let m: CBOR = h.clone().into();
    assert_eq!(m.diagnostic(), r#"{1: "A", 25: "C", 50: "B"}"#);
    let h2: HashMap<i32, String> = m.try_into().unwrap();
    assert_eq!(h, h2);
}

#[test]
fn convert_btree_map() {
    let mut h = BTreeMap::<i32, String>::new();
    h.insert(1, "A".to_string());
    h.insert(50, "B".to_string());
    h.insert(25, "C".to_string());
    let m: CBOR = h.clone().into();
    assert_eq!(m.diagnostic(), r#"{1: "A", 25: "C", 50: "B"}"#);
    let h2: BTreeMap<i32, String> = m.try_into().unwrap();
    assert_eq!(h, h2);
}

#[test]
fn convert_vector() {
    let v: Vec<i32> = vec![1, 50, 25];
    let c: CBOR = v.clone().into();
    assert_eq!(c.diagnostic(), "[1, 50, 25]");
    let v2: Vec<i32> = c.try_into().unwrap();
    assert_eq!(v, v2);
}

#[test]
fn convert_vecdeque() {
    let mut v = VecDeque::<i32>::new();
    v.push_back(1);
    v.push_back(50);
    v.push_back(25);
    let c: CBOR = v.clone().into();
    assert_eq!(c.diagnostic(), "[1, 50, 25]");
    let v2: VecDeque<i32> = c.try_into().unwrap();
    assert_eq!(v, v2);
}

#[test]
fn convert_hashset() {
    let mut v = HashSet::<i32>::new();
    v.insert(1);
    v.insert(50);
    v.insert(25);
    let c: CBOR = v.clone().into();
    let v2: HashSet<i32> = c.try_into().unwrap();
    assert_eq!(v, v2);
}

#[test]
fn usage_test_1() {
    let array = [1000, 2000, 3000];
    let cbor: CBOR = array.into();
    assert_eq!(cbor.hex(), "831903e81907d0190bb8");
}

#[test]
fn usage_test_2() {
    let data = hex!("831903e81907d0190bb8");
    let cbor = CBOR::try_from_data(data).unwrap();
    assert_eq!(cbor.diagnostic(), "[1000, 2000, 3000]");
    let array: Vec<u32> = cbor.try_into().unwrap();
    assert_eq!(format!("{:?}", array), "[1000, 2000, 3000]");
}

#[test]
fn encode_nan() {
    let canonical_nan_data = hex!("f97e00");

    let nonstandard_f64_nan = f64::from_bits(0x7ff9100000000001);
    assert!(nonstandard_f64_nan.is_nan());
    assert_eq!(Into::<CBOR>::into(nonstandard_f64_nan).to_cbor_data(), canonical_nan_data);

    let nonstandard_f32_nan = f32::from_bits(0xffc00001);
    assert!(nonstandard_f32_nan.is_nan());
    assert_eq!(Into::<CBOR>::into(nonstandard_f32_nan).to_cbor_data(), canonical_nan_data);

    let nonstandard_f16_nan = f16::from_bits(0x7e01);
    assert!(nonstandard_f16_nan.is_nan());
    assert_eq!(Into::<CBOR>::into(nonstandard_f16_nan).to_cbor_data(), canonical_nan_data);
}

#[test]
fn decode_nan() {
    // Canonical NaN decodes
    let canonical_nan_data = hex!("f97e00");
    let d: f64 = CBOR::try_from_data(canonical_nan_data).unwrap().try_into().unwrap();
    assert!(d.is_nan());

    // Non-canonical NaNs of any size return an error
    CBOR::try_from_data(hex!("f97e01")).unwrap_err();
    CBOR::try_from_data(hex!("faffc00001")).unwrap_err();
    CBOR::try_from_data(hex!("fb7ff9100000000001")).unwrap_err();
}

#[test]
fn encode_infinit() {
    let canonical_infinity_data = hex!("f97c00");
    let canonical_neg_infinity_data = hex!("f9fc00");
    assert_eq!(Into::<CBOR>::into(f64::INFINITY).to_cbor_data(), canonical_infinity_data);
    assert_eq!(Into::<CBOR>::into(f32::INFINITY).to_cbor_data(), canonical_infinity_data);
    assert_eq!(Into::<CBOR>::into(f16::INFINITY).to_cbor_data(), canonical_infinity_data);
    assert_eq!(Into::<CBOR>::into(f64::NEG_INFINITY).to_cbor_data(), canonical_neg_infinity_data);
    assert_eq!(Into::<CBOR>::into(f32::NEG_INFINITY).to_cbor_data(), canonical_neg_infinity_data);
    assert_eq!(Into::<CBOR>::into(f16::NEG_INFINITY).to_cbor_data(), canonical_neg_infinity_data);
}

#[test]
fn decode_infinity() {
    let canonical_infinity_data = hex!("f97c00");
    let canonical_neg_infinity_data = hex!("f9fc00");

    // Canonical infinity decodes
    let a: f64 = CBOR::try_from_data(canonical_infinity_data).unwrap().try_into().unwrap();
    assert_eq!(a, f64::INFINITY);
    let a: f64 = CBOR::try_from_data(canonical_neg_infinity_data).unwrap().try_into().unwrap();
    assert_eq!(a, f64::NEG_INFINITY);

    // Non-canonical +infinities return error
    CBOR::try_from_data(hex!("fa7f800000")).err().unwrap();
    CBOR::try_from_data(hex!("fb7ff0000000000000")).err().unwrap();

    // Non-canonical -infinities return error
    CBOR::try_from_data(hex!("faff800000")).err().unwrap();
    CBOR::try_from_data(hex!("fbfff0000000000000")).err().unwrap();
}
