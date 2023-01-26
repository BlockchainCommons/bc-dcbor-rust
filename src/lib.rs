//! `dcbor` is a [CBOR](https://cbor.io) codec that focuses on writing and
//! parsing "deterministic" CBOR per [§4.2 of
//! RFC-8949](https://www.rfc-editor.org/rfc/rfc8949.html#name-deterministically-encoded-c).
//! It does not support parts of the spec forbidden by deterministic CBOR (such
//! as indefinite length arrays and maps). It also does not currently support
//! encoding or decoding floating point values. It is strict in both what it
//! writes and reads: in particular it will throw decoding errors if
//! variable-length integers are not encoded in their minimal form, or CBOR map
//! keys are not in lexicographic order, or there is extra data past the end of
//! the decoded CBOR item.

mod array;

mod bytes;
pub use bytes::Bytes;

mod cbor;
pub use cbor::{CBOR, CBOREncodable};

mod decode;
pub use decode::{decode_cbor, DecodeError};

mod hex;
pub use hex::{hex_to_bytes, bytes_to_hex};

mod int;

mod map;
pub use map::{Map, Iter};

mod string;

mod string_util;

mod tagged;
pub use tagged::Tagged;

mod value;
pub use value::Value;

mod varint;

#[cfg(test)]
mod test {
    use crate::{Tagged, CBOREncodable, decode_cbor, bytes_to_hex, Bytes, Map, hex_to_bytes, DecodeError, Value};

    fn test_cbor<T>(t: T, expected_debug: &str, expected_display: &str, expected_data: &str) where T: CBOREncodable {
        let cbor = t.cbor();
        assert_eq!(format!("{:?}", cbor), expected_debug);
        assert_eq!(format!("{}", cbor), expected_display);
        let data = cbor.encode_cbor();
        assert_eq!(bytes_to_hex(&data), expected_data);
        let decoded_cbor = decode_cbor(&data).unwrap();
        assert_eq!(cbor, decoded_cbor);
    }

    #[test]
    fn encode_unsigned() {
        test_cbor(0u8,        "UInt(0)", "0", "00");
        test_cbor(0u16,       "UInt(0)", "0", "00");
        test_cbor(0u32,       "UInt(0)", "0", "00");
        test_cbor(0u64,       "UInt(0)", "0", "00");
        test_cbor(0 as usize, "UInt(0)", "0", "00");

        test_cbor(1u8,        "UInt(1)", "1", "01");
        test_cbor(1u16,       "UInt(1)", "1", "01");
        test_cbor(1u32,       "UInt(1)", "1", "01");
        test_cbor(1u64,       "UInt(1)", "1", "01");
        test_cbor(1 as usize, "UInt(1)", "1", "01");

        test_cbor(23u8,        "UInt(23)", "23", "17");
        test_cbor(23u16,       "UInt(23)", "23", "17");
        test_cbor(23u32,       "UInt(23)", "23", "17");
        test_cbor(23u64,       "UInt(23)", "23", "17");
        test_cbor(23 as usize, "UInt(23)", "23", "17");

        test_cbor(24u8,        "UInt(24)", "24", "1818");
        test_cbor(24u16,       "UInt(24)", "24", "1818");
        test_cbor(24u32,       "UInt(24)", "24", "1818");
        test_cbor(24u64,       "UInt(24)", "24", "1818");
        test_cbor(24 as usize, "UInt(24)", "24", "1818");

        test_cbor(u8::MAX,          "UInt(255)", "255", "18ff");
        test_cbor(u8::MAX as u16,   "UInt(255)", "255", "18ff");
        test_cbor(u8::MAX as u32,   "UInt(255)", "255", "18ff");
        test_cbor(u8::MAX as u64,   "UInt(255)", "255", "18ff");
        test_cbor(u8::MAX as usize, "UInt(255)", "255", "18ff");

        test_cbor(u16::MAX,          "UInt(65535)", "65535", "19ffff");
        test_cbor(u16::MAX as u32,   "UInt(65535)", "65535", "19ffff");
        test_cbor(u16::MAX as u64,   "UInt(65535)", "65535", "19ffff");
        test_cbor(u16::MAX as usize, "UInt(65535)", "65535", "19ffff");

        test_cbor(65536u32,          "UInt(65536)", "65536", "1a00010000");
        test_cbor(65536u64,          "UInt(65536)", "65536", "1a00010000");
        test_cbor(65536u64 as usize, "UInt(65536)", "65536", "1a00010000");

        test_cbor(u32::MAX,          "UInt(4294967295)", "4294967295", "1affffffff");
        test_cbor(u32::MAX as u64,   "UInt(4294967295)", "4294967295", "1affffffff");
        test_cbor(u32::MAX as usize, "UInt(4294967295)", "4294967295", "1affffffff");

        test_cbor(4294967296u64, "UInt(4294967296)", "4294967296", "1b0000000100000000");

        test_cbor(u64::MAX,          "UInt(18446744073709551615)", "18446744073709551615", "1bffffffffffffffff");
        test_cbor(u64::MAX as usize, "UInt(18446744073709551615)", "18446744073709551615", "1bffffffffffffffff");
    }

