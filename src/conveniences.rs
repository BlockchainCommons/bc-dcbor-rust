//! # Conveniences for CBOR Values
//!
//! This module provides convenience functions for working with CBOR
//! (Concise Binary Object Representation) values. It extends the `CBOR`
//! type with a set of methods for creating, converting, and inspecting
//! various CBOR data types.
//!
//! The convenience functions are organized into several categories:
//!
//! * **Byte Strings** - Methods for working with CBOR byte strings
//! * **Tagged Values** - Methods for creating and extracting CBOR tagged values
//! * **Text Strings** - Methods for working with CBOR text strings
//! * **Arrays** - Methods for creating and manipulating CBOR arrays
//! * **Maps** - Methods for creating and manipulating CBOR maps
//! * **Simple Values** - Methods for working with CBOR simple values like
//!   `true`, `false`, `null`
//! * **Numeric Values** - Methods for working with CBOR numeric types
//!
//! These methods make it easier to work with CBOR values in a type-safe
//! manner, providing clearer error messages and more intuitive conversions
//! between Rust and CBOR types.

import_stdlib!();

use crate::{CBOR, CBORCase, Error, Map, Result, Simple, tag::Tag};

/// Conveniences for byte strings.
impl CBOR {
    /// Creates a new CBOR value representing a byte string.
    ///
    /// This method creates a CBOR byte string from any type that
    /// can be referenced as a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - The bytes to include in the byte string, which can be any
    ///   type that can be referenced as a byte slice (e.g., `Vec<u8>`, `&[u8]`,
    ///   etc.)
    ///
    /// # Returns
    ///
    /// A new CBOR value representing the byte string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dcbor::prelude::*;
    ///
    /// // Create a CBOR byte string from a byte slice
    /// let bytes = vec![0x01, 0x02, 0x03];
    /// let cbor = CBOR::to_byte_string(&bytes);
    ///
    /// // Encode to CBOR binary
    /// let encoded = cbor.to_cbor_data();
    /// assert_eq!(hex::encode(&encoded), "43010203");
    ///
    /// // Convert back to bytes
    /// let recovered: Vec<u8> = cbor.try_into().unwrap();
    /// assert_eq!(recovered, vec![0x01, 0x02, 0x03]);
    /// ```
    pub fn to_byte_string(data: impl AsRef<[u8]>) -> CBOR {
        CBORCase::ByteString(data.as_ref().into()).into()
    }

    /// Creates a new CBOR value representing a byte string from a hexadecimal
    /// string.
    ///
    /// This is a convenience method that converts a hexadecimal string to a
    /// byte array and then creates a CBOR byte string value.
    ///
    /// # Arguments
    ///
    /// * `hex` - A string containing hexadecimal characters (no spaces or other
    ///   characters)
    ///
    /// # Returns
    ///
    /// A new CBOR value representing the byte string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Create a CBOR byte string from a hex string
    /// let cbor = CBOR::to_byte_string_from_hex("010203");
    ///
    /// // Get the diagnostic representation
    /// assert_eq!(cbor.diagnostic(), "h'010203'");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the hex string is not well-formed hexadecimal
    /// (contains non-hex characters or an odd number of digits).
    pub fn to_byte_string_from_hex(hex: impl AsRef<str>) -> CBOR {
        Self::to_byte_string(hex::decode(hex.as_ref()).unwrap())
    }

    /// Extract the CBOR value as a byte string.
    ///
    /// Returns `Ok` if the value is a byte string, `Err` otherwise.
    pub fn try_into_byte_string(self) -> Result<Vec<u8>> {
        match self.into_case() {
            CBORCase::ByteString(b) => Ok(b.into()),
            _ => Err(Error::WrongType),
        }
    }

    /// Checks if a CBOR value is a byte string.
    ///
    /// # Arguments
    ///
    /// * `cbor` - A reference to a CBOR value
    ///
    /// # Returns
    ///
    /// * `true` if the value is a byte string
    /// * `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let bytes = CBOR::to_byte_string(vec![1, 2, 3]);
    /// assert!(CBOR::is_byte_string(&bytes));
    ///
    /// let text: CBOR = "hello".into();
    /// assert!(!CBOR::is_byte_string(&text));
    /// ```
    pub fn is_byte_string(&self) -> bool {
        matches!(self.as_case(), CBORCase::ByteString(_))
    }

