import_stdlib!();

use crate::{CBOR, CBOREncodable, CBORError, CBORDecodable, CBORCase};

use super::varint::{EncodeVarInt, MajorType};

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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets an iterator over the entries of the CBOR map, sorted by key.
    ///
    /// Key sorting order is lexicographic by the key's binary-encoded CBOR.
    pub fn iter(&self) -> MapIter<'_> {
        MapIter::new(self.0.values())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: impl Into<CBOR>, value: impl Into<CBOR>) {
        let key = key.into();
        let value = value.into();
        self.0.insert(MapKey::new(key.cbor_data()), MapValue::new(key, value));
    }

    pub(crate) fn insert_next(&mut self, key: CBOR, value: CBOR) -> Result<(), CBORError> {
        match self.0.last_key_value() {
            None => {
                self.insert(key, value);
                Ok(())
            },
            Some(entry) => {
                let new_key = MapKey::new(key.cbor_data());
                if self.0.contains_key(&new_key) {
                    return Err(CBORError::DuplicateMapKey)
                }
                if entry.0 >= &new_key {
                    return Err(CBORError::MisorderedMapKey)
                }
                self.0.insert(new_key, MapValue::new(key, value));
                Ok(())
            }
        }
    }

    /// Get a value from the map, given a key.
    ///
    /// Returns `Some` if the key is present in the map, `None` otherwise.
    pub fn get<K, V>(&self, key: K) -> Option<V>
    where
        K: CBOREncodable, V: CBORDecodable
    {
        match self.0.get(&MapKey::new(key.cbor_data())) {
            Some(value) => V::from_cbor(&value.value).ok(),
            None => None
        }
    }

    /// Get a value from the map, given a key.
    ///
    /// Returns `Ok` if the key is present in the map, `Err` otherwise.
    pub fn extract<K, V>(&self, key: K) -> Result<V, CBORError>
    where
        K: CBOREncodable, V: CBORDecodable
    {
        match self.get(key) {
            Some(value) => Ok(value),
            None => Err(CBORError::MissingMapKey)
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
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
        CBORCase::Map(self.clone()).into()
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

impl From<Map> for CBOR {
    fn from(value: Map) -> Self {
        value.cbor()
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl fmt::Debug for MapValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MapKey {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Debug for MapKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("0x{}", hex::encode(&self.0)))
    }
}

/// Convert a container to a CBOR Map where the container's items are
/// pairs of CBOREncodable values.
impl<T, K, V> From<T> for Map where T: IntoIterator<Item=(K, V)>, K: CBOREncodable, V: CBOREncodable {
    fn from(container: T) -> Self {
        let mut map = Map::new();
        for (k, v) in container {
            map.insert(k.cbor(), v.cbor());
        }
        map
    }
}

impl<K, V> CBOREncodable for HashMap<K, V> where K: CBOREncodable, V: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBORCase::Map(Map::from(self.iter())).into()
    }
}

impl<K, V> From<HashMap<K, V>> for CBOR where K: CBOREncodable, V: CBOREncodable {
    fn from(container: HashMap<K, V>) -> Self {
        CBORCase::Map(Map::from(container.iter())).into()
    }
}

impl<K, V> TryFrom<CBOR> for HashMap<K, V> where K: CBORDecodable + cmp::Eq + (hash::Hash) + Clone, V: CBORDecodable + Clone {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor.case() {
            CBORCase::Map(map) => {
                let mut container = <HashMap<K, V>>::new();
                for (k, v) in map.iter() {
                    container.insert(K::from_cbor(k)?.clone(), V::from_cbor(v)?.clone());
                }
                Ok(container)
            },
            _ => Err(anyhow::Error::msg(CBORError::WrongType))
        }
    }
}

impl<K, V> CBOREncodable for BTreeMap<K, V> where K: CBOREncodable, V: CBOREncodable {
    fn cbor(&self) -> CBOR {
        CBORCase::Map(Map::from(self.iter())).into()
    }
}

impl<K, V> From<BTreeMap<K, V>> for CBOR where K: CBOREncodable, V: CBOREncodable {
    fn from(container: BTreeMap<K, V>) -> Self {
        CBORCase::Map(Map::from(container.iter())).into()
    }
}

impl<K, V> TryFrom<CBOR> for BTreeMap<K, V> where K: CBORDecodable + cmp::Eq + (cmp::Ord) + Clone, V: CBORDecodable + Clone {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor.case() {
            CBORCase::Map(map) => {
                let mut container = <BTreeMap<K, V>>::new();
                for (k, v) in map.iter() {
                    container.insert(K::from_cbor(k)?.clone(), V::from_cbor(v)?.clone());
                }
                Ok(container)
            },
            _ => Err(anyhow::Error::msg(Box::new(CBORError::WrongType)))
        }
    }
}
