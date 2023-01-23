use std::str::{from_utf8, Utf8Error};

use super::{cbor::{CBOR, AsCBOR, IntoCBOR}, varint::MajorType, bytes::Bytes, tagged::Tagged, value::Value, map::{CBORMap, CBORMapInsert}};

#[derive(Debug)]
pub enum Error {
    Empty,
    BadHeaderValue(u8),
    Underrun,
    NonCanonicalInt,
    InvalidString(Utf8Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Empty => format!("the data to decode is empty"),
            Error::BadHeaderValue(v) => format!("unsupported value in header ({})", v),
            Error::Underrun => format!("early end of data"),
            Error::NonCanonicalInt => format!("non-canonical int format"),
            Error::InvalidString(err) => format!("invalid string format: {:?}", err),
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
    let header_value = header & 31;
    (major_type, header_value)
}

fn parse_header_varint(data: &[u8]) -> Result<(MajorType, u64, usize), Error> {
    if data.is_empty() {
        return Err(Error::Empty)
    }
    let (major_type, header_value) = parse_header(data[0]);
    let data_remaining = data.len() - 1;
    let (value, varint_len) = match header_value {
        0..=23 => (header_value as u64, 1),
        24 => {
            if data_remaining < 1 { return Err(Error::Underrun); }
            let val = data[1] as u64;
            if val < 24 { return Err(Error::NonCanonicalInt) }
            (val, 2)
        },
        25 => {
            if data_remaining < 2 { return Err(Error::Underrun); }
            let val =
                ((data[1] as u64) << 8) |
                (data[2] as u64);
            if val <= u8::MAX as u64 { return Err(Error::NonCanonicalInt) }
            (val, 3)
        },
        26 => {
            if data_remaining < 4 { return Err(Error::Underrun); }
            let val =
                ((data[1] as u64) << 24) |
                ((data[2] as u64) << 16) |
                ((data[3] as u64) << 8) |
                (data[4] as u64);
            if val <= u16::MAX as u64 { return Err(Error::NonCanonicalInt) }
            (val, 5)
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
            (val, 9)
        },
        v => return Err(Error::BadHeaderValue(v))
    };
    Ok((major_type, value, varint_len))
}

fn parse_bytes<'a>(data: &'a [u8], len: usize) -> Result<&'a [u8], Error> {
    if data.len() < len {
        return Err(Error::Underrun);
    }
    Ok(&data[0..len])
}

pub fn cbor_decode_internal(data: &[u8]) -> Result<(CBOR, usize), Error> {
    if data.is_empty() {
        return Err(Error::Empty)
    }
    let (major_type, value, header_varint_len) = parse_header_varint(&data)?;
    match major_type {
        MajorType::Uint => Ok((CBOR::Uint(value), header_varint_len)),
        MajorType::Nint => Ok((CBOR::Nint(-(value as i64) - 1), header_varint_len)),
        MajorType::Bytes => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let bytes = Bytes::new(buf);
            Ok((bytes.into_cbor(), header_varint_len + data_len))
        },
        MajorType::String => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let string = from_utf8(buf).map_err(|x| Error::InvalidString(x))?;
            Ok((string.as_cbor(), header_varint_len + data_len))
        },
        MajorType::Array => {
            let mut pos = header_varint_len;
            let mut items = Vec::new();
            for _ in 0..value {
                let (item, item_len) = cbor_decode_internal(&data[pos..])?;
                items.push(item);
                pos += item_len;
            }
            Ok((CBOR::Array(items), pos))
        },
        MajorType::Map => {
            let mut pos = header_varint_len;
            let mut map = CBORMap::new();
            for _ in 0..value {
                let (key, key_len) = cbor_decode_internal(&data[pos..])?;
                pos += key_len;
                let (value, value_len) = cbor_decode_internal(&data[pos..])?;
                map.cbor_insert(key, value);
                pos += value_len;
            }
            Ok((CBOR::Map(map), pos))
        },
        MajorType::Tagged => {
            let (item, item_len) = cbor_decode_internal(&data[header_varint_len..])?;
            let tagged = Tagged::new(value, item);
            Ok((tagged.into_cbor(), header_varint_len + item_len))
        },
        MajorType::Value => Ok((Value::new(value).into_cbor(), header_varint_len)),
    }
}

pub fn cbor_decode(data: &[u8]) -> Result<CBOR, Error> {
    let (cbor, _) = cbor_decode_internal(data)?;
    Ok(cbor)
}

#[cfg(test)]
mod test {
    use crate::cbor::{cbor::{EncodeCBOR, AsCBOR}, bytes::Bytes, tagged::Tagged, value::Value, map::{CBORMap, CBORMapInsert}};

    use super::cbor_decode;

    fn test_decode<T>(value: T) where T: AsCBOR {
        let cbor = value.as_cbor();
        let bytes = cbor.encode_cbor();
        //println!("{}", bytes_to_hex(&bytes));
        let decoded_cbor = cbor_decode(&bytes).unwrap();
        assert_eq!(cbor, decoded_cbor);
    }

    #[test]
    fn decode() {
        test_decode(32);
        test_decode(-32);
        test_decode(Bytes::new([0x11, 0x22, 0x33]));
        test_decode("Hello, world!");
        test_decode("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
        test_decode(Tagged::new(32, "Hello".as_cbor()));
        test_decode([1, 2, 3]);
        {
            let mut array: Vec<Box<dyn AsCBOR>> = Vec::new();
            array.push(Box::new(1));
            array.push(Box::new("Hello"));
            array.push(Box::new([1, 2, 3]));
            test_decode(array);
        }
        {
            let mut map = CBORMap::new();
            map.cbor_insert_into(-1, 3);
            map.cbor_insert_into(vec![-1], 7);
            map.cbor_insert_into("z", 4);
            map.cbor_insert_into(10, 1);
            map.cbor_insert_into(false, 8);
            map.cbor_insert_into(100, 2);
            map.cbor_insert_into("aa", 5);
            map.cbor_insert_into(vec![100], 6);
            test_decode(map);
        }
        test_decode(false);
        test_decode(true);
        test_decode(Value::new(32));
    }
}
