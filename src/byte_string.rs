import_stdlib!();

use crate::{CBOR, Error};

/// Represents a CBOR byte string (major type 2).
///
/// `ByteString` is a wrapper around a byte vector, optimized for use in CBOR
/// encoding and decoding operations. It provides a richer API for working with
/// byte data in the context of CBOR compared to using raw `Vec<u8>` values.
///
/// In dCBOR, byte strings follow the general deterministic encoding rules:
/// - They must use definite-length encoding
/// - Their length must be encoded in the shortest possible form
///
/// # Use Cases
///
/// Byte strings in CBOR are commonly used for:
/// - Binary data such as images, audio, or other non-text content
/// - Cryptographic values like hashes, signatures, and public keys
/// - Embedded CBOR (wrapped with tag 24)
/// - Other serialized data formats embedded in CBOR
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Creating a byte string from various sources
/// let bytes1 = ByteString::new(vec![1, 2, 3, 4]);
/// let bytes2 = ByteString::from([5, 6, 7, 8]);
/// let bytes3 = ByteString::from(&[9, 10, 11, 12][..]);
///
/// // Converting to and from CBOR
/// let cbor = CBOR::from(bytes1.clone());
/// assert_eq!(cbor.diagnostic(), "h'01020304'");
///
/// // The byte string can be extracted from CBOR
/// let extracted: ByteString = cbor.try_into().unwrap();
/// assert_eq!(extracted, bytes1);
///
/// // ByteString provides Vec<u8>-like operations
/// let mut bytes = ByteString::new(vec![1, 2]);
/// bytes.extend(vec![3, 4]);
/// assert_eq!(bytes.len(), 4);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    /// Creates a new `ByteString` from any type that can be converted into a
    /// byte vector.
    ///
    /// This constructor accepts any type that implements `Into<Vec<u8>>`, which
    /// includes common types like `Vec<u8>`, arrays `[u8; N]`, slices
    /// `&[u8]`, and string slices `&str` (via the `From<&str>`
    /// implementation).
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // From a vector
    /// let bytes1 = ByteString::new(vec![1, 2, 3, 4]);
    ///
    /// // From a byte array
    /// let bytes2 = ByteString::new([5, 6, 7, 8]);
    ///
    /// // From a byte slice
    /// let bytes3 = ByteString::new(&[9, 10, 11, 12][..]);
    ///
    /// // From a hex string (using an implementation elsewhere)
    /// let hex = "deadbeef";
    /// let bytes4 = ByteString::new(hex::decode(hex).unwrap());
    /// ```
    pub fn new(data: impl AsRef<[u8]>) -> Self { Self(data.as_ref().to_vec()) }

    /// Returns a reference to the underlying byte data.
    ///
    /// This method provides access to the raw bytes without cloning them.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let bytes = ByteString::new([1, 2, 3, 4]);
    /// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
    ///
    /// // You can use standard slice operations on the result
    /// assert_eq!(bytes.data()[1..3], [2, 3]);
    /// ```
    pub fn data(&self) -> &[u8] { &self.0 }

    /// Returns the length of the byte string in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let empty = ByteString::new(vec![]);
    /// assert_eq!(empty.len(), 0);
    ///
    /// let bytes = ByteString::new([1, 2, 3, 4]);
    /// assert_eq!(bytes.len(), 4);
    /// ```
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the byte string contains no bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let empty = ByteString::new(vec![]);
    /// assert!(empty.is_empty());
    ///
    /// let bytes = ByteString::new([1, 2, 3, 4]);
    /// assert!(!bytes.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Extends the byte string with additional bytes.
    ///
    /// This method appends the bytes from the provided value to the end of
    /// the existing byte string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let mut bytes = ByteString::new([1, 2]);
    /// bytes.extend([3, 4]);
    /// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
    ///
    /// // You can extend with different types
    /// bytes.extend(vec![5, 6]);
    /// assert_eq!(bytes.data(), &[1, 2, 3, 4, 5, 6]);
    /// ```
    pub fn extend(&mut self, other: impl AsRef<[u8]>) {
        self.0.extend(other.as_ref());
    }

    /// Creates a new vector containing a copy of the byte string's data.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let bytes = ByteString::new([1, 2, 3, 4]);
    /// let vec = bytes.to_vec();
    /// assert_eq!(vec, vec![1, 2, 3, 4]);
    ///
    /// // The returned vector is a clone, so you can modify it independently
    /// let mut vec2 = bytes.to_vec();
    /// vec2.push(5);
    /// assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    /// assert_eq!(bytes.data(), &[1, 2, 3, 4]); // original unchanged
    /// ```
    pub fn to_vec(&self) -> Vec<u8> { self.0.clone() }

    /// Returns an iterator over the bytes in the byte string.
    ///
    /// The iterator yields references to each byte in the byte string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let bytes = ByteString::new([1, 2, 3]);
    /// let mut iter = bytes.iter();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    ///
    /// // You can also use for loops
    /// let mut sum = 0;
    /// for byte in bytes.iter() {
    ///     sum += *byte;
    /// }
    /// assert_eq!(sum, 6);
    /// ```
    pub fn iter(&self) -> ByteStringIterator<'_> {
        ByteStringIterator { slice: &self.0, pos: 0 }
    }
}

