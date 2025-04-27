import_stdlib!();

use crate::{ CBOREncodable, Error, Map, MapIter, Result, CBOR };

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Set(Map);

impl Set {
    /// Makes a new, empty CBOR `Set`.
    pub fn new() -> Self {
        Self(Map::new())
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets an iterator over the elements in the CBOR set.
    ///
    /// Order is lexicographically by the value's binary-encoded CBOR.
    pub fn iter(&self) -> SetIter<'_> {
        SetIter::new(self.0.iter())
    }

    /// Inserts a value into the set.
    pub fn insert<T>(&mut self, value: T) where T: Into<CBOR> + Clone {
        self.0.insert(value.clone(), value)
    }

    pub(crate) fn insert_next(&mut self, value: CBOR) -> Result<()> {
        self.0.insert_next(value.clone(), value)
    }

    /// Tests if the set contains a value.
    pub fn contains<T>(&self, value: T) -> bool where T: Into<CBOR> {
        self.0.contains_key(value)
    }
}

impl Set {
    pub fn as_vec(&self) -> Vec<CBOR> {
        self.0
            .iter()
            .map(|(_, v)| v.clone())
            .collect()
    }

    pub fn from_vec<T>(vec: Vec<T>) -> Self where T: Into<CBOR> + Clone {
        let mut set = Set::new();
        for item in vec {
            set.insert(item);
        }
        set
    }

    pub fn try_from_vec<T>(vec: Vec<T>) -> Result<Self> where T: Into<CBOR> + Clone {
        let mut set = Set::new();
        for item in vec {
            set.insert_next(item.into())?;
        }
        Ok(set)
    }

    pub fn cbor_data(&self) -> Vec<u8> {
        self.as_vec().to_cbor_data()
    }
}

impl From<Set> for CBOR {
    fn from(value: Set) -> Self {
        value.as_vec().into()
    }
}

impl<T> TryFrom<Vec<T>> for Set where T: Into<CBOR> + Clone {
    type Error = Error;

    fn try_from(vec: Vec<T>) -> Result<Self> {
        Self::try_from_vec(vec)
    }
}

impl<T> TryFrom<Set> for Vec<T> where T: TryFrom<CBOR, Error = Error> + Clone {
    type Error = Error;

    fn try_from(set: Set) -> Result<Self> {
        set.as_vec()
            .into_iter()
            .map(|item| item.try_into())
            .collect()
    }
}

impl<T> From<HashSet<T>> for Set where T: Into<CBOR> + Clone {
    fn from(vec: HashSet<T>) -> Self {
        Self::from_vec(vec.into_iter().collect())
    }
}

impl<T> TryFrom<Set> for HashSet<T> where T: TryFrom<CBOR, Error = Error> + Clone + Eq + hash::Hash {
    type Error = Error;

    fn try_from(set: Set) -> Result<Self> {
        set.as_vec()
            .into_iter()
            .map(|item| item.try_into())
            .collect()
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_vec())
    }
}

#[derive(Debug)]
pub struct SetIter<'a>(MapIter<'a>);

impl<'a> SetIter<'a> {
    pub fn new(iter: MapIter<'a>) -> Self {
        Self(iter)
    }
}

impl<'a> Iterator for SetIter<'a> {
    type Item = &'a CBOR;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, v)| v)
    }
}
