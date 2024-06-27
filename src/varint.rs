import_stdlib!();

#[derive(Debug)]
pub enum MajorType {
    Unsigned,
    Negative,
    ByteString,
    Text,
    Array,
    Map,
    Tagged,
    Simple
}

fn type_bits(t: MajorType) -> u8 {
    let b = match t {
        MajorType::Unsigned => 0,
        MajorType::Negative => 1,
        MajorType::ByteString => 2,
        MajorType::Text => 3,
        MajorType::Array => 4,
        MajorType::Map => 5,
        MajorType::Tagged => 6,
        MajorType::Simple => 7,
    };
    b << 5
}

pub trait EncodeVarInt {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8>;
    fn encode_int(&self, major_type: MajorType) -> Vec<u8>;
}

impl EncodeVarInt for u8 {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= 23 {
            vec![*self | type_bits(major_type)]
        } else {
            self.encode_int(major_type)
        }
    }

    fn encode_int(&self, major_type: MajorType) -> Vec<u8> {
        vec![
            0x18 | type_bits(major_type),
            *self
        ]
    }
}

impl EncodeVarInt for u16 {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u8::MAX as u16 {
            (*self as u8).encode_varint(major_type)
        } else {
            self.encode_int(major_type)
        }
    }

    fn encode_int(&self, major_type: MajorType) -> Vec<u8> {
        vec![
            0x19 | type_bits(major_type),
            (*self >> 8) as u8, *self as u8
        ]
    }
}

impl EncodeVarInt for u32 {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u16::MAX as u32 {
            (*self as u16).encode_varint(major_type)
        } else {
            self.encode_int(major_type)
        }
    }

    fn encode_int(&self, major_type: MajorType) -> Vec<u8> {
        vec![
            0x1a | type_bits(major_type),
            (*self >> 24) as u8, (*self >> 16) as u8,
            (*self >> 8) as u8, *self as u8
        ]
    }
}

impl EncodeVarInt for u64 {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8> {
        if *self <= u32::MAX as u64 {
            (*self as u32).encode_varint(major_type)
        } else {
            self.encode_int(major_type)
        }
    }

    fn encode_int(&self, major_type: MajorType) -> Vec<u8> {
        vec![
            0x1b | type_bits(major_type),
            (*self >> 56) as u8, (*self >> 48) as u8,
            (*self >> 40) as u8, (*self >> 32) as u8,
            (*self >> 24) as u8, (*self >> 16) as u8,
            (*self >> 8) as u8, *self as u8
        ]
    }
}

impl EncodeVarInt for usize {
    fn encode_varint(&self, major_type: MajorType) -> Vec<u8> {
        match usize::BITS {
            32 => (*self as u32).encode_varint(major_type),
            64 => (*self as u64).encode_varint(major_type),
            _ => panic!()
        }
    }

    fn encode_int(&self, major_type: MajorType) -> Vec<u8> {
        match usize::BITS {
            32 => (*self as u32).encode_int(major_type),
            64 => (*self as u64).encode_int(major_type),
            _ => panic!()
        }
    }
}
