use std::collections::{BTreeMap, btree_map::Values as BTreeMapValues};

use crate::{cbor_encodable::CBOREncodable, CBORError, CBORDecodable};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}, hex::data_to_hex};

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

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets an iterator over the entries of the CBOR map, sorted by key.
    ///
    /// Key sorting order is lexicographic by the key's binary-encoded CBOR.
    pub fn iter<'a>(&'a self) -> MapIter<'a> {
        MapIter::new(self.0.values())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: CBOR, value: CBOR) -> Result<(), CBORError> {
        if value == CBOR::NULL {
            return Err(CBORError::NullMapValue)
        }
        self.0.insert(MapKey::new(key.cbor_data()), MapValue::new(key, value));
        Ok(())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_into<K, V>(&mut self, key: K, value: V) -> Result<(), CBORError>
    where
        K: CBOREncodable, V: CBOREncodable
    {
        self.insert(key.cbor(), value.cbor())
    }

    pub(crate) fn insert_next(&mut self, key: CBOR, value: CBOR) -> Result<(), CBORError> {
        match self.0.last_key_value() {
            None => {
                self.insert(key, value)
            },
            Some(entry) => {
                let new_key = MapKey::new(key.cbor_data());
                if self.0.contains_key(&new_key) {
                    return Err(CBORError::DuplicateMapKey)
                }
                if entry.0 >= &new_key {
                    return Err(CBORError::MisorderedMapKey)
                }
                if value == CBOR::NULL {
                    return Err(CBORError::NullMapValue)
                }
                self.0.insert(new_key, MapValue::new(key, value));
                Ok(())
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

    fn cbor_data(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.0.iter().map(|x| (x.0.0.to_owned(), x.1.value.cbor_data())).collect();
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

/// An iterator over the entries of a CBOR map.
///
/// This iterator always returns the entries in lexicographic order by the key's
/// binary-encoded CBOR value.
pub struct MapIter<'a>(BTreeMapValues<'a, MapKey, MapValue>);

impl<'a> MapIter<'a> {
    fn new(values: BTreeMapValues<'a, MapKey, MapValue>) -> MapIter<'a> {
        MapIter(values)
    }
}

impl<'a> Iterator for MapIter<'a> {
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
        f.write_fmt(format_args!("0x{}", data_to_hex(&self.0)))
    }
}

/// Convert a container to a CBOR Map where the container's items are
/// pairs of CBOREncodable values.
impl<T, K, V> From<T> for Map where T: IntoIterator<Item=(K, V)>, K: CBOREncodable, V: CBOREncodable {
    fn from(container: T) -> Self {
        let mut map = Map::new();
        for (k, v) in container {
            map.insert(k.cbor(), v.cbor()).unwrap();
        }
        map
    }
}

macro_rules! impl_container {
    ($type: ty, $constraint: ty) => {
        impl<K, V> CBOREncodable for $type where K: CBOREncodable, V: CBOREncodable {
            fn cbor(&self) -> CBOR {
                CBOR::Map(Map::from(self.iter()))
            }
        }

        impl<K, V> From<$type> for CBOR where K: CBOREncodable, V: CBOREncodable {
            fn from(container: $type) -> Self {
                CBOR::Map(Map::from(container.iter()))
            }
        }

        impl<K, V> TryInto<$type> for CBOR where K: CBORDecodable + std::cmp::Eq + ($constraint), V: CBORDecodable {
            type Error = CBORError;

            fn try_into(self) -> Result<$type, Self::Error> {
                match self {
                    CBOR::Map(map) => {
                        let mut container = <$type>::new();
                        for (k, v) in map.iter() {
                            container.insert(*K::from_cbor(k)?, *V::from_cbor(v)?);
                        }
                        Ok(container)
                    },
                    _ => Err(CBORError::WrongType)
                }
            }
        }
    }
}

impl_container!(std::collections::HashMap<K, V>, std::hash::Hash);
impl_container!(std::collections::BTreeMap<K, V>, std::cmp::Ord);
