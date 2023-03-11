use std::str::from_utf8;

use half::f16;

use crate::{cbor_error::CBORError, cbor_encodable::CBOREncodable, tag::Tag, float::{validate_canonical_f16, validate_canonical_f32, validate_canonical_f64}};

use super::{cbor::CBOR, varint::MajorType, bytes::Bytes, Simple, Tagged, Map};

/// Decode CBOR binary representation to symbolic representation.
///
/// Returns an error if the data is not well-formed deterministic CBOR.
pub fn decode_cbor(data: &[u8]) -> Result<CBOR, CBORError> {
    let (cbor, len) = decode_cbor_internal(data)?;
    let remaining = data.len() - len;
    if remaining > 0 {
        return Err(CBORError::UnusedData(remaining));
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

fn parse_header_varint(data: &[u8]) -> Result<(MajorType, u64, usize), CBORError> {
    if data.is_empty() {
        return Err(CBORError::Underrun)
    }
    let (major_type, header_value) = parse_header(data[0]);
    let data_remaining = data.len() - 1;
    let (value, varint_len) = match header_value {
        0..=23 => (header_value as u64, 1),
        24 => {
            if data_remaining < 1 { return Err(CBORError::Underrun); }
            let val = data[1] as u64;
            if val < 24 { return Err(CBORError::NonCanonicalNumeric) }
            (val, 2)
        },
        25 => {
            if data_remaining < 2 { return Err(CBORError::Underrun); }
            let val =
                ((data[1] as u64) << 8) |
                (data[2] as u64);
            if val <= u8::MAX as u64 { return Err(CBORError::NonCanonicalNumeric) }
            (val, 3)
        },
        26 => {
            if data_remaining < 4 { return Err(CBORError::Underrun); }
            let val =
                ((data[1] as u64) << 24) |
                ((data[2] as u64) << 16) |
                ((data[3] as u64) << 8) |
                (data[4] as u64);
            if val <= u16::MAX as u64 { return Err(CBORError::NonCanonicalNumeric) }
            (val, 5)
        },
        27 => {
            if data_remaining < 8 { return Err(CBORError::Underrun); }
            let val =
                ((data[1] as u64) << 56) |
                ((data[2] as u64) << 48) |
                ((data[3] as u64) << 40) |
                ((data[4] as u64) << 32) |
                ((data[5] as u64) << 24) |
                ((data[6] as u64) << 16) |
                ((data[7] as u64) << 8) |
                (data[8] as u64);
            if val <= u32::MAX as u64 { return Err(CBORError::NonCanonicalNumeric) }
            (val, 9)
        },
        v => return Err(CBORError::UnsupportedHeaderValue(v))
    };
    Ok((major_type, value, varint_len))
}

fn parse_bytes<'a>(data: &'a [u8], len: usize) -> Result<&'a [u8], CBORError> {
    if data.len() < len {
        return Err(CBORError::Underrun);
    }
    Ok(&data[0..len])
}

fn decode_cbor_internal(data: &[u8]) -> Result<(CBOR, usize), CBORError> {
    if data.is_empty() {
        return Err(CBORError::Underrun)
    }
    let (major_type, value, header_varint_len) = parse_header_varint(&data)?;
    match major_type {
        MajorType::Unsigned => Ok((CBOR::Unsigned(value), header_varint_len)),
        MajorType::Negative => Ok((CBOR::Negative(-(value as i64) - 1), header_varint_len)),
        MajorType::Bytes => {
            let data_len = value as usize;
            let bytes: Bytes = parse_bytes(&data[header_varint_len..], data_len)?.into();
            Ok((bytes.cbor(), header_varint_len + data_len))
        },
        MajorType::Text => {
            let data_len = value as usize;
            let buf = parse_bytes(&data[header_varint_len..], data_len)?;
            let string = from_utf8(buf).map_err(|x| CBORError::InvalidString(x))?;
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
                map.insert_next(key, value)?;
            }
            Ok((map.cbor(), pos))
        },
        MajorType::Tagged => {
            let (item, item_len) = decode_cbor_internal(&data[header_varint_len..])?;
            let tagged = Tagged::new(Tag::new_opt(value, None), item);
            Ok((tagged.cbor(), header_varint_len + item_len))
        },
        MajorType::Simple => {
            match header_varint_len {
                3 => {
                    let f = f16::from_bits(value as u16);
                    validate_canonical_f16(f)?;
                    Ok((f.cbor(), header_varint_len))
                },
                5 => {
                    let f = f32::from_bits(value as u32);
                    validate_canonical_f32(f)?;
                    Ok((f.cbor(), header_varint_len))
                },
                9 => {
                    let f = f64::from_bits(value);
                    validate_canonical_f64(f)?;
                    Ok((f.cbor(), header_varint_len))
                },
                _ => {
                    match value {
                        20 => Ok((CBOR::FALSE, header_varint_len)),
                        21 => Ok((CBOR::TRUE, header_varint_len)),
                        22 => Ok((CBOR::NULL, header_varint_len)),
                        v => Ok((Simple::new(v).cbor(), header_varint_len)),
                    }
                }
            }
        }
    }
}
