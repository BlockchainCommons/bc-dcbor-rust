import_stdlib!();

/// Internal representation of a tag name, with optimization for statically known names.
#[derive(Debug, Clone)]
enum TagName {
    /// A tag name that's known at compile time, avoiding allocation.
    Static(&'static str),

    /// A dynamically created tag name, requiring heap allocation.
    Dynamic(String),
}

/// Represents the numeric value of a CBOR tag.
///
/// In CBOR, tags (major type 6) are identified by unsigned integer values.
/// Per RFC 8949, tag values are registered with IANA to ensure interoperability.
///
/// # Examples of standard CBOR tag values
///
/// - 0: RFC3339 date/time string
/// - 1: Epoch-based date/time
/// - 2: Positive bignum
/// - 3: Negative bignum
/// - 4: Decimal fraction
/// - 5: Bigfloat
/// - 18: CBOR data item (tagged data is embedded CBOR)
/// - 32: URI
/// - 201: dCBOR tag (per the dCBOR specification)
///
/// For the full registry of tag values, see the IANA CBOR Tags registry:
/// <https://www.iana.org/assignments/cbor-tags/>
pub type TagValue = u64;

/// Represents a CBOR tag (major type 6) with optional associated name.
///
/// Tags in CBOR provide semantic information about the tagged data item.
/// They are used to indicate that the data item has some additional semantics
/// beyond its basic CBOR type. For example, a tag might indicate that a string
/// should be interpreted as a date, or that a byte string contains embedded CBOR.
///
/// This implementation supports both the numeric tag value and an optional
/// human-readable name to improve code clarity and debugging.
///
/// # Tag equality and comparison
///
/// Tags are considered equal if their numeric values are equal, regardless of their names.
/// This matches the CBOR specification behavior where the tag value (not the name)
/// determines the semantic meaning.
///
/// # Deterministic encoding
///
/// Tags, like all other CBOR types, must follow deterministic encoding rules in dCBOR.
/// The tag value is encoded according to the general integer encoding rules (shortest
/// form possible), and the tagged content itself must also follow deterministic
/// encoding rules.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create a tag with a name
/// let epoch_time_tag = Tag::new(1, "epoch-time");
/// assert_eq!(epoch_time_tag.value(), 1);
/// assert_eq!(epoch_time_tag.name(), Some("epoch-time".to_string()));
///
/// // Create a tag without a name
/// let unnamed_tag = Tag::with_value(42);
/// assert_eq!(unnamed_tag.value(), 42);
/// assert_eq!(unnamed_tag.name(), None);
///
/// // Create a tag at compile time with a static name
/// const REGISTERED_TAG: Tag = Tag::with_static_name(32, "uri");
/// assert_eq!(REGISTERED_TAG.value(), 32);
/// assert_eq!(REGISTERED_TAG.name(), Some("uri".to_string()));
///
/// // Tags with the same value are equal, even with different names
/// let tag1 = Tag::new(42, "meaning");
/// let tag2 = Tag::with_value(42);
/// assert_eq!(tag1, tag2);
/// ```
#[derive(Debug, Clone)]
pub struct Tag {
    /// The numeric value of the tag.
    value: TagValue,

    /// Optional human-readable name for the tag.
    name: Option<TagName>,
}

impl Tag {
    /// Creates a new CBOR tag with the given value and associated name.
    ///
    /// This constructor allocates memory for the tag name, storing it as a dynamic string.
    /// If the tag name is known at compile time, consider using `with_static_name` instead.
    ///
    /// # Parameters
    ///
    /// * `value` - The numeric tag value, typically registered with IANA
    /// * `name` - A human-readable name for the tag, improving code readability
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let date_tag = Tag::new(12345, "This is a tagged string.");
    /// assert_eq!(date_tag.value(), 12345);
    /// assert_eq!(date_tag.name(), Some("This is a tagged string.".to_string()));
    /// ```
    pub fn new(value: TagValue, name: impl Into<String>) -> Tag {
        Self { value, name: Some(TagName::Dynamic(name.into())) }
    }

    /// Creates a new CBOR tag with the given value and no name.
    ///
    /// This constructor creates an unnamed tag, which is sufficient for many use cases
    /// but may be less readable in code compared to named tags.
    ///
    /// # Parameters
    ///
    /// * `value` - The numeric tag value, typically registered with IANA
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let tag = Tag::with_value(42);
    /// assert_eq!(tag.value(), 42);
    /// assert_eq!(tag.name(), None);
    /// ```
    pub const fn with_value(value: TagValue) -> Tag {
        Self { value, name: None }
    }

