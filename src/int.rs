use crate::{cbor::CBOREncode, varint::VarIntEncode, cbor::{IntoCBOR, CBOR}};

impl CBOREncode for u8 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(0)
    }
}

impl IntoCBOR for u8 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u16 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(0)
    }
}

impl IntoCBOR for u16 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u32 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(0)
    }
}

impl IntoCBOR for u32 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self as u64)
    }
}

impl CBOREncode for u64 {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(0)
    }
}

impl IntoCBOR for u64 {
    fn cbor(&self) -> CBOR {
        CBOR::UINT(*self)
    }
}

impl CBOREncode for usize {
    fn cbor_encode(&self) -> Vec<u8> {
        self.varint_encode(0)
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
            a.varint_encode(1)
        } else {
            let a = *self as u8;
            a.varint_encode(0)
        }
    }
}

impl IntoCBOR for i8 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i128)
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
            a.varint_encode(1)
        } else {
            let a = *self as u16;
            a.varint_encode(0)
        }
    }
}

impl IntoCBOR for i16 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i128)
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
            a.varint_encode(1)
        } else {
            let a = *self as u32;
            a.varint_encode(0)
        }
    }
}

impl IntoCBOR for i32 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i128)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

impl CBOREncode for i64 {
    fn cbor_encode(&self) -> Vec<u8> {
        if *self < 0 {
            let b = *self as i128;
            let a = (-b - 1) as u64;
            a.varint_encode(1)
        } else {
            let a = *self as u64;
            a.varint_encode(0)
        }
    }
}

impl IntoCBOR for i64 {
    fn cbor(&self) -> CBOR {
        if *self < 0 {
            CBOR::NINT(*self as i128)
        } else {
            CBOR::UINT(*self as u64)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_util::test_encode, cbor::IntoCBOR, bytes::Bytes};

    #[test]
    fn encode_u8() {
        test_encode(0u8, "00");
        test_encode(1u8, "01");
        test_encode(23u8, "17");
        test_encode(24u8, "1818");
        test_encode(u8::MAX, "18ff");
        // let mut buf: Vec<u8> = Vec::new();
        // buf.push(0x85);
        // 0u8.cbor_append(&mut buf);
        // 1u8.cbor_append(&mut buf);
        // 23u8.cbor_append(&mut buf);
        // 24u8.cbor_append(&mut buf);
        // 255u8.cbor_append(&mut buf);
        // assert_eq!(bytes_to_hex(&buf), "85000117181818ff");
    }

    #[test]
    fn encode_u16() {
        test_encode(0u16, "00");
        test_encode(1u16, "01");
        test_encode(23u16, "17");
        test_encode(24u16, "1818");
        test_encode(255u16, "18ff");
        test_encode(256u16, "190100");
        test_encode(u16::MAX, "19ffff");
    }

    #[test]
    fn encode_u32() {
        test_encode(0u32, "00");
        test_encode(1u32, "01");
        test_encode(23u32, "17");
        test_encode(24u32, "1818");
        test_encode(255u32, "18ff");
        test_encode(256u32, "190100");
        test_encode(65535u32, "19ffff");
        test_encode(65536u32, "1a00010000");
        test_encode(u32::MAX, "1affffffff");
    }

    #[test]
    fn encode_u64() {
        test_encode(0u64, "00");
        test_encode(1u64, "01");
        test_encode(23u64, "17");
        test_encode(24u64, "1818");
        test_encode(255u64, "18ff");
        test_encode(256u64, "190100");
        test_encode(65535u64, "19ffff");
        test_encode(65536u64, "1a00010000");
        test_encode(4294967295u64, "1affffffff");
        test_encode(4294967296u64, "1b0000000100000000");
        test_encode(u64::MAX, "1bffffffffffffffff");
    }

    #[test]
    fn encode_i8() {
        test_encode(0i8, "00");
        test_encode(1i8, "01");
        test_encode(23i8, "17");
        test_encode(i8::MAX, "187f");
        test_encode(-1i8, "20");
        test_encode(-2i8, "21");
        test_encode(-127i8, "387e");
        test_encode(i8::MIN, "387f");
    }

    #[test]
    fn encode_i16() {
        test_encode(0i16, "00");
        test_encode(1i16, "01");
        test_encode(23i16, "17");
        test_encode(127i16, "187f");
        test_encode(-1i16, "20");
        test_encode(-2i16, "21");
        test_encode(-127i16, "387e");
        test_encode(-128i16, "387f");
        test_encode(i16::MIN, "397fff");
        test_encode(i16::MAX, "197fff");
    }

    #[test]
    fn encode_i32() {
        test_encode(0i32, "00");
        test_encode(1i32, "01");
        test_encode(23i32, "17");
        test_encode(127i32, "187f");
        test_encode(-1i32, "20");
        test_encode(-2i32, "21");
        test_encode(-127i32, "387e");
        test_encode(-128i32, "387f");
        test_encode(-32768i32, "397fff");
        test_encode(32767i32, "197fff");
        test_encode(i32::MIN, "3a7fffffff");
        test_encode(i32::MAX, "1a7fffffff");
    }

    #[test]
    fn encode_i64() {
        test_encode(0i64, "00");
        test_encode(1i64, "01");
        test_encode(23i64, "17");
        test_encode(127i64, "187f");
        test_encode(-1i64, "20");
        test_encode(-2i64, "21");
        test_encode(-127i64, "387e");
        test_encode(-128i64, "387f");
        test_encode(-32768i64, "397fff");
        test_encode(32767i64, "197fff");
        test_encode(-2147483648i64, "3a7fffffff");
        test_encode(2147483647i64, "1a7fffffff");
        test_encode(i64::MIN, "3b7fffffffffffffff");
        test_encode(i64::MAX, "1b7fffffffffffffff");
    }

    #[test]
    fn into_cbor() {
        let a = Bytes::from_hex("112233");
        let cbor = a.cbor();
        println!("{:?}", cbor);
    }
}
