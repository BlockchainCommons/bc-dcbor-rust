import_stdlib!();

use crate::CBOR;

/// # CBOR Encoding and Decoding Traits
///
/// These traits provide functionality for converting between Rust types and CBOR data.
/// They form the foundation of the dCBOR serialization infrastructure.
///
/// The main traits are:
///
/// - `CBOREncodable`: For types that can be encoded to CBOR
/// - `CBORDecodable`: For types that can be decoded from CBOR
/// - `CBORCodable`: For types that can do both (a combination of the above)
///
/// These traits allow for ergonomic conversions using Rust's type system and
/// enable seamless integration with dCBOR's deterministic encoding rules.
/// A trait for types that can be encoded to CBOR.
///
/// This trait is automatically implemented for any type that implements `Into<CBOR>` and `Clone`.
/// It provides convenient methods for converting instances into CBOR objects and binary data.
///
/// ## Example
///
/// ```
/// use dcbor::prelude::*;
///
/// // Custom type that implements Into<CBOR>
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// // Implement conversion to CBOR
/// impl From<Person> for CBOR {
///     fn from(person: Person) -> Self {
///         let mut map = Map::new();
///         map.insert("name", person.name);
///         map.insert("age", person.age);
///         map.into()
///     }
/// }
///
/// // The CBOREncodable trait is automatically implemented
/// let person = Person { name: "Alice".to_string(), age: 30 };
///
/// // Convert to CBOR with to_cbor()
/// let cbor = person.to_cbor();
///
/// // Convert directly to binary CBOR data
/// let data = person.to_cbor_data();
/// ```
pub trait CBOREncodable: Into<CBOR> + Clone {
    /// Converts this value to a CBOR object.
    ///
    /// This is a convenience method that avoids consuming the original value.
    fn to_cbor(&self) -> CBOR {
        self.clone().into()
    }

    /// Converts this value directly to binary CBOR data.
    ///
    /// This is a shorthand for `self.to_cbor().to_cbor_data()`.
    fn to_cbor_data(&self) -> Vec<u8> {
        self.to_cbor().to_cbor_data()
    }
}

impl<T> CBOREncodable for T where T: Into<CBOR> + Clone { }

/// A trait for types that can be decoded from CBOR.
///
/// This trait is automatically implemented for any type that implements `TryFrom<CBOR>`.
/// It serves as a marker trait to indicate that a type supports being created from CBOR data.
///
/// ## Example
///
/// ```no_run
/// use dcbor::prelude::*;
///
/// // Custom type that implements TryFrom<CBOR>
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// // Implement conversion from CBOR
/// impl TryFrom<CBOR> for Person {
///     type Error = dcbor::Error;
///
///     fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
///         if let CBORCase::Map(map) = cbor.into_case() {
///             let name: String = map.extract("name")?;
///             let age: u8 = map.extract("age")?;
///             Ok(Person { name, age })
///         } else {
///             Err("Expected a CBOR map".into())
///         }
///     }
/// }
///
/// // The CBORDecodable trait is automatically implemented
/// // Convert a CBOR object to our type
///
/// // Create a sample CBOR map
/// let mut map = Map::new();
/// map.insert("name", "Alice");
/// map.insert("age", 42);
/// let cbor = map.to_cbor();
///
/// // Parse from CBOR to our type
/// let person: Person = cbor.try_into().unwrap();
/// ```
pub trait CBORDecodable: TryFrom<CBOR, Error = crate::Error> {
    fn try_from_cbor(cbor: &CBOR) -> crate::Result<Self> {
        Self::try_from(cbor.clone())
    }
}

impl<T> CBORDecodable for T where T: TryFrom<CBOR, Error = crate::Error> { }

/// A trait for types that can be both encoded to and decoded from CBOR.
///
/// This trait is automatically implemented for any type that implements both
/// `CBOREncodable` and `CBORDecodable`. It serves as a convenience marker trait
/// to indicate full CBOR serialization support.
///
/// ## Example
///
/// ```
/// use dcbor::prelude::*;
///
/// // Custom type that implements both conversion directions
/// #[derive(Clone)]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// // Implement conversion to CBOR
/// impl From<Person> for CBOR {
///     fn from(person: Person) -> Self {
///         let mut map = Map::new();
///         map.insert("name", person.name);
///         map.insert("age", person.age);
///         map.into()
///     }
/// }
///
/// // Implement conversion from CBOR
/// impl TryFrom<CBOR> for Person {
///     type Error = dcbor::Error;
///
///     fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
///         if let CBORCase::Map(map) = cbor.into_case() {
///             let name: String = map.extract("name")?;
///             let age: u8 = map.extract("age")?;
///             Ok(Person { name, age })
///         } else {
///             Err("Expected a CBOR map".into())
///         }
///     }
/// }
///
/// // Person now automatically implements CBORCodable
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// let cbor = person.to_cbor(); // Using CBOREncodable
///
/// // Create a round-trip copy
/// let person_copy: Person = cbor.try_into().unwrap(); // Using CBORDecodable
/// ```
pub trait CBORCodable { }

impl<T> CBORCodable for T where T: CBORDecodable + CBOREncodable { }