/// Converts a `ByteString` into a `Vec<u8>`.
///
/// This allows consuming a byte string and getting ownership of the underlying
/// byte vector.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
/// let vec: Vec<u8> = bytes.into();
/// assert_eq!(vec, vec![1, 2, 3, 4]);
///
/// // This is useful when you need to pass the data to a function that expects Vec<u8>
/// fn process_bytes(data: Vec<u8>) -> usize {
///     data.len()
/// }
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
/// let size = process_bytes(bytes.into());
/// assert_eq!(size, 4);
/// ```
impl From<ByteString> for Vec<u8> {
    fn from(value: ByteString) -> Self { value.0 }
}

/// Converts a `Vec<u8>` into a `ByteString`.
///
/// This conversion takes ownership of the vector, avoiding a copy operation.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let vec = vec![1, 2, 3, 4];
/// let bytes = ByteString::from(vec);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // You can also use the .into() method
/// let vec = vec![5, 6, 7, 8];
/// let bytes: ByteString = vec.into();
/// assert_eq!(bytes.data(), &[5, 6, 7, 8]);
/// ```
impl From<Vec<u8>> for ByteString {
    fn from(value: Vec<u8>) -> Self { Self(value) }
}

/// Converts a reference to a `Vec<u8>` into a `ByteString`.
///
/// This conversion clones the vector data since it only has a reference.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let vec = vec![1, 2, 3, 4];
/// let bytes = ByteString::from(&vec);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // The original vector is unchanged
/// assert_eq!(vec, vec![1, 2, 3, 4]);
/// ```
impl From<&Vec<u8>> for ByteString {
    fn from(value: &Vec<u8>) -> Self { Self(value.clone()) }
}

/// Converts a byte slice into a `ByteString`.
///
/// This conversion clones the slice data since it only has a reference.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let slice: &[u8] = &[1, 2, 3, 4];
/// let bytes = ByteString::from(slice);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // You can also use the From trait with array slices
/// let array = [5, 6, 7, 8];
/// let bytes = ByteString::from(&array[..]);
/// assert_eq!(bytes.data(), &[5, 6, 7, 8]);
/// ```
impl From<&[u8]> for ByteString {
    fn from(value: &[u8]) -> Self { Self(value.to_vec()) }
}

/// Converts a string slice into a `ByteString`.
///
/// This conversion treats the string as UTF-8 and stores its raw byte
/// representation. Note that this doesn't perform any additional encoding or
/// normalization - it simply uses the raw UTF-8 bytes of the string.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let s = "hello";
/// let bytes = ByteString::from(s);
/// assert_eq!(bytes.data(), b"hello");
///
/// // UTF-8 multi-byte characters work as expected
/// let s = "こんにちは"; // "hello" in Japanese
/// let bytes = ByteString::from(s);
/// assert_eq!(bytes.data(), "こんにちは".as_bytes());
/// ```
impl From<&str> for ByteString {
    fn from(value: &str) -> Self { Self(value.as_bytes().to_vec()) }
}

/// Converts a `ByteString` into a CBOR byte string value.
///
/// This conversion creates a CBOR item of major type 2 (byte string)
/// containing the byte string's data.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
/// let cbor = CBOR::from(bytes);
///
/// // The result is a CBOR byte string
/// assert_eq!(cbor.diagnostic(), "h'01020304'");
///
/// // The encoding follows dCBOR rules
/// assert_eq!(cbor.to_cbor_data(), vec![0x44, 0x01, 0x02, 0x03, 0x04]);
/// //           0x44: byte string (major type 2) of length 4
/// ```
impl From<ByteString> for CBOR {
    fn from(value: ByteString) -> Self { CBOR::to_byte_string(value) }
}

/// Attempts to convert a CBOR value into a `ByteString`.
///
/// This conversion succeeds if the CBOR value is a byte string (major type 2),
/// otherwise it returns a `WrongType` error.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Converting from a CBOR byte string works
/// let cbor = CBOR::to_byte_string([1, 2, 3, 4]);
/// let bytes: ByteString = cbor.try_into().unwrap();
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // Converting from a different CBOR type fails
/// let cbor = CBOR::from(42);
/// let result: dcbor::Result<ByteString> = cbor.try_into();
/// assert!(result.is_err());
/// ```
impl TryFrom<CBOR> for ByteString {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        Ok(CBOR::try_into_byte_string(cbor)?.into())
    }
}

