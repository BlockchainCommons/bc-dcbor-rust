use super::{cbor::{CBOREncode, IntoCBOR, CBOR}, varint::{VarIntEncode, MajorType}};

impl CBOREncode for u8 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::Uint)
    }
}

impl IntoCBOR for u8 {
    fn cbor(&self) -> CBOR {
        CBOR::Uint(*self as u64)
    }
}

impl CBOREncode for u16 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::Uint)
    }
}

impl IntoCBOR for u16 {
    fn cbor(&self) -> CBOR {
        CBOR::Uint(*self as u64)
    }
}

impl CBOREncode for u32 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::Uint)
    }
}

impl IntoCBOR for u32 {
    fn cbor(&self) -> CBOR {
        CBOR::Uint(*self as u64)
    }
}

impl CBOREncode for u64 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::Uint)
    }
}

impl IntoCBOR for u64 {
    fn cbor(&self) -> CBOR {
        CBOR::Uint(*self)
    }
}

impl CBOREncode for usize {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(MajorType::Uint)
    }
}

impl IntoCBOR for usize {
    fn cbor(&self) -> CBOR {
        CBOR::Uint(*self as u64)
    }
}

impl CBOREncode for i8 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i16;
            let a = (-b - 1) as u8;
            a.varint_encode(MajorType::Nint)
        } else {
            let a = *self as u8;
            a.varint_encode(MajorType::Uint)
        }
    }
}

impl IntoCBOR for i8 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::Nint(*self as i64)
        } else {
            CBOR::Uint(*self as u64)
        }
    }
}

impl CBOREncode for i16 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i32;
            let a = (-b - 1) as u16;
            a.varint_encode(MajorType::Nint)
        } else {
            let a = *self as u16;
            a.varint_encode(MajorType::Uint)
        }
    }
}

impl IntoCBOR for i16 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::Nint(*self as i64)
        } else {
            CBOR::Uint(*self as u64)
        }
    }
}

impl CBOREncode for i32 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i64;
            let a = (-b - 1) as u32;
            a.varint_encode(MajorType::Nint)
        } else {
            let a = *self as u32;
            a.varint_encode(MajorType::Uint)
        }
    }
}

impl IntoCBOR for i32 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::Nint(*self as i64)
        } else {
            CBOR::Uint(*self as u64)
        }
    }
}

impl CBOREncode for i64 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as u64;
            let a = (-(b as i128) - 1) as u64;
            a.varint_encode(MajorType::Nint)
        } else {
            let a = *self as u64;
            a.varint_encode(MajorType::Uint)
        }
    }
}

