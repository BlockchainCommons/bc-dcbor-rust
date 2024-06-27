import_stdlib!();

use anyhow::Error;

use crate::CBOR;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }

    pub fn data(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend(&mut self, other: impl Into<Vec<u8>>) {
        self.0.extend(other.into())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn iter(&self) -> ByteStringIterator<'_> {
        ByteStringIterator {
            slice: &self.0,
            pos: 0,
        }
    }
}

impl From<ByteString> for Vec<u8> {
    fn from(value: ByteString) -> Self {
        value.0
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&Vec<u8>> for ByteString {
    fn from(value: &Vec<u8>) -> Self {
        Self(value.clone())
    }
}

impl From<&[u8]> for ByteString {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl From<&str> for ByteString {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl From<ByteString> for CBOR {
    fn from(value: ByteString) -> Self {
        CBOR::to_byte_string(value)
    }
}

impl TryFrom<CBOR> for ByteString {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        Ok(CBOR::try_into_byte_string(cbor)?.into())
    }
}

impl<const N: usize> From<[u8; N]> for ByteString {
    fn from(value: [u8; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for ByteString {
    fn from(value: &[u8; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<const N: usize> TryFrom<ByteString> for [u8; N] {
    type Error = TryFromSliceError;

    fn try_from(value: ByteString) -> Result<Self, Self::Error> {
        value.0.as_slice().try_into()
    }
}

impl<'a> IntoIterator for &'a ByteString {
    type Item = &'a u8;
    type IntoIter = ByteStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for ByteString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ByteStringIterator<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ByteStringIterator<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.slice.len() {
            let item = &self.slice[self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
