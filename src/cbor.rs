import_stdlib!();

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(all(not(feature = "multithreaded"), not(feature = "std")))]
use alloc::rc::Rc as RefCounted;
#[cfg(all(feature = "multithreaded", not(feature = "std")))]
use alloc::sync::Arc as RefCounted;
#[cfg(all(not(feature = "multithreaded"), feature = "std"))]
use std::rc::Rc as RefCounted;
#[cfg(all(feature = "multithreaded", feature = "std"))]
use std::sync::Arc as RefCounted;

use unicode_normalization::UnicodeNormalization;

use super::string_util::flanked;
use crate::{
    ByteString, Map, Simple,
    decode::decode_cbor,
    error::Result,
    tag::Tag,
    varint::{EncodeVarInt, MajorType},
};

/// A symbolic representation of CBOR data.
///
/// The `CBOR` type is the central type in the dCBOR library, representing any
/// CBOR data item using a reference-counted wrapper around a [`CBORCase`] enum.
/// This design allows efficient sharing of CBOR data structures in memory
/// without excessive copying.
///
/// # Features
///
/// - **Deterministic encoding**: Guarantees that semantically equivalent data
///   structures will always be encoded to identical byte sequences
///
/// - **Immutability**: `CBOR` is immutable. Operations that appear to "modify"
///   a `CBOR` value actually create a new value.
///
/// - **O(1) Cloning**: `CBOR` uses reference counting (`Rc` or `Arc` when the
///   `multithreaded` feature is enabled) and structure sharing to enable
///   efficient O(1) cloning of a `CBOR` or recursively, any `CBOR`s it
///   contains. Cloning a `CBOR` simply increments the reference count, allowing
///   multiple owners without duplicating the underlying data.
///
/// - **Reference counting**: Enables efficient sharing of CBOR structures using
///   `Rc` or `Arc` when the `multithreaded` feature is enabled
///
/// - **Type safety**: Uses Rust's type system to safely handle different CBOR
///   data types
///
/// - **Conversion traits**: Implements Rust's standard conversion traits for
///   ergonomic use
///
/// # Thread Safety
///
/// With the `multithreaded` feature enabled, `CBOR` uses `Arc` for reference
/// counting, making it thread-safe. Without this feature, it uses `Rc`, which
/// is more efficient but not thread-safe.
///
/// # Example
///
/// ```
/// use dcbor::prelude::*;
///
/// // 1. Create and round-trip a homogeneous array
/// let array = CBOR::from(vec![1, 2, 3]);
///
/// // Encode to bytes
/// let encoded = array.to_cbor_data();
/// assert_eq!(hex::encode(&encoded), "83010203");
///
/// // Decode from bytes
/// let decoded = CBOR::try_from_data(&encoded).unwrap();
/// assert_eq!(decoded, array);
///
/// // 2. Create and round-trip a heterogeneous array
/// let mixed_array: Vec<CBOR> =
///     vec![1.into(), "Hello".into(), vec![1, 2, 3].into()];
/// let mixed = CBOR::from(mixed_array);
///
/// // Encode the heterogeneous array to bytes
/// let mixed_encoded = mixed.to_cbor_data();
/// assert_eq!(hex::encode(&mixed_encoded), "83016548656c6c6f83010203");
///
/// // Decode from bytes
/// let mixed_decoded = CBOR::try_from_data(&mixed_encoded).unwrap();
/// assert_eq!(mixed_decoded, mixed);
/// // Use diagnostic_flat() for a compact single-line representation
/// assert_eq!(
///     mixed_decoded.diagnostic_flat(),
///     r#"[1, "Hello", [1, 2, 3]]"#
/// );
/// ```
#[derive(Clone, Eq)]
pub struct CBOR(RefCounted<CBORCase>);

impl CBOR {
    pub fn as_case(&self) -> &CBORCase { &self.0 }