    /// Attempts to convert the CBOR value into a byte string.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<u8>)` if the value is a byte string
    /// * `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Create a CBOR byte string
    /// let cbor = CBOR::to_byte_string(vec![1, 2, 3]);
    /// assert_eq!(cbor.into_byte_string(), Some(vec![1, 2, 3]));
    ///
    /// // This will return None since the value is a text string, not a byte string
    /// let text: CBOR = "hello".into();
    /// assert_eq!(text.into_byte_string(), None);
    /// ```
    pub fn into_byte_string(self) -> Option<Vec<u8>> {
        self.try_into_byte_string().ok()
    }

    /// Tries to convert a reference to a CBOR value into a byte string.
    ///
    /// # Arguments
    ///
    /// * `cbor` - A reference to a CBOR value
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` if the value is a byte string
    /// * `Err(Error::WrongType)` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let cbor = CBOR::to_byte_string(vec![1, 2, 3]);
    /// let bytes = CBOR::try_byte_string(&cbor).unwrap();
    /// assert_eq!(bytes, vec![1, 2, 3]);
    /// ```
    pub fn try_byte_string(&self) -> Result<Vec<u8>> {
        self.clone().try_into_byte_string()
    }

    pub fn as_byte_string(&self) -> Option<&[u8]> {
        match self.as_case() {
            CBORCase::ByteString(b) => Some(b),
            _ => None,
        }
    }
}

/// Conveniences for tagged values.
impl CBOR {
    /// Creates a new CBOR value representing a tagged value.
    ///
    /// This method creates a CBOR tagged value by applying a tag
    /// to another CBOR value. Tags provide semantic information about how
    /// the tagged data should be interpreted.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to apply, which can be any type that can be converted
    ///   to a `Tag`
    /// * `item` - The CBOR value to tag, which can be any type that can be
    ///   converted to `CBOR`
    ///
    /// # Returns
    ///
    /// A new CBOR value representing the tagged value.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// // Create a CBOR value with tag 42 applied to the string "hello"
    /// let tagged = CBOR::to_tagged_value(42, "hello");
    ///
    /// // Get the diagnostic representation
    /// assert_eq!(tagged.diagnostic(), "42(\"hello\")");
    ///
    /// // Extract the tag and the tagged value
    /// let (tag, value) = tagged.try_into_tagged_value().unwrap();
    /// assert_eq!(tag.value(), 42);
    /// let s: String = value.try_into().unwrap();
    /// assert_eq!(s, "hello");
    /// ```
    pub fn to_tagged_value(tag: impl Into<Tag>, item: impl Into<CBOR>) -> CBOR {
        CBORCase::Tagged(tag.into(), item.into()).into()
    }

    /// Extract the CBOR value as a tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value, `Err` otherwise.
    pub fn try_into_tagged_value(self) -> Result<(Tag, CBOR)> {
        match self.into_case() {
            CBORCase::Tagged(tag, value) => Ok((tag, value)),
            _ => Err(Error::WrongType),
        }
    }

    pub fn is_tagged_value(&self) -> bool {
        matches!(self.as_case(), CBORCase::Tagged(_, _))
    }

    pub fn as_tagged_value(&self) -> Option<(&Tag, &CBOR)> {
        match self.as_case() {
            CBORCase::Tagged(tag, value) => Some((tag, value)),
            _ => None,
        }
    }

    /// Tries to convert a reference to a CBOR value into a tagged value.
    ///
    /// # Arguments
    ///
    /// * `cbor` - A reference to a CBOR value
    ///
    /// # Returns
    ///
    /// * `Ok((Tag, CBOR))` - The tag and the tagged value if the CBOR value is
    ///   a tagged value
    /// * `Err(Error::WrongType)` - If the value is not a tagged value
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let tagged = CBOR::to_tagged_value(42, "hello");
    /// let (tag, value) = CBOR::try_tagged_value(&tagged).unwrap();
    /// assert_eq!(tag.value(), 42);
    /// let s: String = value.try_into().unwrap();
    /// assert_eq!(s, "hello");
    /// ```
    pub fn try_tagged_value(&self) -> Result<(Tag, CBOR)> {
        self.clone().try_into_tagged_value()
    }

