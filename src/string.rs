import_stdlib!();

use anyhow::{bail, Error, Result};

use crate::{CBOR, CBORError, CBORCase};

/// # Text Strings in dCBOR
/// 
/// dCBOR encodes text strings according to strict deterministic rules:
/// 
/// - All strings must be in Unicode Normalization Form C (NFC)
/// - This ensures that semantically equivalent strings (like composed vs decomposed characters)
///   are always encoded identically
///
/// dCBOR provides conversions for both `String` and `&str` types through the
/// `Into<CBOR>` trait, making it easy to create CBOR values from Rust strings.
/// 
/// ## Examples
/// 
/// ```
/// use dcbor::prelude::*;
/// 
/// // Create CBOR from string literals
/// let cbor_str: CBOR = "Hello, world!".into();
/// 
/// // Create CBOR from owned String
/// let string = String::from("Hello, dCBOR!");
/// let cbor_string: CBOR = string.into();
/// 
/// // Use in collections
/// let mut map = Map::new();
/// map.insert("key", "value");
/// map.insert("array", [1, 2, 3]);
/// 
/// // Convert back to String
/// let value: String = cbor_str.try_into().unwrap();
/// assert_eq!(value, "Hello, world!");
/// ```
/// 
/// ## Unicode Normalization
/// 
/// dCBOR automatically ensures strings are in NFC form, which guarantees consistent 
/// encoding across platforms. This is important for applications that rely on
/// deterministic encoding for hashing or signing operations.
/// 
/// For example, characters like "é" can be represented as either a single code point (U+00E9, NFC)
/// or as "e" followed by the combining acute accent (U+0065 U+0301, NFD). dCBOR ensures
/// these are always encoded consistently in NFC form.
impl From<&str> for CBOR {
    fn from(value: &str) -> Self {
        CBORCase::Text(value.to_string()).into()
    }
}

impl From<String> for CBOR {
    fn from(value: String) -> Self {
        CBORCase::Text(value.clone()).into()
    }
}

impl TryFrom<CBOR> for String {
    type Error = Error;
    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Text(s) => Ok(s),
            _ => bail!(CBORError::WrongType),
        }
    }
}