    #[test]
    fn encode_signed() {
        test_cbor(-1i8,  "NInt(-1)", "-1", "20");
        test_cbor(-1i16, "NInt(-1)", "-1", "20");
        test_cbor(-1i32, "NInt(-1)", "-1", "20");
        test_cbor(-1i64, "NInt(-1)", "-1", "20");

        test_cbor(-2i8,  "NInt(-2)", "-2", "21");
        test_cbor(-2i16, "NInt(-2)", "-2", "21");
        test_cbor(-2i32, "NInt(-2)", "-2", "21");
        test_cbor(-2i64, "NInt(-2)", "-2", "21");

        test_cbor(-127i8,  "NInt(-127)", "-127", "387e");
        test_cbor(-127i16, "NInt(-127)", "-127", "387e");
        test_cbor(-127i32, "NInt(-127)", "-127", "387e");
        test_cbor(-127i64, "NInt(-127)", "-127", "387e");

        test_cbor(i8::MIN,        "NInt(-128)", "-128", "387f");
        test_cbor(i8::MIN as i16, "NInt(-128)", "-128", "387f");
        test_cbor(i8::MIN as i32, "NInt(-128)", "-128", "387f");
        test_cbor(i8::MIN as i64, "NInt(-128)", "-128", "387f");

        test_cbor(i8::MAX,        "UInt(127)", "127", "187f");
        test_cbor(i8::MAX as i16, "UInt(127)", "127", "187f");
        test_cbor(i8::MAX as i32, "UInt(127)", "127", "187f");
        test_cbor(i8::MAX as i64, "UInt(127)", "127", "187f");

        test_cbor(i16::MIN,        "NInt(-32768)", "-32768", "397fff");
        test_cbor(i16::MIN as i32, "NInt(-32768)", "-32768", "397fff");
        test_cbor(i16::MIN as i64, "NInt(-32768)", "-32768", "397fff");

        test_cbor(i16::MAX,        "UInt(32767)", "32767", "197fff");
        test_cbor(i16::MAX as i32, "UInt(32767)", "32767", "197fff");
        test_cbor(i16::MAX as i64, "UInt(32767)", "32767", "197fff");

        test_cbor(i32::MIN,        "NInt(-2147483648)", "-2147483648", "3a7fffffff");
        test_cbor(i32::MIN as i64, "NInt(-2147483648)", "-2147483648", "3a7fffffff");

        test_cbor(i32::MAX,        "UInt(2147483647)", "2147483647", "1a7fffffff");
        test_cbor(i32::MAX as i64, "UInt(2147483647)", "2147483647", "1a7fffffff");

        test_cbor(i64::MIN, "NInt(-9223372036854775808)", "-9223372036854775808", "3b7fffffffffffffff");

        test_cbor(i64::MAX, "UInt(9223372036854775807)", "9223372036854775807", "1b7fffffffffffffff");
    }

