use std::collections::BTreeMap;
use std::collections::btree_map::Values as BTreeMapValues;

use crate::util::hex::bytes_to_hex;

use super::{cbor::{CBOR, CBOREncodable}, varint::{EncodeVarInt, MajorType}};


#[derive(Clone)]
struct CBORMapValue {
    key: CBOR,
    value: CBOR,
}

impl CBORMapValue {
    fn new(key: CBOR, value: CBOR) -> CBORMapValue {
        CBORMapValue { key, value }
    }
}

impl PartialEq for CBORMapValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl std::fmt::Debug for CBORMapValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?}, {:?})", self.key, self.value))
    }
}

#[derive(Clone)]
struct CBORMapKey(Vec<u8>);

impl CBORMapKey {
    fn new(key_data: Vec<u8>) -> CBORMapKey {
        CBORMapKey(key_data)
    }
}

impl PartialEq for CBORMapKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CBORMapKey {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for CBORMapKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CBORMapKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for CBORMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("0x{}", bytes_to_hex(&self.0)))
    }
}

/// An iterator over a CBOR map.
pub struct Iter<'a>(BTreeMapValues<'a, CBORMapKey, CBORMapValue>);

impl<'a> Iter<'a> {
    fn new(values: BTreeMapValues<'a, CBORMapKey, CBORMapValue>) -> Iter<'a> {
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

/// A CBOR map. Keys are kept sorted by encoded CBOR form in
/// ascending lexicographic order.
#[derive(Clone)]
pub struct Map(BTreeMap<CBORMapKey, CBORMapValue>);

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
        self.0.insert(CBORMapKey::new(k.encode_cbor()), CBORMapValue::new(k, v));
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
                let new_key = CBORMapKey::new(k.encode_cbor());
                let entry_key = entry.key();
                if entry_key >= &new_key {
                    return false
                }
                self.0.insert(new_key, CBORMapValue::new(k, v));
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
