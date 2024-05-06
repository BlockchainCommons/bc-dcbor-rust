import_stdlib!();

use crate::{CBOR, error::CBORError, CBORCase};

use anyhow::bail;

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
T: TryFrom<CBOR> + Clone,
<T as TryFrom<CBOR>>::Error: Into<anyhow::Error>,
{
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = Vec::new();
                for cbor in cbor_array {
                    let element = T::try_from(cbor).map_err(|e| e.into())?;
                    result.push(element);
                }
                Ok(result)
            },
            _ => bail!(CBORError::WrongType)
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
T: TryFrom<CBOR> + Clone,
<T as TryFrom<CBOR>>::Error: Into<anyhow::Error>,
{
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = VecDeque::new();
                for cbor in cbor_array {
                    let element = T::try_from(cbor).map_err(|e| e.into())?;
                    result.push_back(element);
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
T: TryFrom<CBOR> + Eq + hash::Hash + Clone,
<T as TryFrom<CBOR>>::Error: Into<anyhow::Error>,
{
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> anyhow::Result<Self> {
        match cbor.into_case() {
            CBORCase::Array(cbor_array) => {
                let mut result = HashSet::new();
                for cbor in cbor_array {
                    let element = T::try_from(cbor).map_err(|e| e.into())?;
                    result.insert(element);
                }
                Ok(result)
            },
            _ => bail!(CBORError::WrongType)
        }
    }
}
