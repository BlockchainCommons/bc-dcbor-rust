use super::{cbor::{CBOREncodable, CBOR}, varint::{EncodeVarInt, MajorType}};

impl CBOREncodable for u8 {
    fn as_cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.encode_varint(MajorType::UInt)
    }
}

impl CBOREncodable for u16 {
    fn as_cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.encode_varint(MajorType::UInt)
    }
}

impl CBOREncodable for u32 {
    fn as_cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.encode_varint(MajorType::UInt)
    }
}

impl CBOREncodable for u64 {
    fn as_cbor(&self) -> CBOR {
        CBOR::UInt(*self)
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.encode_varint(MajorType::UInt)
    }
}

impl CBOREncodable for usize {
    fn as_cbor(&self) -> CBOR {
        CBOR::UInt(*self as u64)
    }

    fn encode_cbor(&self) -> Vec<u8> {
        self.encode_varint(MajorType::UInt)
    }
}

impl CBOREncodable for i8 {
    fn as_cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn encode_cbor(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i16;
            let a = (-b - 1) as u8;
            a.encode_varint(MajorType::NInt)
        } else {
            let a = *self as u8;
            a.encode_varint(MajorType::UInt)
        }
    }
}

impl CBOREncodable for i16 {
    fn as_cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn encode_cbor(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i32;
            let a = (-b - 1) as u16;
            a.encode_varint(MajorType::NInt)
        } else {
            let a = *self as u16;
            a.encode_varint(MajorType::UInt)
        }
    }
}

impl CBOREncodable for i32 {
    fn as_cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn encode_cbor(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i64;
            let a = (-b - 1) as u32;
            a.encode_varint(MajorType::NInt)
        } else {
            let a = *self as u32;
            a.encode_varint(MajorType::UInt)
        }
    }
}

impl CBOREncodable for i64 {
    fn as_cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NInt(*self as i64)
        } else {
            CBOR::UInt(*self as u64)
        }
    }

    fn encode_cbor(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as u64;
            let a = (-(b as i128) - 1) as u64;
            a.encode_varint(MajorType::NInt)
        } else {
            let a = *self as u64;
            a.encode_varint(MajorType::UInt)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::test_util::test_cbor;

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
}
