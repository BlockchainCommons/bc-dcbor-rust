import_stdlib!();

use crate::{ Error, CBORCase, CBOR };

/// # Array Support in dCBOR
///
/// dCBOR provides convenient conversions to and from CBOR arrays through implementation of the
/// `From<T>` and `TryFrom<CBOR>` traits for various collection types. This enables idiomatic
/// conversions using Rust's `.into()` method.
///
/// Supported collection types:
/// - Rust arrays (`[T; N]`)
/// - Vectors (`Vec<T>`)
/// - VecDeque (`VecDeque<T>`)
/// - HashSet (`HashSet<T>`)
///
/// For all of these, the elements must be convertible to CBOR via the `Into<CBOR>` trait.
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create CBOR from Rust array (fixed size)
/// let array_cbor: CBOR = [1, 2, 3].into();
///
/// // Create CBOR from Vec
/// let vec = vec![1, 2, 3];
/// let vec_cbor: CBOR = vec.into();
///
/// // Mixed types in Vec
/// let mixed_vec: Vec<CBOR> = vec![
///     1.into(),                  // Integer
///     "hello".into(),            // String
///     [1, 2, 3].into(),          // Nested array
///     true.into()                // Boolean
/// ];
/// let mixed_cbor: CBOR = mixed_vec.into();
///
/// // Convert back to Vec
/// let numbers: Vec<i32> = array_cbor.try_into().unwrap();
/// assert_eq!(numbers, vec![1, 2, 3]);
/// ```
impl<T> From<Vec<T>> for CBOR where T: Into<CBOR> {
    fn from(vec: Vec<T>) -> Self {
        CBORCase::Array(
            vec
                .into_iter()
                .map(|x| x.into())
                .collect()
        ).into()
    }
}

impl<T> From<&[T]> for CBOR where T: Into<CBOR> + Clone {
    fn from(array: &[T]) -> Self {
        CBORCase::Array(
            array
                .iter()
                .map(|x| x.clone().into())
                .collect()
        ).into()
    }
}

impl<T> TryFrom<CBOR> for Vec<T> where T: TryFrom<CBOR, Error = Error> + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    result.push(T::try_from(cbor)?);
                }
                Ok(result)
            }
            _ => Err(Error::WrongType),
        }
    }
}

impl<T, const N: usize> From<[T; N]> for CBOR where T: Into<CBOR> {
    fn from(array: [T; N]) -> Self {
        CBORCase::Array(
            array
                .into_iter()
                .map(|x| x.into())
                .collect()
        ).into()
    }
}

impl<T> From<VecDeque<T>> for CBOR where T: Into<CBOR> {
    fn from(deque: VecDeque<T>) -> Self {
        CBORCase::Array(
            deque
                .into_iter()
                .map(|x| x.into())
                .collect()
        ).into()
    }
}

impl<T> TryFrom<CBOR> for VecDeque<T> where T: TryFrom<CBOR, Error = Error> + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = VecDeque::new();
                for cbor in cbor_array {
                    result.push_back(T::try_from(cbor)?);
                }
                Ok(result)
            }
            _ => Err(Error::WrongType),
        }
    }
}

impl<T> From<HashSet<T>> for CBOR where T: Into<CBOR> {
    fn from(set: HashSet<T>) -> Self {
        CBORCase::Array(
            set
                .into_iter()
                .map(|x| x.into())
                .collect()
        ).into()
    }
}

impl<T> TryFrom<CBOR> for HashSet<T> where T: TryFrom<CBOR, Error = Error> + Eq + hash::Hash + Clone {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = HashSet::new();
                for cbor in cbor_array {
                    result.insert(T::try_from(cbor)?);
                }
                Ok(result)
            }
            _ => Err(Error::WrongType),
        }
    }
}

pub fn sort_array_by_cbor_encoding<T>(array: impl AsRef<[T]>) -> Vec<T> where T: Into<CBOR> + Clone {
    let mut a: Vec<(Vec<u8>, T)> = array
        .as_ref()
        .iter()
        .map(|item| (item.clone().into().to_cbor_data(), item.clone()))
        .collect();
    a.sort_by(|a, b| a.0.cmp(&b.0));
    a.into_iter()
        .map(|(_, item)| item)
        .collect()
}

pub trait CBORSortable<T> {
    fn sort_by_cbor_encoding(&self) -> Vec<T> where Self: Sized + Into<CBOR> + Clone;
}

impl<T> CBORSortable<T> for Vec<T> where T: Into<CBOR> + Clone {
    fn sort_by_cbor_encoding(&self) -> Vec<T> {
        sort_array_by_cbor_encoding(self)
    }
}

impl<T> CBORSortable<T> for &[T] where T: Into<CBOR> + Clone {
    fn sort_by_cbor_encoding(&self) -> Vec<T> {
        sort_array_by_cbor_encoding(self)
    }
}

impl<T> CBORSortable<T> for HashSet<T> where T: Into<CBOR> + Clone {
    fn sort_by_cbor_encoding(&self) -> Vec<T> {
        let array: Vec<T> = self.iter().cloned().collect();
        array.sort_by_cbor_encoding()
    }
}
