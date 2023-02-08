use crate::{Tagged, bytes_to_hex, Data, Map, Simple, decode_error::DecodeError, tag::Tag, cbor_encodable::CBOREncodable, CBORCodable, CBOR, Date};

fn test_cbor<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBOREncodable {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(bytes_to_hex(&data), expected_data);
    let decoded_cbor = CBOR::from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
}

fn test_cbor_codable<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBORCodable {
    let cbor = t.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(bytes_to_hex(&data), expected_data);

    let decoded_cbor = CBOR::from_data(&data).unwrap();
    assert_eq!(cbor, decoded_cbor);
    let t2 = T::from_cbor(&decoded_cbor).unwrap();

    let cbor = t2.cbor();
    assert_eq!(format!("{:?}", cbor), expected_debug);
    assert_eq!(format!("{}", cbor), expected_display);
    let data = cbor.cbor_data();
    assert_eq!(bytes_to_hex(&data), expected_data);
}

#[test]
fn encode_unsigned() {
    test_cbor_codable(0u8,        "unsigned(0)", "0", "00");
    test_cbor_codable(0u16,       "unsigned(0)", "0", "00");
    test_cbor_codable(0u32,       "unsigned(0)", "0", "00");
    test_cbor_codable(0u64,       "unsigned(0)", "0", "00");
    test_cbor_codable(0 as usize, "unsigned(0)", "0", "00");

    test_cbor_codable(1u8,        "unsigned(1)", "1", "01");
    test_cbor_codable(1u16,       "unsigned(1)", "1", "01");
    test_cbor_codable(1u32,       "unsigned(1)", "1", "01");
    test_cbor_codable(1u64,       "unsigned(1)", "1", "01");
    test_cbor_codable(1 as usize, "unsigned(1)", "1", "01");

    test_cbor_codable(23u8,        "unsigned(23)", "23", "17");
    test_cbor_codable(23u16,       "unsigned(23)", "23", "17");
    test_cbor_codable(23u32,       "unsigned(23)", "23", "17");
    test_cbor_codable(23u64,       "unsigned(23)", "23", "17");
    test_cbor_codable(23 as usize, "unsigned(23)", "23", "17");

    test_cbor_codable(24u8,        "unsigned(24)", "24", "1818");
    test_cbor_codable(24u16,       "unsigned(24)", "24", "1818");
    test_cbor_codable(24u32,       "unsigned(24)", "24", "1818");
    test_cbor_codable(24u64,       "unsigned(24)", "24", "1818");
    test_cbor_codable(24 as usize, "unsigned(24)", "24", "1818");

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
fn encode_bytes() {
    test_cbor_codable(
        Data::from_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"),
        "bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
        "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
        "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
    );
    test_cbor_codable(Data::from_data([0x11, 0x22, 0x33]),
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
    test_cbor(&[1, -2, 3], "array([unsigned(1), negative(-2), unsigned(3)])", "[1, -2, 3]", "83012103");
}

#[test]
fn encode_heterogenous_array() {
    let mut array: Vec<Box<dyn CBOREncodable>> = Vec::new();
    array.push(Box::new(1));
    array.push(Box::new("Hello"));
    array.push(Box::new([1, 2, 3]));

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
    test_cbor(m,
        r#"map({0x0a: (unsigned(10), unsigned(1)), 0x1864: (unsigned(100), unsigned(2)), 0x20: (negative(-1), unsigned(3)), 0x617a: (text("z"), unsigned(4)), 0x626161: (text("aa"), unsigned(5)), 0x811864: (array([unsigned(100)]), unsigned(6)), 0x8120: (array([negative(-1)]), unsigned(7)), 0xf4: (simple(false), unsigned(8))})"#,
        r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
        "a80a011864022003617a046261610581186406812007f408");
}

#[test]
fn encode_map_misordered() {
    let cbor = CBOR::from_hex("a2026141016142");
    assert_eq!(cbor, Err(DecodeError::MisorderedMapKey));
}

#[test]
fn encode_tagged() {
    test_cbor(Tagged::new(1, "Hello"), r#"tagged(1, text("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
}

#[test]
fn encode_value() {
    test_cbor(false, "simple(false)", "false", "f4");
    test_cbor(true, "simple(true)", "true", "f5");
    test_cbor(Simple::new(100), "simple(100)", "simple(100)", "f864");
}

#[test]
fn encode_envelope() {
    let alice = Tagged::new(200, Tagged::new(24, "Alice"));
    let knows = Tagged::new(200, Tagged::new(24, "knows"));
    let bob = Tagged::new(200, Tagged::new(24, "Bob"));
    let knows_bob = Tagged::new(200, Tagged::new(221, [knows, bob]));
    let envelope = Tagged::new(200, [alice, knows_bob]);
    let cbor = envelope.cbor();
    assert_eq!(format!("{}", cbor), r#"200([200(24("Alice")), 200(221([200(24("knows")), 200(24("Bob"))]))])"#);
    let bytes = cbor.cbor_data();
    assert_eq!(format!("{}", bytes_to_hex(&bytes)), "d8c882d8c8d81865416c696365d8c8d8dd82d8c8d818656b6e6f7773d8c8d81863426f62");
    let decoded_cbor = CBOR::from_data(&bytes).unwrap();
    assert_eq!(cbor, decoded_cbor);
}

#[test]
fn unused_data() {
    let cbor = CBOR::from_hex("0001");
    assert_eq!(cbor, Err(DecodeError::UnusedData(1)));
}

#[test]
fn tag() {
    let tag = Tag::new_opt(1, Some(&"A"));
    assert_eq!(format!("{}", tag), "A");
    assert_eq!(format!("{:?}", tag), r#"Tag { value: 1, name: Some("A") }"#);
    let tag = Tag::new_opt(2, None);
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
