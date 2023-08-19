use std::collections::{HashMap, BTreeMap, VecDeque, HashSet};

use dcbor::*;
use half::f16;
use hex_literal::hex;

fn test_cbor<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBOREncodable {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(hex::encode(&data), expected_data);
    let decoded_cbor = CBOR::from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}

fn test_cbor_codable<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBORCodable {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(hex::encode(&data), expected_data);

    let decoded_cbor = CBOR::from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
    let t2 = T::from_cbor(&decoded_cbor).unwrap();

    let cbor = t2.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(hex::encode(data), expected_data);
}

#[test]
fn encode_unsigned() {
    test_cbor_codable(0u8,        "unsigned(0)", "0", "00");
    test_cbor_codable(0u16,       "unsigned(0)", "0", "00");
    test_cbor_codable(0u32,       "unsigned(0)", "0", "00");
    test_cbor_codable(0u64,       "unsigned(0)", "0", "00");
    test_cbor_codable(0_usize, "unsigned(0)", "0", "00");

    test_cbor_codable(1u8,        "unsigned(1)", "1", "01");
    test_cbor_codable(1u16,       "unsigned(1)", "1", "01");
    test_cbor_codable(1u32,       "unsigned(1)", "1", "01");
    test_cbor_codable(1u64,       "unsigned(1)", "1", "01");
    test_cbor_codable(1_usize, "unsigned(1)", "1", "01");

    test_cbor_codable(23u8,        "unsigned(23)", "23", "17");
    test_cbor_codable(23u16,       "unsigned(23)", "23", "17");
    test_cbor_codable(23u32,       "unsigned(23)", "23", "17");
    test_cbor_codable(23u64,       "unsigned(23)", "23", "17");
    test_cbor_codable(23_usize, "unsigned(23)", "23", "17");

    test_cbor_codable(24u8,        "unsigned(24)", "24", "1818");
    test_cbor_codable(24u16,       "unsigned(24)", "24", "1818");
    test_cbor_codable(24u32,       "unsigned(24)", "24", "1818");
    test_cbor_codable(24u64,       "unsigned(24)", "24", "1818");
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
    test_cbor_codable(CBOR::byte_string_hex("00112233"), "bytes(00112233)", "h'00112233'", "4400112233");
}