    /// Extract the CBOR value as an expected tagged value.
    ///
    /// Returns `Ok` if the value is a tagged value with the expected tag, `Err`
    /// otherwise.
    pub fn try_into_expected_tagged_value(
        self,
        expected_tag: impl Into<Tag>,
    ) -> Result<CBOR> {
        let (tag, value) = self.try_into_tagged_value()?;
        let expected_tag = expected_tag.into();
        if tag == expected_tag {
            Ok(value)
        } else {
            Err(Error::WrongTag(expected_tag, tag))
        }
    }

    /// Tries to extract a CBOR value that is tagged with an expected tag from a
    /// reference.
    ///
    /// # Arguments
    ///
    /// * `cbor` - A reference to a CBOR value
    /// * `expected_tag` - The tag value that is expected
    ///
    /// # Returns
    ///
    /// * `Ok(CBOR)` - The tagged value if the CBOR value is tagged with the
    ///   expected tag
    /// * `Err(Error::WrongType)` - If the value is not a tagged value
    /// * `Err(Error::WrongTag)` - If the value is tagged but with a different
    ///   tag
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let tagged = CBOR::to_tagged_value(42, "hello");
    ///
    /// // This will succeed because the tag matches
    /// let value = CBOR::try_expected_tagged_value(&tagged, 42).unwrap();
    /// let s: String = value.try_into().unwrap();
    /// assert_eq!(s, "hello");
    ///
    /// // This will fail because the tag doesn't match
    /// let result = CBOR::try_expected_tagged_value(&tagged, 43);
    /// assert!(result.is_err());
    /// ```
    pub fn try_expected_tagged_value(
        &self,
        expected_tag: impl Into<Tag>,
    ) -> Result<CBOR> {
        self.clone().try_into_expected_tagged_value(expected_tag)
    }
}

/// Conveniences for text strings.
impl CBOR {
    /// Extract the CBOR value as a text string.
    ///
    /// Returns `Ok` if the value is a text string, `Err` otherwise.
    pub fn try_into_text(self) -> Result<String> {
        match self.into_case() {
            CBORCase::Text(t) => Ok(t),
            _ => Err(Error::WrongType),
        }
    }

    pub fn is_text(&self) -> bool {
        matches!(self.as_case(), CBORCase::Text(_))
    }

    pub fn try_text(&self) -> Result<String> { self.clone().try_into_text() }

    pub fn into_text(self) -> Option<String> { self.try_into_text().ok() }

    pub fn as_text(&self) -> Option<&str> {
        match self.as_case() {
            CBORCase::Text(t) => Some(t),
            _ => None,
        }
    }
}

/// Conveniences for arrays.
impl CBOR {
    /// Extract the CBOR value as an array.
    ///
    /// Returns `Ok` if the value is an array, `Err` otherwise.
    pub fn try_into_array(self) -> Result<Vec<CBOR>> {
        match self.into_case() {
            CBORCase::Array(a) => Ok(a),
            _ => Err(Error::WrongType),
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self.as_case(), CBORCase::Array(_))
    }

    pub fn try_array(&self) -> Result<Vec<CBOR>> {
        self.clone().try_into_array()
    }

    pub fn into_array(self) -> Option<Vec<CBOR>> { self.try_into_array().ok() }

    pub fn as_array(&self) -> Option<&[CBOR]> {
        match self.as_case() {
            CBORCase::Array(a) => Some(a),
            _ => None,
        }
    }
}

/// Conveniences for maps.
impl CBOR {
    /// Extract the CBOR value as a map.
    ///
    /// Returns `Ok` if the value is a map, `Err` otherwise.
    pub fn try_into_map(self) -> Result<Map> {
        match self.into_case() {
            CBORCase::Map(m) => Ok(m),
            _ => Err(Error::WrongType),
        }
    }

    pub fn is_map(&self) -> bool { matches!(self.as_case(), CBORCase::Map(_)) }

    pub fn try_map(&self) -> Result<Map> { self.clone().try_into_map() }

    pub fn into_map(self) -> Option<Map> { self.try_into_map().ok() }

    /// Extract the CBOR value as a simple value.
    ///
    /// Returns `Ok` if the value is a simple value, `Err` otherwise.
    pub fn try_into_simple_value(self) -> Result<Simple> {
        match self.into_case() {
            CBORCase::Simple(s) => Ok(s),
            _ => Err(Error::WrongType),
        }
    }

    pub fn as_map(&self) -> Option<&Map> {
        match self.as_case() {
            CBORCase::Map(m) => Some(m),
            _ => None,
        }
    }
}

