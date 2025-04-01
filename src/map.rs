import_stdlib!();

use anyhow::{bail, Error, Result};

use crate::{CBOR, CBORError, CBORCase};

use super::varint::{EncodeVarInt, MajorType};

/// # Map Support in dCBOR
/// 
/// A deterministic CBOR map implementation that ensures maps with the same content always 
/// produce identical binary encodings, regardless of insertion order.
/// 
/// ## Deterministic Map Representation
/// 
/// The `Map` type follows strict deterministic encoding rules as specified by dCBOR:
/// 
/// - Map keys are always sorted in lexicographic order of their encoded CBOR bytes
/// - Duplicate keys are not allowed (enforced by the implementation)
/// - Keys and values can be any type that implements `Into<CBOR>`
/// - Numeric reduction is applied (e.g., 3.0 is stored as integer 3)
/// 
/// This deterministic encoding ensures that equivalent maps always produce identical byte 
/// representations, which is crucial for applications that rely on consistent hashing,
/// digital signatures, or other cryptographic operations.
/// 
/// ## Features
/// 
/// The `Map` type provides:
/// - Built-in conversions from standard Rust collections like `HashMap` and `BTreeMap`
/// - Type-safe conversions when extracting values with `get<K, V>()` and `extract<K, V>()`
/// - Automatic deterministic ordering of keys
/// - Prevention of duplicate keys
/// - Support for heterogeneous key and value types
/// 
/// ## Examples
/// 
/// ### Creating and using maps
/// 
/// ```
/// use dcbor::prelude::*;
/// use std::collections::HashMap;
/// 
/// // Create a new Map directly
/// let mut map = Map::new();
/// map.insert(1, "one");                // Integer key
/// map.insert("two", 2);                // String key
/// map.insert([1, 2, 3], "array key");  // Array key
/// map.insert(3.0, "numeric reduction"); // Float key (stored as integer 3)
/// 
/// // Check the map size
/// assert_eq!(map.len(), 4);
/// 
/// // Create a CBOR value from the map
/// let cbor_map: CBOR = map.into();
/// 
/// // Round-trip through binary encoding
/// let encoded = cbor_map.to_cbor_data();
/// let decoded = CBOR::try_from_data(&encoded).unwrap();
/// 
/// // View the diagnostic representation
/// assert!(decoded.diagnostic_flat().contains(r#""two": 2"#));
/// ```
/// 
/// ### Converting from standard Rust collections
/// 
/// ```
/// use dcbor::prelude::*;
/// use std::collections::{HashMap, BTreeMap};
/// 
/// // Convert HashMap to CBOR Map
/// let mut hash_map = HashMap::new();
/// hash_map.insert("a", 1);
/// hash_map.insert("b", 2);
/// let cbor_from_hashmap: CBOR = hash_map.into();
/// 
/// // Convert BTreeMap to CBOR Map
/// let mut btree_map = BTreeMap::new();
/// btree_map.insert("x", "value1");
/// btree_map.insert("y", "value2");
/// let cbor_from_btree: CBOR = btree_map.into();
/// ```
/// 
/// ### Type-safe extraction of values
/// 
/// ```
/// use dcbor::prelude::*;
/// 
/// // Create a map with various types
/// let mut typed_map = Map::new();
/// typed_map.insert("number", 42);
/// typed_map.insert("text", "hello");
/// typed_map.insert("array", vec![1, 2, 3]);
/// 
/// // Type-safe extraction
/// let number: i32 = typed_map.extract("number").unwrap();
/// let text: String = typed_map.extract("text").unwrap();
/// let array: Vec<i32> = typed_map.extract("array").unwrap();
/// 
/// assert_eq!(number, 42);
/// assert_eq!(text, "hello");
/// assert_eq!(array, vec![1, 2, 3]);
/// 
/// // Using get() for optional extraction
/// let present: Option<i32> = typed_map.get("number");
/// let absent: Option<i32> = typed_map.get("missing");
/// 
/// assert_eq!(present, Some(42));
/// assert_eq!(absent, None);
/// ```
/// 
/// ## Implementation Details
/// 
/// The `Map` implementation:
/// - Uses a `BTreeMap` internally to maintain the sorted order of keys
/// - Encodes keys with their CBOR representation for lexicographic sorting
/// - Applies all dCBOR deterministic encoding rules automatically
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
        self.0.insert(MapKey::new(key.to_cbor_data()), MapValue::new(key, value));
    }

    pub(crate) fn insert_next(&mut self, key: CBOR, value: CBOR) -> Result<()> {
        match self.0.last_key_value() {
            None => {
                self.insert(key, value);
                Ok(())
            },
            Some(entry) => {
                let new_key = MapKey::new(key.to_cbor_data());
                if self.0.contains_key(&new_key) {
                    bail!(CBORError::DuplicateMapKey)
                }
                if entry.0 >= &new_key {
                    bail!(CBORError::MisorderedMapKey)
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
        K: Into<CBOR>, V: TryFrom<CBOR>
    {
        match self.0.get(&MapKey::new(key.into().to_cbor_data())) {
            Some(value) => V::try_from(value.value.clone()).ok(),
            None => None
        }
    }

    /// Get a value from the map, given a key.
    ///
    /// Returns `Ok` if the key is present in the map, `Err` otherwise.
    pub fn extract<K, V>(&self, key: K) -> Result<V>
    where
        K: Into<CBOR>, V: TryFrom<CBOR>
    {
        match self.get(key) {
            Some(value) => Ok(value),
            None => bail!(CBORError::MissingMapKey)
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

impl Map {
    pub fn cbor_data(&self) -> Vec<u8> {
        let pairs: Vec<(Vec<u8>, Vec<u8>)> = self.0.iter().map(|x| {
            let a: Vec<u8> = x.0.0.to_owned();
            let cbor: &CBOR = &x.1.value;
            let b: Vec<u8> = cbor.to_cbor_data();
            (a, b)
        }).collect();
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
        CBORCase::Map(value.clone()).into()
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

/// An iterator over the entries of a CBOR map.
///
/// This iterator provides a way to traverse the key-value pairs in a CBOR map. 
/// It always returns entries in lexicographic order by the key's binary-encoded 
/// CBOR value, which is a requirement for deterministic encoding in dCBOR.
///
/// Each item returned by the iterator is a tuple of references to the key and 
/// value CBOR objects: `(&'a CBOR, &'a CBOR)`.
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
/// 
/// // Create a map with several entries
/// let mut map = Map::new();
/// map.insert(1, "one");
/// map.insert(2, "two");
/// map.insert(3, "three");
/// 
/// // Iterate through the map entries
/// for (key, value) in map.iter() {
///     // Process each key-value pair
///     let k_option: Option<i64> = key.clone().try_into().ok();
///     let v_option: Option<String> = value.clone().try_into().ok();
///     
///     if let (Some(k), Some(v)) = (k_option, v_option) {
///         // This would normally print entries in order: 1, 2, 3
///         assert!(k >= 1 && k <= 3);
///         assert!(v == "one" || v == "two" || v == "three");
///     }
/// }
/// 
/// // Use iterator methods directly
/// let first_entry = map.iter().next().unwrap();
/// let key_as_i64: i64 = first_entry.0.clone().try_into().unwrap();
/// assert_eq!(key_as_i64, 1);
/// ```
///
/// Note that unlike standard Rust collections which may have implementation-specific 
/// ordering, CBOR maps in dCBOR guarantee deterministic iteration order based on 
/// the binary encoding of keys.
#[derive(Debug)]
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
impl<T, K, V> From<T> for Map where T: IntoIterator<Item=(K, V)>, K: Into<CBOR>, V: Into<CBOR> {
    fn from(container: T) -> Self {
        let mut map = Map::new();
        for (k, v) in container {
            map.insert(k.into(), v.into());
        }
        map
    }
}

impl<K, V> From<HashMap<K, V>> for CBOR where K: Into<CBOR>, V: Into<CBOR> {
    fn from(container: HashMap<K, V>) -> Self {
        CBORCase::Map(Map::from(container.into_iter())).into()
    }
}

impl<K, V> TryFrom<CBOR> for HashMap<K, V>
where
    K: TryFrom<CBOR, Error = Error> + cmp::Eq + hash::Hash + Clone,
    V: TryFrom<CBOR, Error = Error> + Clone,
{
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Map(map) => {
                let mut container = <HashMap<K, V>>::new();
                for (k, v) in map.iter() {
                    container.insert(k.clone().try_into()?, v.clone().try_into()?);
                }
                Ok(container)
            },
            _ => Err(Error::msg(CBORError::WrongType))
        }
    }
}

impl<K, V> From<BTreeMap<K, V>> for CBOR
where
    K: Into<CBOR>,
    V: Into<CBOR>,
{
    fn from(container: BTreeMap<K, V>) -> Self {
        CBORCase::Map(Map::from(container.into_iter())).into()
    }
}

impl<K, V> TryFrom<CBOR> for BTreeMap<K, V>
where
    K: TryFrom<CBOR, Error = Error> + cmp::Eq + (cmp::Ord) + Clone,
    V: TryFrom<CBOR, Error = Error> + Clone,
{
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Map(map) => {
                let mut container = <BTreeMap<K, V>>::new();
                for (k, v) in map.iter() {
                    let key = k.clone().try_into()?;
                    let value = v.clone().try_into()?;
                    container.insert(key, value);
                }
                Ok(container)
            },
            _ => Err(Error::msg(Box::new(CBORError::WrongType)))
        }
    }
}
