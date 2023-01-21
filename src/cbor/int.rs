use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

impl CBOREncode for u8 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::UINT)
    }
}

impl IntoCBOR for u8 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u16 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::UINT)
    }
}

impl IntoCBOR for u16 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u32 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::UINT)
    }
}

impl IntoCBOR for u32 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u64 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::UINT)
    }
}

impl IntoCBOR for u64 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self)
    }
}

impl CBOREncode for usize {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::UINT)
    }
}

impl IntoCBOR for usize {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for i8 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i16;
            let a = (-b - 1) as u8;
            a.varint_encode(MajorType::NINT)
        } else {
            let a = *self as u8;
            a.varint_encode(MajorType::UINT)
        }
    }
}

impl IntoCBOR for i8 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i64)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

impl CBOREncode for i16 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i32;
            let a = (-b - 1) as u16;
            a.varint_encode(MajorType::NINT)
        } else {
            let a = *self as u16;
            a.varint_encode(MajorType::UINT)
        }
    }
}

impl IntoCBOR for i16 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i64)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

impl CBOREncode for i32 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i64;
            let a = (-b - 1) as u32;
            a.varint_encode(MajorType::NINT)
        } else {
            let a = *self as u32;
            a.varint_encode(MajorType::UINT)
        }
    }
}

impl IntoCBOR for i32 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i64)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

impl CBOREncode for i64 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as u64;
            let a = (-(b as i128) - 1) as u64;
            a.varint_encode(MajorType::NINT)
        } else {
            let a = *self as u64;
            a.varint_encode(MajorType::UINT)
        }
    }
}

impl IntoCBOR for i64 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i64)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    #[test]
    fn encode_unsigned() {
        test_cbor(0u8, "UINT(0)", "00");
        test_cbor(0u16, "UINT(0)", "00");
        test_cbor(0u32, "UINT(0)", "00");
        test_cbor(0u64, "UINT(0)", "00");
        test_cbor(0 as usize, "UINT(0)", "00");

        test_cbor(1u8, "UINT(1)", "01");
        test_cbor(1u16, "UINT(1)", "01");
        test_cbor(1u32, "UINT(1)", "01");
        test_cbor(1u64, "UINT(1)", "01");
        test_cbor(1 as usize, "UINT(1)", "01");

        test_cbor(23u8, "UINT(23)", "17");
        test_cbor(23u16, "UINT(23)", "17");
        test_cbor(23u32, "UINT(23)", "17");
        test_cbor(23u64, "UINT(23)", "17");
        test_cbor(23 as usize, "UINT(23)", "17");

        test_cbor(24u8, "UINT(24)", "1818");
        test_cbor(24u16, "UINT(24)", "1818");
        test_cbor(24u32, "UINT(24)", "1818");
        test_cbor(24u64, "UINT(24)", "1818");
        test_cbor(24 as usize, "UINT(24)", "1818");

        test_cbor(u8::MAX, "UINT(255)", "18ff");
        test_cbor(u8::MAX as u16, "UINT(255)", "18ff");
        test_cbor(u8::MAX as u32, "UINT(255)", "18ff");
        test_cbor(u8::MAX as u64, "UINT(255)", "18ff");
        test_cbor(u8::MAX as usize, "UINT(255)", "18ff");

        test_cbor(u16::MAX, "UINT(65535)", "19ffff");
        test_cbor(u16::MAX as u32, "UINT(65535)", "19ffff");
        test_cbor(u16::MAX as u64, "UINT(65535)", "19ffff");
        test_cbor(u16::MAX as usize, "UINT(65535)", "19ffff");

        test_cbor(65536u32, "UINT(65536)", "1a00010000");
        test_cbor(65536u64, "UINT(65536)", "1a00010000");
        test_cbor(65536u64 as usize, "UINT(65536)", "1a00010000");

        test_cbor(u32::MAX, "UINT(4294967295)", "1affffffff");
        test_cbor(u32::MAX as u64, "UINT(4294967295)", "1affffffff");
        test_cbor(u32::MAX as usize, "UINT(4294967295)", "1affffffff");

        test_cbor(4294967296u64, "UINT(4294967296)", "1b0000000100000000");

        test_cbor(u64::MAX, "UINT(18446744073709551615)", "1bffffffffffffffff");
        test_cbor(u64::MAX as usize, "UINT(18446744073709551615)", "1bffffffffffffffff");
    }

    #[test]
    fn encode_signed() {
        test_cbor(-1i8, "NINT(-1)", "20");
        test_cbor(-1i16, "NINT(-1)", "20");
        test_cbor(-1i32, "NINT(-1)", "20");
        test_cbor(-1i64, "NINT(-1)", "20");

        test_cbor(-2i8, "NINT(-2)", "21");
        test_cbor(-2i16, "NINT(-2)", "21");
        test_cbor(-2i32, "NINT(-2)", "21");
        test_cbor(-2i64, "NINT(-2)", "21");

        test_cbor(-127i8, "NINT(-127)", "387e");
        test_cbor(-127i16, "NINT(-127)", "387e");
        test_cbor(-127i32, "NINT(-127)", "387e");
        test_cbor(-127i64, "NINT(-127)", "387e");

        test_cbor(i8::MIN, "NINT(-128)", "387f");
        test_cbor(i8::MIN as i16, "NINT(-128)", "387f");
        test_cbor(i8::MIN as i32, "NINT(-128)", "387f");
        test_cbor(i8::MIN as i64, "NINT(-128)", "387f");

        test_cbor(i8::MAX, "UINT(127)", "187f");
        test_cbor(i8::MAX as i16, "UINT(127)", "187f");
        test_cbor(i8::MAX as i32, "UINT(127)", "187f");
        test_cbor(i8::MAX as i64, "UINT(127)", "187f");

        test_cbor(i16::MIN, "NINT(-32768)", "397fff");
        test_cbor(i16::MIN as i32, "NINT(-32768)", "397fff");
        test_cbor(i16::MIN as i64, "NINT(-32768)", "397fff");

        test_cbor(i16::MAX, "UINT(32767)", "197fff");
        test_cbor(i16::MAX as i32, "UINT(32767)", "197fff");
        test_cbor(i16::MAX as i64, "UINT(32767)", "197fff");

        test_cbor(i32::MIN, "NINT(-2147483648)", "3a7fffffff");
        test_cbor(i32::MIN as i64, "NINT(-2147483648)", "3a7fffffff");

        test_cbor(i32::MAX, "UINT(2147483647)", "1a7fffffff");
        test_cbor(i32::MAX as i64, "UINT(2147483647)", "1a7fffffff");

        test_cbor(i64::MIN, "NINT(-9223372036854775808)", "3b7fffffffffffffff");

        test_cbor(i64::MAX, "UINT(9223372036854775807)", "1b7fffffffffffffff");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", 42.cbor()), "42");
        assert_eq!(format!("{}", (-42).cbor()), "-42");
    }
}
