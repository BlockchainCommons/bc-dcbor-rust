#[derive(Debug)]
pub enum MajorType {
    Uint,
    Nint,
    Bytes,
    String,
    Array,
    Map,
    Tagged,
    Value
}

fn type_bits(t: MajorType) -> u8 {
    let b = match t {
        MajorType::Uint => 0,
        MajorType::Nint => 1,
        MajorType::Bytes => 2,
        MajorType::String => 3,
        MajorType::Array => 4,
        MajorType::Map => 5,
        MajorType::Tagged => 6,
        MajorType::Value => 7,
    };
    b << 5
}

pub trait EncodeVarInt {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8>;
}

impl EncodeVarInt for u8 {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= 23 {
            vec![*self | type_bits(major_type)]
        } else {
            vec![
                0x18 | type_bits(major_type),
                *self
            ]
        }
    }
}

impl EncodeVarInt for u16 {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u8::MAX as u16 {
            (*self as u8).varint_encode(major_type)
        } else {
            vec![
                0x19 | type_bits(major_type),
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl EncodeVarInt for u32 {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u16::MAX as u32 {
            (*self as u16).varint_encode(major_type)
        } else {
            vec![
                0x1a | type_bits(major_type),
                (*self >> 24) as u8, (*self >> 16) as u8,
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl EncodeVarInt for u64 {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u32::MAX as u64 {
            (*self as u32).varint_encode(major_type)
        } else {
            vec![
                0x1b | type_bits(major_type),
                (*self >> 56) as u8, (*self >> 48) as u8,
                (*self >> 40) as u8, (*self >> 32) as u8,
                (*self >> 24) as u8, (*self >> 16) as u8,
                (*self >> 8) as u8, *self as u8
            ]
        }
    }
}

impl EncodeVarInt for usize {
    fn varint_encode(&self, major_type: MajorType) -> Vec<u8> {
        match usize::BITS {
            32 => (*self as u32).varint_encode(major_type),
            64 => (*self as u64).varint_encode(major_type),
            _ => panic!()
        }
    }
}
