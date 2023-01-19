pub trait VarIntEncode {
    fn varint_encode(&self, major_type: u8) -> Vec<u8>;
}

impl VarIntEncode for u8 {
    fn varint_encode(&self, major_type: u8) -> Vec<u8> {
        let type_bits = major_type << 5;
        if *self <= 23 {
            vec![*self | type_bits]
        } else {
            vec![
                0x18 | type_bits,
                *self
            ]
        }
    }
}

impl VarIntEncode for u16 {
    fn varint_encode(&self, major_type: u8) -> Vec<u8> {
        if *self <= u8::MAX as u16 {
            (*self as u8).varint_encode(major_type)
        } else {
            let type_bits = major_type << 5;
            vec![
                0x19 | type_bits,
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl VarIntEncode for u32 {
    fn varint_encode(&self, major_type: u8) -> Vec<u8> {
        if *self <= u16::MAX as u32 {
            (*self as u16).varint_encode(major_type)
        } else {
            let type_bits = major_type << 5;
            vec![
                0x1a | type_bits,
                (*self >> 24) as u8, (*self >> 16) as u8,
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl VarIntEncode for u64 {
    fn varint_encode(&self, major_type: u8) -> Vec<u8> {
        if *self <= u32::MAX as u64 {
            (*self as u32).varint_encode(major_type)
        } else {
            let type_bits = major_type << 5;
            vec![
                0x1b | type_bits,
                (*self >> 56) as u8, (*self >> 48) as u8,
                (*self >> 40) as u8, (*self >> 32) as u8,
                (*self >> 24) as u8, (*self >> 16) as u8,
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl VarIntEncode for usize {
    fn varint_encode(&self, major_type: u8) -> Vec<u8> {
        match usize::BITS {
            32 => (*self as u32).varint_encode(major_type),
            64 => (*self as u64).varint_encode(major_type),
            _ => panic!()
        }
    }
}