/// Converts a fixed-size byte array into a `ByteString`.
///
/// This provides a convenient way to create byte strings from array literals.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::from([1, 2, 3, 4]);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // Works with arrays of any size
/// let bytes = ByteString::from([0; 10]);
/// assert_eq!(bytes.data(), &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// ```
impl<const N: usize> From<[u8; N]> for ByteString {
    fn from(value: [u8; N]) -> Self { Self(value.to_vec()) }
}

/// Converts a reference to a fixed-size byte array into a `ByteString`.
///
/// This avoids moving ownership of the array while still creating a byte string
/// from it.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let array = [1, 2, 3, 4];
/// let bytes = ByteString::from(&array);
/// assert_eq!(bytes.data(), &[1, 2, 3, 4]);
///
/// // The original array is unchanged
/// assert_eq!(array, [1, 2, 3, 4]);
/// ```
impl<const N: usize> From<&[u8; N]> for ByteString {
    fn from(value: &[u8; N]) -> Self { Self(value.to_vec()) }
}

/// Attempts to convert a `ByteString` into a fixed-size byte array.
///
/// This conversion succeeds only if the byte string has exactly the right
/// length for the target array. If it has a different length, it returns a
/// `TryFromSliceError`.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // When the lengths match, conversion succeeds
/// let bytes = ByteString::new([1, 2, 3, 4]);
/// let array: [u8; 4] = bytes.try_into().unwrap();
/// assert_eq!(array, [1, 2, 3, 4]);
/// // When the lengths don't match, conversion fails
/// let bytes = ByteString::new([1, 2, 3]);
/// let result: core::result::Result<[u8; 4], _> = bytes.try_into();
/// assert!(result.is_err());
/// ```
impl<const N: usize> TryFrom<ByteString> for [u8; N] {
    type Error = TryFromSliceError;

    fn try_from(value: ByteString) -> Result<Self, Self::Error> {
        value.0.as_slice().try_into()
    }
}

/// Implements the `IntoIterator` trait for references to `ByteString`.
///
/// This allows using a byte string directly in a `for` loop, which will
/// iterate over references to each byte.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
///
/// // You can use a for loop directly on a ByteString reference
/// let mut sum = 0;
/// for byte in &bytes {
///     sum += *byte;
/// }
/// assert_eq!(sum, 10);
///
/// // This is equivalent to using the iter() method
/// let mut product = 1;
/// for byte in bytes.iter() {
///     product *= *byte;
/// }
/// assert_eq!(product, 24);
/// ```
impl<'a> IntoIterator for &'a ByteString {
    type Item = &'a u8;
    type IntoIter = ByteStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// Implements the `AsRef<[u8]>` trait for `ByteString`.
///
/// This allows a byte string to be used with functions that accept
/// references to byte slices.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
///
/// // Function that takes anything that can be referenced as a byte slice
/// fn first_byte(data: impl AsRef<[u8]>) -> Option<u8> {
///     let slice = data.as_ref();
///     slice.first().copied()
/// }
///
/// assert_eq!(first_byte(&bytes), Some(1));
/// ```
impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

/// Implements the `Deref` trait for `ByteString`, allowing it to be treated as
/// a slice.
///
/// This implementation makes it possible to use slice methods directly on a
/// `ByteString` without explicitly calling `.data()` or `.as_ref()`.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3, 4]);
///
/// // You can use slice indexing directly
/// assert_eq!(bytes[0], 1);
/// assert_eq!(bytes[1..3], [2, 3]);
///
/// // You can use slice methods directly
/// assert_eq!(bytes.first(), Some(&1));
/// assert_eq!(bytes.last(), Some(&4));
/// ```
impl Deref for ByteString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// An iterator over the bytes in a `ByteString`.
///
/// This iterator yields references to each byte in the byte string in sequence.
/// It is created by the `ByteString::iter()` method or by using the
/// `IntoIterator` trait on a reference to a `ByteString`.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let bytes = ByteString::new([1, 2, 3]);
/// let mut iter = bytes.iter();
///
/// assert_eq!(iter.next(), Some(&1));
/// assert_eq!(iter.next(), Some(&2));
/// assert_eq!(iter.next(), Some(&3));
/// assert_eq!(iter.next(), None);
/// ```
pub struct ByteStringIterator<'a> {
    /// The slice being iterated over
    slice: &'a [u8],

    /// The current position in the slice
    pos: usize,
}

/// Implements the `Iterator` trait for `ByteStringIterator`.
///
/// This provides the core functionality for iterating over a byte string's
/// contents.
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        use crate::prelude::*;
        // When the lengths match, conversion succeeds
        let bytes = ByteString::new([1, 2, 3, 4]);
        let array: [u8; 4] = bytes.try_into().unwrap();
        assert_eq!(array, [1, 2, 3, 4]);
        // When the lengths don't match, conversion fails
        let bytes = ByteString::new([1, 2, 3]);
        let result: core::result::Result<[u8; 4], _> = bytes.try_into();
        assert!(result.is_err());
    }
}
