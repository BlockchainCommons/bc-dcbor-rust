use std::str::from_utf8;

use crate::{decode_error::DecodeError, cbor_encodable::CBOREncodable, tag::Tag};

use super::{cbor::CBOR, varint::MajorType, data::Data, Simple, Tagged, Map};

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

fn parse_header(header: u8) -> (MajorType, u8) {
    let major_type = match header >> 5 {
        0 => MajorType::Unsigned,
        1 => MajorType::Negative,
        2 => MajorType::Bytes,
        3 => MajorType::Text,
        4 => MajorType::Array,
        5 => MajorType::Map,
        6 => MajorType::Tagged,
        7 => MajorType::Simple,
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
        MajorType::Unsigned => Ok((CBOR::Unsigned(value), header_varint_len)),
        MajorType::Negative => Ok((CBOR::Negative(-(value as i64) - 1), header_varint_len)),
        MajorType::Bytes => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let bytes = Data::from_data(buf);
            Ok((bytes.cbor(), header_varint_len + data_len))
        },
        MajorType::Text => {
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
            let tagged = Tagged::new(Tag::new_opt(value, None), item);
            Ok((tagged.cbor(), header_varint_len + item_len))
        },
        MajorType::Simple => Ok((Simple::new(value).cbor(), header_varint_len)),
    }
}
