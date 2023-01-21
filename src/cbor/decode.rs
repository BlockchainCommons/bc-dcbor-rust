use super::{cbor::CBOR, varint::MajorType};

#[derive(Debug)]
pub enum Error {
    Empty,
    BadHeaderValue,
    Underrun,
    NonCanonicalInt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Empty => format!("the data to decode is empty"),
            Error::BadHeaderValue => format!("unsupported value in header"),
            Error::Underrun => format!("early end of data"),
            Error::NonCanonicalInt => format!("non-canonical int format"),
        };
        f.write_str(&s)
    }
}

impl std::error::Error for Error {
}

fn parse_header(header: u8) -> (MajorType, u8) {
    let major_type = match header >> 5 {
        0 => MajorType::Uint,
        1 => MajorType::Nint,
        2 => MajorType::Bytes,
        3 => MajorType::String,
        4 => MajorType::Array,
        5 => MajorType::Map,
        6 => MajorType::Tagged,
        7 => MajorType::Value,
        _ => panic!()
    };
    let header_value = header & 32;
    (major_type, header_value)
}

fn parse_varint(data: &[u8]) -> Result<(MajorType, u64, usize), Error> {
    if data.is_empty() {
        return Err(Error::Empty)
    }
    let (major_type, header_value) = parse_header(data[0]);
    let data_remaining = data.len() - 1;
    let (value, len) = match header_value {
        0..=23 => (header_value as u64, 1),
        24 => {
            if data_remaining < 1 { return Err(Error::Underrun); }
            let val = data[1] as u64;
            if val < 24 { return Err(Error::NonCanonicalInt) }
            (val, 1)
        },
        25 => {
            if data_remaining < 2 { return Err(Error::Underrun); }
            let val =
                ((data[1] as u64) << 8) |
                (data[2] as u64);
            if val <= u8::MAX as u64 { return Err(Error::NonCanonicalInt) }
            (val, 2)
        },
        26 => {
            if data_remaining < 4 { return Err(Error::Underrun); }
            let val =
                ((data[1] as u64) << 24) |
                ((data[2] as u64) << 16) |
                ((data[3] as u64) << 8) |
                (data[4] as u64);
            if val <= u16::MAX as u64 { return Err(Error::NonCanonicalInt) }
            (val, 4)
        },
        27 => {
            if data_remaining < 8 { return Err(Error::Underrun); }
            let val =
                ((data[1] as u64) << 56) |
                ((data[2] as u64) << 48) |
                ((data[3] as u64) << 40) |
                ((data[4] as u64) << 32) |
                ((data[5] as u64) << 24) |
                ((data[6] as u64) << 16) |
                ((data[7] as u64) << 8) |
                (data[8] as u64);
            if val <= u32::MAX as u64 { return Err(Error::NonCanonicalInt) }
            (val, 8)
        },
        _ => todo!()
    };
    Ok((major_type, value, len))
}

// pub fn decode(data: &[u8]) -> Result<CBOR, Error> {
//     if data.is_empty() {
//         return Err(Error::Empty)
//     }
//     let header = data[0];
//     let (major_type, header_value) = parse_header(header);
//     let value = if header_value <= 23 {
//         value
//     } else {
//         match header_value {
//             24 => Ok(CBOR::Uint(3)),
//             25 => Ok(CBOR::Uint(3)),
//             26 => Ok(CBOR::Uint(3)),
//             27 => Ok(CBOR::Uint(3)),
//             _ => todo!(),
//         }
//     }

//     let result: Result<CBOR, Error> = match major_type {
//         MajorType::Uint => todo!(),
//         MajorType::Nint => todo!(),
//         MajorType::Bytes => todo!(),
//         MajorType::String => todo!(),
//         MajorType::Array => todo!(),
//         MajorType::Map => todo!(),
//         MajorType::Tagged => todo!(),
//         MajorType::Value => todo!(),
//     };
//     result
// }