    pub fn into_case(self) -> CBORCase {
        match RefCounted::try_unwrap(self.0) {
            Ok(b) => b,
            Err(ref_counted) => (*ref_counted).clone(),
        }
    }
}

impl From<CBORCase> for CBOR {
    fn from(case: CBORCase) -> Self { Self(RefCounted::new(case)) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// An enum representing all possible CBOR data types.
///
/// `CBORCase` is the core enum that represents all possible CBOR data types
/// according to [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) and the dCBOR specification.
/// Each variant corresponds to one of the eight major types in CBOR.
///
/// This enum is not typically used directly by users of the library. Instead,
/// it's wrapped by the reference-counted [`CBOR`] type, which provides a more
/// ergonomic API.
///
/// # Major Types
///
/// CBOR defines eight major types, numbered 0 through 7:
///
/// | Major Type | Name | Description |
/// |------------|------|-------------|
/// | 0 | Unsigned integer | A non-negative integer |
/// | 1 | Negative integer | A negative integer |
/// | 2 | Byte string | A sequence of bytes |
/// | 3 | Text string | A UTF-8 string |
/// | 4 | Array | A sequence of data items |
/// | 5 | Map | A collection of key-value pairs |
/// | 6 | Tagged value | A data item with a semantic tag |
/// | 7 | Simple value | A simple value like true, false, null, or float |
///
/// # dCBOR Constraints
///
/// According to the dCBOR specification, deterministic encoding adds several
/// constraints:
///
/// - Maps must have lexicographically ordered keys
/// - Numeric values must use the smallest possible encoding
/// - Floats with integer values are reduced to integers
/// - All NaN values are canonicalized to a single representation
/// - Strings must be in Unicode Normalization Form C (NFC)
///
/// # Example
///
/// ```
/// use dcbor::{CBORCase, Simple, prelude::*};
///
/// // Create a CBOR value using the CBORCase enum
/// let case = CBORCase::Array(vec![
///     CBORCase::Unsigned(1).into(),
///     CBORCase::Text("hello".to_string()).into(),
///     CBORCase::Simple(Simple::True).into(),
/// ]);
///
/// // Wrap in the CBOR type for easier handling
/// let cbor = CBOR::from(case);
/// assert_eq!(cbor.diagnostic(), "[1, \"hello\", true]");
/// ```
pub enum CBORCase {
    /// Unsigned integer (major type 0).
    ///
    /// Represents a non-negative integer from 0 to 2^64-1.
    Unsigned(u64),

    /// Negative integer (major type 1).
    ///
    /// Actual value is -1 - n, allowing representation of negative integers
    /// from -1 to -2^64.
    Negative(u64),

    /// Byte string (major type 2).
    ///
    /// Represents a sequence of bytes. In dCBOR, byte strings must use
    /// the most compact representation possible.
    ByteString(ByteString),

    /// UTF-8 string (major type 3).
    ///
    /// Represents a UTF-8 encoded string. In dCBOR, text strings must
    /// be in Unicode Normalization Form C (NFC).
    Text(String),

    /// Array (major type 4).
    ///
    /// Represents a sequence of CBOR data items. dCBOR does not support
    /// indefinite-length arrays.
    Array(Vec<CBOR>),

    /// Map (major type 5).
    ///
    /// Represents a collection of key-value pairs. In dCBOR, map keys
    /// must be in lexicographic order, and duplicate keys are not allowed.
    Map(Map),

    /// Tagged value (major type 6).
    ///
    /// Represents a data item with a semantic tag. The tag provides
    /// additional information about how to interpret the data.
    Tagged(Tag, CBOR),

    /// Simple value (major type 7).
    ///
    /// Represents simple values like true, false, null, and floating-point
    /// numbers. In dCBOR, only a limited set of simple values are allowed.
    Simple(Simple),
}

/// Methods for decoding CBOR from binary representation and encoding to binary.
impl CBOR {
    /// Decodes binary data into CBOR symbolic representation.
    ///
    /// This method parses the provided binary data according to the CBOR and
    /// dCBOR specifications, validating that it follows all deterministic
    /// encoding rules.
    ///
    /// # Arguments
    ///
    /// * `data` - The binary data to decode, which can be any type that can be
    ///   referenced as a byte slice (e.g., `Vec<u8>`, `&[u8]`, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(CBOR)` - A CBOR value if decoding was successful
    /// * `Err` - If the data is not valid CBOR or violates dCBOR encoding rules
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Decode a CBOR array [1, 2, 3]
    /// let data = hex_literal::hex!("83010203");
    /// let cbor = CBOR::try_from_data(&data).unwrap();
    ///
    /// // Get the array contents
    /// let array: Vec<u64> = cbor.try_into().unwrap();
    /// assert_eq!(array, vec![1, 2, 3]);
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The data is not valid CBOR
    /// - The data violates dCBOR encoding rules (e.g., non-canonical integer
    ///   encoding)
    /// - The data has content after the end of the CBOR item
    pub fn try_from_data(data: impl AsRef<[u8]>) -> Result<CBOR> {
        decode_cbor(data)
    }

    /// Decodes a hexadecimal string into CBOR symbolic representation.
    ///
    /// This is a convenience method that converts a hexadecimal string to
    /// binary data and then calls [`try_from_data`](Self::try_from_data).
    ///
    /// # Arguments
    ///
    /// * `hex` - A string containing hexadecimal characters (no spaces or other
    ///   characters)
    ///
    /// # Returns
    ///
    /// * `Ok(CBOR)` - A CBOR value if decoding was successful
    /// * `Err` - If the hex string is invalid or the resulting data is not
    ///   valid dCBOR
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Decode a CBOR array [1, 2, 3] from hex
    /// let cbor = CBOR::try_from_hex("83010203").unwrap();
    /// assert_eq!(cbor.diagnostic(), "[1, 2, 3]");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the hex string is not well-formed hexadecimal
    /// (contains non-hex characters or an odd number of digits).
    pub fn try_from_hex(hex: &str) -> Result<CBOR> {
        let data = hex::decode(hex).unwrap();
        Self::try_from_data(data)
    }

    /// Encodes this CBOR value to binary data following dCBOR encoding rules.
    ///
    /// This method converts the CBOR value to a byte vector according to the
    /// dCBOR specification, ensuring deterministic encoding.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encoded CBOR data.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Create a CBOR map
    /// let mut map = Map::new();
    /// map.insert(CBOR::from("key"), CBOR::from(123));
    /// let cbor = CBOR::from(map);
    ///
    /// // Encode to binary
    /// let encoded = cbor.to_cbor_data();
    /// assert_eq!(hex::encode(&encoded), "a1636b6579187b");
    ///
    /// // Round-trip through encoding and decoding
    /// let decoded = CBOR::try_from_data(&encoded).unwrap();
    /// assert_eq!(decoded, cbor);
    /// ```
    pub fn to_cbor_data(&self) -> Vec<u8> {
        match self.as_case() {
            CBORCase::Unsigned(x) => x.encode_varint(MajorType::Unsigned),
            CBORCase::Negative(x) => x.encode_varint(MajorType::Negative),
            CBORCase::ByteString(x) => {
                let mut buf = x.len().encode_varint(MajorType::ByteString);
                buf.extend(x);
                buf
            }
            CBORCase::Text(x) => {
                let nfc = x.nfc().collect::<String>();
                let mut buf = nfc.len().encode_varint(MajorType::Text);
                buf.extend(nfc.as_bytes());
                buf
            }
            CBORCase::Array(x) => {
                let mut buf = x.len().encode_varint(MajorType::Array);
                for item in x {
                    buf.extend(item.to_cbor_data());
                }
                buf
            }
            CBORCase::Map(x) => x.cbor_data(),
            CBORCase::Tagged(tag, item) => {
                let mut buf = tag.value().encode_varint(MajorType::Tagged);
                buf.extend(item.to_cbor_data());
                buf
            }
            CBORCase::Simple(x) => x.cbor_data(),
        }
    }
}

impl PartialEq for CBOR {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_case(), other.as_case()) {
            (CBORCase::Unsigned(l0), CBORCase::Unsigned(r0)) => l0 == r0,
            (CBORCase::Negative(l0), CBORCase::Negative(r0)) => l0 == r0,
            (CBORCase::ByteString(l0), CBORCase::ByteString(r0)) => l0 == r0,
            (CBORCase::Text(l0), CBORCase::Text(r0)) => l0 == r0,
            (CBORCase::Array(l0), CBORCase::Array(r0)) => l0 == r0,
            (CBORCase::Map(l0), CBORCase::Map(r0)) => l0 == r0,
            (CBORCase::Tagged(l0, l1), CBORCase::Tagged(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (CBORCase::Simple(l0), CBORCase::Simple(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl hash::Hash for CBOR {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        use CBORCase::*;
        match self.as_case() {
            Unsigned(x) => {
                0u8.hash(state);
                x.hash(state);
            }
            Negative(x) => {
                1u8.hash(state);
                x.hash(state);
            }
            ByteString(x) => {
                2u8.hash(state);
                x.hash(state);
            }
            Text(x) => {
                3u8.hash(state);
                x.hash(state);
            }
            Array(x) => {
                4u8.hash(state);
                x.hash(state);
            }
            Map(x) => {
                5u8.hash(state);
                x.hash(state);
            }
            Tagged(tag, item) => {
                6u8.hash(state);
                tag.hash(state);
                item.hash(state);
            }
            Simple(x) => {
                7u8.hash(state);
                x.hash(state);
            }
        }
    }
}

fn format_string(s: &str) -> String {
    let mut result = "".to_string();
    for c in s.chars() {
        if c == '"' {
            result.push_str(r#"\""#);
        } else {
            result.push(c);
        }
    }
    flanked(&result, r#"""#, r#"""#)
}

fn format_array(a: &[CBOR]) -> String {
    let s: Vec<String> = a.iter().map(|x| format!("{}", x)).collect();
    flanked(&s.join(", "), "[", "]")
}

fn format_map(m: &Map) -> String {
    let s: Vec<String> =
        m.iter().map(|x| format!("{}: {}", x.0, x.1)).collect();
    flanked(&s.join(", "), "{", "}")
}

impl fmt::Debug for CBOR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_case() {
            CBORCase::Unsigned(x) => {
                f.debug_tuple("unsigned").field(x).finish()
            }
            CBORCase::Negative(x) => f
                .debug_tuple("negative")
                .field(&(-1 - (*x as i128)))
                .finish(),
            CBORCase::ByteString(x) => {
                f.write_fmt(format_args!("bytes({})", hex::encode(x)))
            }
            CBORCase::Text(x) => f.debug_tuple("text").field(x).finish(),
            CBORCase::Array(x) => f.debug_tuple("array").field(x).finish(),
            CBORCase::Map(x) => f.debug_tuple("map").field(x).finish(),
            CBORCase::Tagged(tag, item) => {
                f.write_fmt(format_args!("tagged({}, {:?})", tag, item))
            }
            CBORCase::Simple(x) => {
                f.write_fmt(format_args!("simple({})", x.name()))
            }
        }
    }
}

impl fmt::Display for CBOR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.as_case() {
            CBORCase::Unsigned(x) => format!("{}", x),
            CBORCase::Negative(x) => format!("{}", -1 - (*x as i128)),
            CBORCase::ByteString(x) => format!("h'{}'", hex::encode(x)),
            CBORCase::Text(x) => format_string(x),
            CBORCase::Array(x) => format_array(x),
            CBORCase::Map(x) => format_map(x),
            CBORCase::Tagged(tag, item) => format!("{}({})", tag, item),
            CBORCase::Simple(x) => format!("{}", x),
        };
        f.write_str(&s)
    }
}
