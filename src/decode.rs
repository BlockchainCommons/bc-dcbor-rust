use std::str::{from_utf8, Utf8Error};

use super::{cbor::{CBOR, CBOREncodable}, varint::MajorType, bytes::Data, Value, Tagged, Map};

/// Decode CBOR binary representation to symbolic representation.
///
/// Returns an error if the data is not well-formed deterministic CBOR.
pub fn decode_cbor(data: &[u8]) -> Result<CBOR, DecodeError> {
    let (cbor, len) = decode_cbor_internal(data)?;
    let remaining = data.len() - len;
    if remaining > 0 {
        return Err(DecodeError::UnusedData(remaining));
    }
    Ok(cbor)
}

/// An error encountered while decoding CBOR.
#[derive(Debug)]
pub enum DecodeError {
    /// Early end of data.
    Underrun,
    /// Unsupported value in CBOR header.
    UnsupportedHeaderValue(u8),
    /// An integer was encoded in non-canonical form.
    NonCanonicalInt,
    /// An invalidly-encoded UTF-8 string was encountered.
    InvalidString(Utf8Error),
    /// The decoded CBOR had extra data at the end.
    UnusedData(usize),
    /// The decoded CBOR map has keys that are not in canonical order.
    MisorderedMapKey,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DecodeError::Underrun => format!("early end of data"),
            DecodeError::UnsupportedHeaderValue(v) => format!("unsupported value in header ({})", v),
            DecodeError::NonCanonicalInt => format!("non-canonical int format"),
            DecodeError::InvalidString(err) => format!("invalid string format: {:?}", err),
            DecodeError::UnusedData(len) => format!("unused data past end: {:?} bytes", len),
            DecodeError::MisorderedMapKey => format!("mis-ordered map key")
        };
        f.write_str(&s)
    }
}

impl std::error::Error for DecodeError {
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnsupportedHeaderValue(l0), Self::UnsupportedHeaderValue(r0)) => l0 == r0,
            (Self::InvalidString(l0), Self::InvalidString(r0)) => l0 == r0,
            (Self::UnusedData(l0), Self::UnusedData(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

fn parse_header(header: u8) -> (MajorType, u8) {
    let major_type = match header >> 5 {
        0 => MajorType::UInt,
        1 => MajorType::NInt,
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

fn parse_header_varint(data: &[u8]) -> Result<(MajorType, u64, usize), DecodeError> {
    if data.is_empty() {
        return Err(DecodeError::Underrun)
    }
    let (major_type, header_value) = parse_header(data[0]);
    let data_remaining = data.len() - 1;
    let (value, varint_len) = match header_value {
        0..=23 => (header_value as u64, 1),
        24 => {
            if data_remaining < 1 { return Err(DecodeError::Underrun); }
            let val = data[1] as u64;
            if val < 24 { return Err(DecodeError::NonCanonicalInt) }
            (val, 2)
        },
        25 => {
            if data_remaining < 2 { return Err(DecodeError::Underrun); }
            let val =
                ((data[1] as u64) << 8) |
                (data[2] as u64);
            if val <= u8::MAX as u64 { return Err(DecodeError::NonCanonicalInt) }
            (val, 3)
        },
        26 => {
            if data_remaining < 4 { return Err(DecodeError::Underrun); }
            let val =
                ((data[1] as u64) << 24) |
                ((data[2] as u64) << 16) |
                ((data[3] as u64) << 8) |
                (data[4] as u64);
            if val <= u16::MAX as u64 { return Err(DecodeError::NonCanonicalInt) }
            (val, 5)
        },
        27 => {
            if data_remaining < 8 { return Err(DecodeError::Underrun); }
            let val =
                ((data[1] as u64) << 56) |
                ((data[2] as u64) << 48) |
                ((data[3] as u64) << 40) |
                ((data[4] as u64) << 32) |
                ((data[5] as u64) << 24) |
                ((data[6] as u64) << 16) |
                ((data[7] as u64) << 8) |
                (data[8] as u64);
            if val <= u32::MAX as u64 { return Err(DecodeError::NonCanonicalInt) }
            (val, 9)
        },
        v => return Err(DecodeError::UnsupportedHeaderValue(v))
    };
    Ok((major_type, value, varint_len))
}

fn parse_bytes<'a>(data: &'a [u8], len: usize) -> Result<&'a [u8], DecodeError> {
    if data.len() < len {
        return Err(DecodeError::Underrun);
    }
    Ok(&data[0..len])
}

fn decode_cbor_internal(data: &[u8]) -> Result<(CBOR, usize), DecodeError> {
    if data.is_empty() {
        return Err(DecodeError::Underrun)
    }
    let (major_type, value, header_varint_len) = parse_header_varint(&data)?;
    match major_type {
        MajorType::UInt => Ok((CBOR::UInt(value), header_varint_len)),
        MajorType::NInt => Ok((CBOR::NInt(-(value as i64) - 1), header_varint_len)),
        MajorType::Bytes => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let bytes = Data::new(buf);
            Ok((bytes.cbor(), header_varint_len + data_len))
        },
        MajorType::String => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let string = from_utf8(buf).map_err(|x| DecodeError::InvalidString(x))?;
            Ok((string.cbor(), header_varint_len + data_len))
        },
        MajorType::Array => {
            let mut pos = header_varint_len;
            let mut items = Vec::new();
            for _ in 0..value {
                let (item, item_len) = decode_cbor_internal(&data[pos..])?;
                items.push(item);
                pos += item_len;
            }
            Ok((items.cbor(), pos))
        },
        MajorType::Map => {
            let mut pos = header_varint_len;
            let mut map = Map::new();
            for _ in 0..value {
                let (key, key_len) = decode_cbor_internal(&data[pos..])?;
                pos += key_len;
                let (value, value_len) = decode_cbor_internal(&data[pos..])?;
                pos += value_len;
                if !map.insert_next(key, value) {
                    return Err(DecodeError::MisorderedMapKey);
                }
            }
            Ok((map.cbor(), pos))
        },
        MajorType::Tagged => {
            let (item, item_len) = decode_cbor_internal(&data[header_varint_len..])?;
            let tagged = Tagged::new(value, item);
            Ok((tagged.cbor(), header_varint_len + item_len))
        },
        MajorType::Value => Ok((Value::new(value).cbor(), header_varint_len)),
    }
}
