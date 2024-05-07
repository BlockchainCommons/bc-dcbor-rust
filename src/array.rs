import_stdlib!();

use crate::{error::CBORError, CBORCase, CBOR};

use anyhow::{bail, Error, Result};

impl<T> From<Vec<T>> for CBOR where T: Into<CBOR> {
    fn from(vec: Vec<T>) -> Self {
        CBORCase::Array(vec.into_iter().map(|x| x.into()).collect()).into()
    }
}

impl<T> From<&[T]> for CBOR where T: Into<CBOR> + Clone {
    fn from(array: &[T]) -> Self {
        CBORCase::Array(array.iter().map(|x| x.clone().into()).collect()).into()
    }
}

impl<T> TryFrom<CBOR> for Vec<T>
where
    T: TryFrom<CBOR, Error = Error> + Clone,
{
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    result.push(cbor.try_into()?);
                }
                Ok(result)
            },
            _ => panic!()
        }
    }
}

impl<T, const N: usize> From<[T; N]> for CBOR where T: Into<CBOR> {
    fn from(array: [T; N]) -> Self {
        CBORCase::Array(array.into_iter().map(|x| x.into()).collect()).into()
    }
}

impl<T> From<VecDeque<T>> for CBOR where T: Into<CBOR> {
    fn from(deque: VecDeque<T>) -> Self {
        CBORCase::Array(deque.into_iter().map(|x| x.into()).collect()).into()
    }
}

impl<T> TryFrom<CBOR> for VecDeque<T>
where
    T: TryFrom<CBOR, Error = Error> + Clone,
{
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = VecDeque::new();
                for cbor in cbor_array {
                    result.push_back(cbor.try_into()?);
                }
                Ok(result)
            },
            _ => bail!(CBORError::WrongType)
        }
    }
}

impl<T> From<HashSet<T>> for CBOR where T: Into<CBOR> {
    fn from(set: HashSet<T>) -> Self {
        CBORCase::Array(set.into_iter().map(|x| x.into()).collect()).into()
    }
}

impl<T> TryFrom<CBOR> for HashSet<T>
where
    T: TryFrom<CBOR, Error = Error> + Eq + hash::Hash + Clone,
{
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = HashSet::new();
                for cbor in cbor_array {
                    result.insert(cbor.try_into()?);
                }
                Ok(result)
            },
            _ => bail!(CBORError::WrongType)
        }
    }
}