#[test]
fn encode_bytes() {
    test_cbor_codable(
        CBOR::byte_string_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"),
        "bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
        "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
        "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
    );
    test_cbor_codable(CBOR::byte_string([0x11, 0x22, 0x33]),
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
fn encode_array() {
    test_cbor(vec![1, 2, 3], "array([unsigned(1), unsigned(2), unsigned(3)])", "[1, 2, 3]", "83010203");
    test_cbor([1, 2, 3], "array([unsigned(1), unsigned(2), unsigned(3)])", "[1, 2, 3]", "83010203");
    test_cbor([1, -2, 3], "array([unsigned(1), negative(-2), unsigned(3)])", "[1, -2, 3]", "83012103");
}

#[test]
fn encode_heterogenous_array() {
    let array = vec![
        1.cbor(),
        "Hello".cbor(),
        vec![1, 2, 3].cbor(),
    ];

    let cbor = array.cbor();
    let data = cbor.cbor_data();
    let decoded_cbor = CBOR::from_data(&data).unwrap();
    match decoded_cbor {
        CBOR::Array(a) => {
            assert_eq!(a[0], 1.cbor());
            assert_eq!(a[1], "Hello".cbor());
            assert_eq!(a[2], [1, 2, 3].cbor());
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
    m.insert_into(-1, 3);
    m.insert_into(vec![-1], 7);
    m.insert_into("z", 4);
    m.insert_into(10, 1);
    m.insert_into(false, 8);
    m.insert_into(100, 2);
    m.insert_into("aa", 5);
    m.insert_into(vec![100], 6);
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
fn encode_anders_map() {
    let mut m = Map::new();
    m.insert_into(1, 45.7);
    m.insert_into(2, "Hi there!");
    assert_eq!(m.cbor_data(), hex!("a201fb4046d9999999999a0269486920746865726521"));
    assert_eq!(m.extract::<i32, f64>(1).unwrap(), 45.7);
}

#[test]
fn encode_map_misordered() {
    let cbor = CBOR::from_hex("a2026141016142");
    if let Err(Error::MisorderedMapKey) = cbor {
    } else {
        panic!("Expected MisorderedMapKey error");
    }
}

#[test]
fn encode_tagged() {
    test_cbor(CBOR::tagged_value(1, "Hello"), r#"tagged(1, text("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
}

#[test]
fn encode_value() {
    test_cbor(false, "simple(false)", "false", "f4");
    test_cbor(true, "simple(true)", "true", "f5");
}

#[test]
fn encode_envelope() {
    let alice = CBOR::tagged_value(200, CBOR::tagged_value(24, "Alice"));
    let knows = CBOR::tagged_value(200, CBOR::tagged_value(24, "knows"));
    let bob = CBOR::tagged_value(200, CBOR::tagged_value(24, "Bob"));
    let knows_bob = CBOR::tagged_value(200, CBOR::tagged_value(221, [knows, bob]));
    let envelope = CBOR::tagged_value(200, [alice, knows_bob]);
    let cbor = envelope.cbor();
    assert_eq!(format!("{}", cbor), r#"200([200(24("Alice")), 200(221([200(24("knows")), 200(24("Bob"))]))])"#);
    let bytes = cbor.cbor_data();
    assert_eq!(format!("{}", hex::encode(&bytes)), "d8c882d8c8d81865416c696365d8c8d8dd82d8c8d818656b6e6f7773d8c8d81863426f62");
    let decoded_cbor = CBOR::from_data(&bytes).unwrap();
    assert_eq!(cbor, decoded_cbor);
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
}

#[test]
fn int_coerced_to_float() {
    let n = 42;
    let c = n.cbor();
    let f: f64 = (&c).try_into().unwrap();
    assert_eq!(f, n as f64);
    let c2 = f.cbor();
    assert_eq!(c2, c);
    let i: i32 = (&c).try_into().unwrap();
    assert_eq!(i, n);
}

#[test]
fn fail_float_coerced_to_int() {
    // Floating point values cannot be coerced to integer types.
    let n = 42.5;
    let c = n.cbor();
    let f: f64 = (&c).try_into().unwrap();
    assert_eq!(f, n);
    if let Err(Error::WrongType) = i32::try_from(&c) {
    } else {
        panic!("Expected WrongType error");
    }
}

#[test]
fn non_canonical_float_1() {
    // Non-canonical representation of 1.5 that could be represented at a smaller width.
    if let Err(Error::NonCanonicalNumeric) = CBOR::from_hex("FB3FF8000000000000") {
    } else {
        panic!("Expected NonCanonicalNumeric error");
    }
}

#[test]
fn non_canonical_float_2() {
    // Non-canonical representation of a floating point value that could be represented as an integer.
    if let Err(Error::NonCanonicalNumeric) = CBOR::from_hex("F94A00") {
    } else {
        panic!("Expected NonCanonicalNumeric error");
    }
}

#[test]
fn unused_data() {
    if let Err(Error::UnusedData(1)) = CBOR::from_hex("0001") {
    } else {
        panic!("Expected UnusedData error");
    }
}

#[test]
fn tag() {
    let tag = Tag::new_with_name(1, "A");
    assert_eq!(format!("{}", tag), "A");
    assert_eq!(format!("{:?}", tag), r#"Tag { value: 1, name: Some(Dynamic("A")) }"#);
    let tag = Tag::new(2);
    assert_eq!(format!("{}", tag), "2");
    assert_eq!(format!("{:?}", tag), "Tag { value: 2, name: None }");
}

#[test]
fn encode_date() {
    test_cbor_codable(
        Date::from_timestamp(1675854714),
        "tagged(1, unsigned(1675854714))",
        "1(1675854714)",
        "c11a63e3837a"
    )
}

fn test_convert<T>(value: T)
where
    T: PartialEq + Clone + Into<CBOR> + From<CBOR> + TryFrom<CBOR> + std::fmt::Debug,
    T::Error: std::fmt::Debug,
{
    let cbor = value.clone().into();
    let value2 = cbor.try_into().unwrap();
    assert_eq!(value, value2);
    let value3 = T::from(value.clone().into());
    assert_eq!(value, value3);
}

#[test]
fn convert_values() {
    test_convert(10);
    test_convert(-10i64);
    test_convert(false);
    test_convert("Hello".to_string());
    test_convert(10.0);
    test_convert(CBOR::byte_string_hex("001122334455"))
}

#[test]
fn convert_hash_map() {
    let mut h = HashMap::<i32, String>::new();
    h.insert(1, "A".to_string());
    h.insert(50, "B".to_string());
    h.insert(25, "C".to_string());
    let m = h.cbor();
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
    let m = h.cbor();
    assert_eq!(m.diagnostic(), r#"{1: "A", 25: "C", 50: "B"}"#);
    let h2: BTreeMap<i32, String> = m.try_into().unwrap();
    assert_eq!(h, h2);
}

#[test]
fn convert_vector() {
    let v: Vec<i32> = vec![1, 50, 25];
    let c = v.cbor();
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
    let c = v.cbor();
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
    let c = v.cbor();
    let v2: HashSet<i32> = c.try_into().unwrap();
    assert_eq!(v, v2);
}

#[test]
fn usage_test_1() {
    let array = [1000, 2000, 3000];
    let cbor = array.cbor();
    assert_eq!(cbor.hex(), "831903e81907d0190bb8");
}

#[test]
fn usage_test_2() {
    let data = hex!("831903e81907d0190bb8");
    let cbor = CBOR::from_data(&data).unwrap();
    assert_eq!(cbor.diagnostic(), "[1000, 2000, 3000]");
    let array: Vec::<u32> = cbor.try_into().unwrap();
    assert_eq!(format!("{:?}", array), "[1000, 2000, 3000]");
}

#[test]
fn encode_nan() {
    let canonical_nan_data = hex!("f97e00");

    let nonstandard_f64_nan = f64::from_bits(0x7ff9100000000001);
    assert!(nonstandard_f64_nan.is_nan());
    assert_eq!(nonstandard_f64_nan.cbor_data(), canonical_nan_data);

    let nonstandard_f32_nan = f32::from_bits(0xffc00001);
    assert!(nonstandard_f32_nan.is_nan());
    assert_eq!(nonstandard_f32_nan.cbor_data(), canonical_nan_data);

    let nonstandard_f16_nan = f16::from_bits(0x7e01);
    assert!(nonstandard_f16_nan.is_nan());
    assert_eq!(nonstandard_f16_nan.cbor_data(), canonical_nan_data);
}

#[test]
fn decode_nan() {
    // Canonical NaN decodes
    let canonical_nan_data = hex!("f97e00");
    let d: f64 = CBOR::from_data(&canonical_nan_data).unwrap().into();
    assert!(d.is_nan());

    // Non-canonical NaNs of any size return an error
    CBOR::from_data(&hex!("f97e01")).unwrap_err();
    CBOR::from_data(&hex!("faffc00001")).unwrap_err();
    CBOR::from_data(&hex!("fb7ff9100000000001")).unwrap_err();
}

#[test]
fn encode_infinit() {
    let canonical_infinity_data = hex!("f97c00");
    let canonical_neg_infinity_data = hex!("f9fc00");
    assert_eq!(f64::INFINITY.cbor_data(), canonical_infinity_data);
    assert_eq!(f32::INFINITY.cbor_data(), canonical_infinity_data);
    assert_eq!(f16::INFINITY.cbor_data(), canonical_infinity_data);
    assert_eq!(f64::NEG_INFINITY.cbor_data(), canonical_neg_infinity_data);
    assert_eq!(f32::NEG_INFINITY.cbor_data(), canonical_neg_infinity_data);
    assert_eq!(f16::NEG_INFINITY.cbor_data(), canonical_neg_infinity_data);
}

#[test]
fn decode_infinity() {
    let canonical_infinity_data = hex!("f97c00");
    let canonical_neg_infinity_data = hex!("f9fc00");

    // Canonical infinity decodes
    let a: f64 = CBOR::from_data(&canonical_infinity_data).unwrap().into();
    assert_eq!(a, f64::INFINITY);
    let a: f64 = CBOR::from_data(&canonical_neg_infinity_data).unwrap().into();
    assert_eq!(a, f64::NEG_INFINITY);

    // Non-canonical +infinities return error
    CBOR::from_data(&hex!("fa7f800000")).err().unwrap();
    CBOR::from_data(&hex!("fb7ff0000000000000")).err().unwrap();

    // Non-canonical -infinities return error
    CBOR::from_data(&hex!("faff800000")).err().unwrap();
    CBOR::from_data(&hex!("fbfff0000000000000")).err().unwrap();
}
