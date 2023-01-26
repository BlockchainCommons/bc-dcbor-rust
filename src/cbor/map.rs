use std::collections::BTreeMap;
use std::collections::btree_map::Values as BTreeMapValues;

use crate::util::hex::bytes_to_hex;

use super::{cbor::{CBOR, CBOREncodable}, varint::{EncodeVarInt, MajorType}};

/// A CBOR map.
///
/// Keys are kept sorted by encoded CBOR form in ascending lexicographic order.
#[derive(Clone)]
pub struct Map(BTreeMap<MapKey, MapValue>);

impl Map {
    /// Makes a new, empty CBOR `Map`.
    pub fn new() -> Map {
        Map(BTreeMap::new())
    }

    /// Gets an iterator over the entries of the CBOR map, sorted by key.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter::new(self.0.values())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, k: CBOR, v: CBOR) {
        self.0.insert(MapKey::new(k.encode_cbor()), MapValue::new(k, v));
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_into<K, V>(&mut self, k: K, v: V) where K: CBOREncodable, V: CBOREncodable {
        self.insert(k.cbor(), v.cbor());
    }

    pub(crate) fn insert_next(&mut self, k: CBOR, v: CBOR) -> bool {
        match self.0.last_entry() {
            None => {
                self.insert(k, v);
                true
            },
            Some(entry) => {
                let new_key = MapKey::new(k.encode_cbor());
                let entry_key = entry.key();
                if entry_key >= &new_key {
                    return false
                }
                self.0.insert(new_key, MapValue::new(k, v));
                true
            }
        }
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Map {
    fn assert_receiver_is_total_eq(&self) {}
}

impl CBOREncodable for Map {
    fn cbor(&self) -> CBOR {
        CBOR::Map(self.clone())
    }

    fn encode_cbor(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.0.iter().map(|x| (x.0.0.to_owned(), x.1.value.encode_cbor())).collect();
        let mut buf = pairs.len().encode_varint(MajorType::Map);
        for pair in pairs {
            buf.extend(pair.0);
            buf.extend(pair.1);
        }
        buf
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

/// An iterator over a CBOR map.
pub struct Iter<'a>(BTreeMapValues<'a, MapKey, MapValue>);

impl<'a> Iter<'a> {
    fn new(values: BTreeMapValues<'a, MapKey, MapValue>) -> Iter<'a> {
        Iter(values)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a CBOR, &'a CBOR);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.0.next()?;
        Some((&entry.key, &entry.value))
    }
}

#[derive(Clone)]
struct MapValue {
    key: CBOR,
    value: CBOR,
}

impl MapValue {
    fn new(key: CBOR, value: CBOR) -> MapValue {
        MapValue { key, value }
    }
}

impl PartialEq for MapValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl std::fmt::Debug for MapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?}, {:?})", self.key, self.value))
    }
}

#[derive(Clone)]
struct MapKey(Vec<u8>);

impl MapKey {
    fn new(key_data: Vec<u8>) -> MapKey {
        MapKey(key_data)
    }
}

impl PartialEq for MapKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MapKey {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for MapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for MapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for MapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("0x{}", bytes_to_hex(&self.0)))
    }
}

#[cfg(test)]
mod tests {
    use decode::DecodeError;

    use crate::{cbor::{test_util::test_cbor, decode}, util::hex::hex_to_bytes};

    use super::Map;

    #[test]
    fn encode() {
        let mut m = Map::new();
        m.insert_into(-1, 3);
        m.insert_into(vec![-1], 7);
        m.insert_into("z", 4);
        m.insert_into(10, 1);
        m.insert_into(false, 8);
        m.insert_into(100, 2);
        m.insert_into("aa", 5);
        m.insert_into(vec![100], 6);
        test_cbor(m,
            r#"Map({0x0a: (UInt(10), UInt(1)), 0x1864: (UInt(100), UInt(2)), 0x20: (NInt(-1), UInt(3)), 0x617a: (String("z"), UInt(4)), 0x626161: (String("aa"), UInt(5)), 0x811864: (Array([UInt(100)]), UInt(6)), 0x8120: (Array([NInt(-1)]), UInt(7)), 0xf4: (Value(false), UInt(8))})"#,
            r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#,
            "a80a011864022003617a046261610581186406812007f408");
    }

    #[test]
    fn misordered() {
        let cbor = decode(&hex_to_bytes("a2026141016142"));
        assert_eq!(cbor, Err(DecodeError::MisorderedMapKey));
    }
}