/// Conveniences for booleans.
impl CBOR {
    /// The CBOR simple value representing `false`.
    pub fn r#false() -> Self { CBORCase::Simple(Simple::False).into() }

    /// The CBOR simple value representing `true`.
    pub fn r#true() -> Self { CBORCase::Simple(Simple::True).into() }

    pub fn as_bool(&self) -> Option<bool> {
        match self.as_case() {
            CBORCase::Simple(Simple::True) => Some(true),
            CBORCase::Simple(Simple::False) => Some(false),
            _ => None,
        }
    }
    /// Extract the CBOR value as a boolean.
    pub fn try_into_bool(self) -> Result<bool> {
        match Self::as_bool(&self) {
            Some(b) => Ok(b),
            None => Err(Error::WrongType),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(
            self.as_case(),
            CBORCase::Simple(Simple::True | Simple::False)
        )
    }

    pub fn try_bool(&self) -> Result<bool> { self.clone().try_into_bool() }

    /// Check if the CBOR value is true.
    pub fn is_true(&self) -> bool {
        matches!(self.as_case(), CBORCase::Simple(Simple::True))
    }

    /// Check if the CBOR value is false.
    pub fn is_false(&self) -> bool {
        matches!(self.as_case(), CBORCase::Simple(Simple::False))
    }
}

/// Conveniences for simple values.
impl CBOR {
    /// Creates a CBOR value representing `null`.
    ///
    /// This is equivalent to the CBOR simple value `null`.
    ///
    /// # Returns
    ///
    /// A CBOR value representing `null`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let null_value = CBOR::null();
    /// assert!(null_value.is_null());
    /// assert_eq!(null_value.diagnostic(), "null");
    /// ```
    pub fn null() -> Self { CBORCase::Simple(Simple::Null).into() }

    /// Checks if the CBOR value is `null`.
    ///
    /// # Returns
    ///
    /// * `true` if the value is the CBOR simple value `null`
    /// * `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let null_value = CBOR::null();
    /// assert!(null_value.is_null());
    ///
    /// let other_value: CBOR = 42u64.into();
    /// assert!(!other_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self.as_case(), CBORCase::Simple(Simple::Null))
    }
}

/// Conveniences for numeric values.
impl CBOR {
    /// Checks if the CBOR value represents a number.
    ///
    /// A CBOR value is considered to be a number if it is:
    /// - An unsigned integer (major type 0)
    /// - A negative integer (major type 1)
    /// - A floating-point value (major type 7, simple values 25, 26, or 27)
    ///
    /// # Returns
    ///
    /// * `true` if the value represents a number
    /// * `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::{CBORCase, Simple, prelude::*};
    ///
    /// let unsigned: CBOR = 42u64.into();
    /// assert!(unsigned.is_number());
    ///
    /// let negative: CBOR = (-42i64).into();
    /// assert!(negative.is_number());
    ///
    /// let float: CBOR = CBORCase::Simple(Simple::Float(3.14)).into();
    /// assert!(float.is_number());
    ///
    /// let text: CBOR = "not a number".into();
    /// assert!(!text.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match self.as_case() {
            CBORCase::Unsigned(_) | CBORCase::Negative(_) => true,
            CBORCase::Simple(s) => s.is_float(),
            _ => false,
        }
    }

    /// Checks if the CBOR value represents the NaN (Not a Number) value.
    ///
    /// # Returns
    ///
    /// * `true` if the value is the CBOR representation of NaN
    /// * `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::{CBORCase, Simple, prelude::*};
    ///
    /// let nan_value = CBOR::nan();
    /// assert!(nan_value.is_nan());
    ///
    /// let float: CBOR = CBORCase::Simple(Simple::Float(3.14)).into();
    /// assert!(!float.is_nan());
    /// ```
    pub fn is_nan(&self) -> bool {
        match self.as_case() {
            CBORCase::Simple(s) => s.is_nan(),
            _ => false,
        }
    }

    /// Creates a CBOR value representing NaN (Not a Number).
    ///
    /// # Returns
    ///
    /// A CBOR value representing NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let nan_value = CBOR::nan();
    /// assert!(nan_value.is_nan());
    /// ```
    pub fn nan() -> Self { CBORCase::Simple(Simple::Float(f64::NAN)).into() }
}