    /// Creates a new CBOR tag at compile time with the given value and associated name.
    ///
    /// This constructor is optimized for cases where the tag name is known at compile time,
    /// avoiding runtime allocations for the name string. It can be used in `const` contexts.
    ///
    /// # Parameters
    ///
    /// * `value` - The numeric tag value, typically registered with IANA
    /// * `name` - A static string literal as the human-readable name for the tag
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// const DATE_TAG: Tag = Tag::with_static_name(0, "date-time-string");
    /// assert_eq!(DATE_TAG.value(), 0);
    /// assert_eq!(DATE_TAG.name(), Some("date-time-string".to_string()));
    /// ```
    pub const fn with_static_name(value: TagValue, name: &'static str) -> Tag {
        Self { value, name: Some(TagName::Static(name)) }
    }

    /// Returns the numeric value of the tag.
    ///
    /// The tag value is the primary identifier for a tag in CBOR and determines
    /// its semantic meaning according to the IANA CBOR Tags registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let tag = Tag::new(18, "cbor-data-item");
    /// assert_eq!(tag.value(), 18);
    /// ```
    pub fn value(&self) -> TagValue {
        self.value
    }

    /// Returns the tag's associated human-readable name, if any.
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The tag's name if it has one
    /// * `None` - If the tag was created without a name
    ///
    /// # Examples
    ///
    /// ```
    /// use dcbor::prelude::*;
    ///
    /// let named_tag = Tag::new(32, "uri");
    /// assert_eq!(named_tag.name(), Some("uri".to_string()));
    ///
    /// let unnamed_tag = Tag::with_value(32);
    /// assert_eq!(unnamed_tag.name(), None);
    /// ```
    pub fn name(&self) -> Option<String> {
        match &self.name {
            Some(TagName::Static(name)) => Some(name.to_string()),
            Some(TagName::Dynamic(name)) => Some(name.clone()),
            None => None,
        }
    }
}

/// Compares tags for equality based on their numeric values.
///
/// Tags are considered equal if they have the same numeric value, regardless of
/// whether they have different names or if one has a name and the other doesn't.
/// This matches the CBOR standard behavior where the tag value, not its human-readable
/// name, determines its semantic meaning.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let tag1 = Tag::new(32, "uri");
/// let tag2 = Tag::with_value(32);
/// let tag3 = Tag::new(32, "different-name");
/// let tag4 = Tag::with_value(42);
///
/// assert_eq!(tag1, tag2);       // Same value, one named and one unnamed
/// assert_eq!(tag1, tag3);       // Same value, different names
/// assert_ne!(tag1, tag4);       // Different values
/// ```
impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Confirms `Tag` implements full equality, not just partial equality.
///
/// This is required for `Tag` to be used in collections like `HashSet` and
/// as keys in `HashMap` along with the `Hash` implementation.
impl Eq for Tag { }

/// Implements hashing for `Tag` based solely on the numeric tag value.
///
/// This implementation ensures that two tags with the same value but different
/// names will hash to the same value, which is consistent with the equality
/// implementation. This allows tags to be used as keys in hash-based collections.
impl hash::Hash for Tag {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Formats a tag for display, preferring the name if available.
///
/// If the tag has a name, the name will be displayed. Otherwise, the numeric
/// value will be displayed.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let named_tag = Tag::new(32, "uri");
/// let unnamed_tag = Tag::with_value(42);
///
/// assert_eq!(named_tag.to_string(), "uri");
/// assert_eq!(unnamed_tag.to_string(), "42");
/// ```
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(TagName::Static(name)) => write!(f, "{}", name),
            Some(TagName::Dynamic(name)) => write!(f, "{}", name),
            None => write!(f, "{}", self.value),
        }
    }
}

/// Converts a raw tag value to a `Tag` instance.
///
/// This provides a convenient way to create unnamed tags directly from numeric values.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // These are equivalent:
/// let tag1: Tag = 42.into();
/// let tag2 = Tag::with_value(42);
///
/// assert_eq!(tag1, tag2);
/// ```
impl From<TagValue> for Tag {
    fn from(value: TagValue) -> Self {
        Tag::with_value(value)
    }
}

/// Converts an `i32` integer to a `Tag` instance.
///
/// This provides a convenient way to create unnamed tags from 32-bit signed integers.
/// Note that the value will be converted to an unsigned 64-bit integer internally.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let tag: Tag = 42i32.into();
/// assert_eq!(tag.value(), 42);
/// ```
impl From<i32> for Tag {
    fn from(value: i32) -> Self {
        Tag::with_value(value as TagValue)
    }
}

/// Converts a `usize` to a `Tag` instance.
///
/// This provides a convenient way to create unnamed tags from platform-specific
/// sized unsigned integers. Note that on platforms where `usize` is larger than
/// 64 bits, values that don't fit in 64 bits will be truncated.
///
/// # Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// let tag: Tag = 42usize.into();
/// assert_eq!(tag.value(), 42);
/// ```
impl From<usize> for Tag {
    fn from(value: usize) -> Self {
        Tag::with_value(value as TagValue)
    }
}
