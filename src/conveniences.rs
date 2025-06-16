use crate::{CBOR, CBORCase, Error, Map, Result, Simple, tag::Tag};

/// Conveniences for byte strings.
impl CBOR {
    /// Creates a new CBOR value representing a byte string.
    ///
    /// This method creates a CBOR byte string (major type 2) from any type that
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

    pub fn is_byte_string(cbor: &Self) -> bool {
        matches!(cbor.as_case(), CBORCase::ByteString(_))
    }

    pub fn into_byte_string(self) -> Option<Vec<u8>> {
        self.try_into_byte_string().ok()
    }

    pub fn try_byte_string(cbor: &Self) -> Result<Vec<u8>> {
        cbor.clone().try_into_byte_string()
    }

    pub fn as_byte_string(cbor: &Self) -> Option<&[u8]> {
        match cbor.as_case() {
            CBORCase::ByteString(b) => Some(b),
            _ => None,
        }
    }
}

/// Conveniences for tagged values.
impl CBOR {
    /// Creates a new CBOR value representing a tagged value.
    ///
    /// This method creates a CBOR tagged value (major type 6) by applying a tag
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

    pub fn is_tagged_value(cbor: &Self) -> bool {
        matches!(cbor.as_case(), CBORCase::Tagged(_, _))
    }

    pub fn as_tagged_value(cbor: &Self) -> Option<(&Tag, &CBOR)> {
        match cbor.as_case() {
            CBORCase::Tagged(tag, value) => Some((tag, value)),
            _ => None,
        }
    }

    pub fn try_tagged_value(cbor: &Self) -> Result<(Tag, CBOR)> {
        cbor.clone().try_into_tagged_value()
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

    pub fn try_expected_tagged_value(
        cbor: &Self,
        expected_tag: impl Into<Tag>,
    ) -> Result<CBOR> {
        cbor.clone().try_into_expected_tagged_value(expected_tag)
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

    pub fn is_text(cbor: &Self) -> bool {
        matches!(cbor.as_case(), CBORCase::Text(_))
    }

    pub fn try_text(cbor: &Self) -> Result<String> {
        cbor.clone().try_into_text()
    }

    pub fn into_text(self) -> Option<String> { self.try_into_text().ok() }

    pub fn as_text(cbor: &Self) -> Option<&str> {
        match cbor.as_case() {
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

    pub fn is_array(cbor: &Self) -> bool {
        matches!(cbor.as_case(), CBORCase::Array(_))
    }

    pub fn try_array(cbor: &Self) -> Result<Vec<CBOR>> {
        cbor.clone().try_into_array()
    }

    pub fn into_array(self) -> Option<Vec<CBOR>> { self.try_into_array().ok() }

    pub fn as_array(cbor: &Self) -> Option<&[CBOR]> {
        match cbor.as_case() {
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

    pub fn is_map(cbor: &Self) -> bool {
        matches!(cbor.as_case(), CBORCase::Map(_))
    }

    pub fn try_map(cbor: &Self) -> Result<Map> { cbor.clone().try_into_map() }

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

    pub fn as_map(cbor: &Self) -> Option<&Map> {
        match cbor.as_case() {
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

    pub fn as_bool(cbor: &Self) -> Option<bool> {
        match cbor.as_case() {
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

    pub fn is_bool(cbor: &Self) -> bool {
        matches!(
            cbor.as_case(),
            CBORCase::Simple(Simple::True | Simple::False)
        )
    }

    pub fn try_bool(cbor: &Self) -> Result<bool> {
        cbor.clone().try_into_bool()
    }

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
    /// The CBOR simple value representing `null` (`None`).
    pub fn null() -> Self { CBORCase::Simple(Simple::Null).into() }

    /// Check if the CBOR value is null.
    pub fn is_null(&self) -> bool {
        matches!(self.as_case(), CBORCase::Simple(Simple::Null))
    }
}

/// Conveniences for numeric values.
impl CBOR {
    /// Check if the CBOR value is a number.
    pub fn is_number(&self) -> bool {
        match self.as_case() {
            CBORCase::Unsigned(_) | CBORCase::Negative(_) => true,
            CBORCase::Simple(s) => s.is_float(),
            _ => false,
        }
    }

    /// Check if the CBOR value is the NaN (Not a Number) representation.
    pub fn is_nan(&self) -> bool {
        match self.as_case() {
            CBORCase::Simple(s) => s.is_nan(),
            _ => false,
        }
    }

    /// The CBOR simple value representing `NaN` (Not a Number).
    pub fn nan() -> Self { CBORCase::Simple(Simple::Float(f64::NAN)).into() }
}