impl IntoCBOR for i64 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::Nint(*self as i64)
        } else {
            CBOR::Uint(*self as u64)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cbor::{test_util::test_cbor, cbor::IntoCBOR};

    #[test]
    fn encode_unsigned() {
        test_cbor(0u8, "Uint(0)", "00");
        test_cbor(0u16, "Uint(0)", "00");
        test_cbor(0u32, "Uint(0)", "00");
        test_cbor(0u64, "Uint(0)", "00");
        test_cbor(0 as usize, "Uint(0)", "00");

        test_cbor(1u8, "Uint(1)", "01");
        test_cbor(1u16, "Uint(1)", "01");
        test_cbor(1u32, "Uint(1)", "01");
        test_cbor(1u64, "Uint(1)", "01");
        test_cbor(1 as usize, "Uint(1)", "01");

        test_cbor(23u8, "Uint(23)", "17");
        test_cbor(23u16, "Uint(23)", "17");
        test_cbor(23u32, "Uint(23)", "17");
        test_cbor(23u64, "Uint(23)", "17");
        test_cbor(23 as usize, "Uint(23)", "17");

        test_cbor(24u8, "Uint(24)", "1818");
        test_cbor(24u16, "Uint(24)", "1818");
        test_cbor(24u32, "Uint(24)", "1818");
        test_cbor(24u64, "Uint(24)", "1818");
        test_cbor(24 as usize, "Uint(24)", "1818");

        test_cbor(u8::MAX, "Uint(255)", "18ff");
        test_cbor(u8::MAX as u16, "Uint(255)", "18ff");
        test_cbor(u8::MAX as u32, "Uint(255)", "18ff");
        test_cbor(u8::MAX as u64, "Uint(255)", "18ff");
        test_cbor(u8::MAX as usize, "Uint(255)", "18ff");

        test_cbor(u16::MAX, "Uint(65535)", "19ffff");
        test_cbor(u16::MAX as u32, "Uint(65535)", "19ffff");
        test_cbor(u16::MAX as u64, "Uint(65535)", "19ffff");
        test_cbor(u16::MAX as usize, "Uint(65535)", "19ffff");

        test_cbor(65536u32, "Uint(65536)", "1a00010000");
        test_cbor(65536u64, "Uint(65536)", "1a00010000");
        test_cbor(65536u64 as usize, "Uint(65536)", "1a00010000");

        test_cbor(u32::MAX, "Uint(4294967295)", "1affffffff");
        test_cbor(u32::MAX as u64, "Uint(4294967295)", "1affffffff");
        test_cbor(u32::MAX as usize, "Uint(4294967295)", "1affffffff");

        test_cbor(4294967296u64, "Uint(4294967296)", "1b0000000100000000");

        test_cbor(u64::MAX, "Uint(18446744073709551615)", "1bffffffffffffffff");
        test_cbor(u64::MAX as usize, "Uint(18446744073709551615)", "1bffffffffffffffff");
    }

    #[test]
    fn encode_signed() {
        test_cbor(-1i8, "Nint(-1)", "20");
        test_cbor(-1i16, "Nint(-1)", "20");
        test_cbor(-1i32, "Nint(-1)", "20");
        test_cbor(-1i64, "Nint(-1)", "20");

        test_cbor(-2i8, "Nint(-2)", "21");
        test_cbor(-2i16, "Nint(-2)", "21");
        test_cbor(-2i32, "Nint(-2)", "21");
        test_cbor(-2i64, "Nint(-2)", "21");

        test_cbor(-127i8, "Nint(-127)", "387e");
        test_cbor(-127i16, "Nint(-127)", "387e");
        test_cbor(-127i32, "Nint(-127)", "387e");
        test_cbor(-127i64, "Nint(-127)", "387e");

        test_cbor(i8::MIN, "Nint(-128)", "387f");
        test_cbor(i8::MIN as i16, "Nint(-128)", "387f");
        test_cbor(i8::MIN as i32, "Nint(-128)", "387f");
        test_cbor(i8::MIN as i64, "Nint(-128)", "387f");

        test_cbor(i8::MAX, "Uint(127)", "187f");
        test_cbor(i8::MAX as i16, "Uint(127)", "187f");
        test_cbor(i8::MAX as i32, "Uint(127)", "187f");
        test_cbor(i8::MAX as i64, "Uint(127)", "187f");

        test_cbor(i16::MIN, "Nint(-32768)", "397fff");
        test_cbor(i16::MIN as i32, "Nint(-32768)", "397fff");
        test_cbor(i16::MIN as i64, "Nint(-32768)", "397fff");

        test_cbor(i16::MAX, "Uint(32767)", "197fff");
        test_cbor(i16::MAX as i32, "Uint(32767)", "197fff");
        test_cbor(i16::MAX as i64, "Uint(32767)", "197fff");

        test_cbor(i32::MIN, "Nint(-2147483648)", "3a7fffffff");
        test_cbor(i32::MIN as i64, "Nint(-2147483648)", "3a7fffffff");

        test_cbor(i32::MAX, "Uint(2147483647)", "1a7fffffff");
        test_cbor(i32::MAX as i64, "Uint(2147483647)", "1a7fffffff");

        test_cbor(i64::MIN, "Nint(-9223372036854775808)", "3b7fffffffffffffff");

        test_cbor(i64::MAX, "Uint(9223372036854775807)", "1b7fffffffffffffff");
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", 42.cbor()), "42");
        assert_eq!(format!("{}", (-42).cbor()), "-42");
    }
}