    #[test]
    fn encode_bytes() {
        // test_cbor(Bytes::new([0x11, 0x22, 0x33]), "Bytes(112233)", "43112233");
        test_cbor(
            Bytes::from_hex("c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"),
            "Bytes(c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7)",
            "h'c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7'",
            "5820c0a7da14e5847c526244f7e083d26fe33f86d2313ad2b77164233444423a50a7"
        );
        test_cbor(Bytes::new([0x11, 0x22, 0x33]),
        "Bytes(112233)",
        "h'112233'",
        "43112233"
        )
    }

    #[test]
    fn encode_string() {
        test_cbor("Hello", r#"String("Hello")"#, r#""Hello""#, "6548656c6c6f");
        test_cbor("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
        r#"String("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")"#,
        r#""Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""#,
        "7901bd4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20557420656e696d206164206d696e696d2076656e69616d2c2071756973206e6f737472756420657865726369746174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c697175697020657820656120636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f7220696e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e204578636570746575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e696d20696420657374206c61626f72756d2e");
    }

    #[test]
    fn encode_array() {
        test_cbor(vec![1, 2, 3], "Array([UInt(1), UInt(2), UInt(3)])", "[1, 2, 3]", "83010203");
        test_cbor([1, 2, 3], "Array([UInt(1), UInt(2), UInt(3)])", "[1, 2, 3]", "83010203");
        test_cbor(&[1, -2, 3], "Array([UInt(1), NInt(-2), UInt(3)])", "[1, -2, 3]", "83012103");
        let mut array: Vec<Box<dyn CBOREncodable>> = Vec::new();
        array.push(Box::new(1));
        array.push(Box::new("Hello"));
        array.push(Box::new([1, 2, 3]));
        test_cbor(array,
        "Array([UInt(1), String(\"Hello\"), Array([UInt(1), UInt(2), UInt(3)])])",
        r#"[1, "Hello", [1, 2, 3]]"#,
        "83016548656c6c6f83010203");
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
            r#"Map({0x0a: (UInt(10), UInt(1)), 0x1864: (UInt(100), UInt(2)), 0x20: (NInt(-1), UInt(3)), 0x617a: (String("z"), UInt(4)), 0x626161: (String("aa"), UInt(5)), 0x811864: (Array([UInt(100)]), UInt(6)), 0x8120: (Array([NInt(-1)]), UInt(7)), 0xf4: (Value(false), UInt(8))})"#,
            r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
            "a80a011864022003617a046261610581186406812007f408");
    }

    #[test]
    fn encode_map_misordered() {
        let cbor = decode_cbor(&hex_to_bytes("a2026141016142"));
        assert_eq!(cbor, Err(DecodeError::MisorderedMapKey));
    }

    #[test]
    fn encode_tagged() {
        test_cbor(Tagged::new(1, "Hello"), r#"Tagged(1, String("Hello"))"#, r#"1("Hello")"#, "c16548656c6c6f");
    }

    #[test]
    fn encode_value() {
        test_cbor(false, "Value(false)", "false", "f4");
        test_cbor(true, "Value(true)", "true", "f5");
        test_cbor(Value::new(100), "Value(100)", "simple(100)", "f864");
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
        let bytes = cbor.encode_cbor();
        assert_eq!(format!("{}", bytes_to_hex(&bytes)), "d8c882d8c8d81865416c696365d8c8d8dd82d8c8d818656b6e6f7773d8c8d81863426f62");
        let decoded_cbor = decode_cbor(&bytes).unwrap();
        assert_eq!(cbor, decoded_cbor);
    }

    #[test]
    fn unused_data() {
        let cbor = decode_cbor(&hex_to_bytes("0001"));
        assert_eq!(cbor, Err(DecodeError::UnusedData(1)));
    }
}
