import_stdlib!();

#[derive(Debug, Clone)]
enum TagName {
    Static(&'static str),
    Dynamic(String),
}

pub type TagValue = u64;

/// A CBOR tag.
#[derive(Debug, Clone)]
pub struct Tag {
    value: TagValue,
    name: Option<TagName>,
}

impl Tag {
    /// Creates a new CBOR tag with the given value and associated name.
    pub fn new(value: TagValue, name: impl Into<String>) -> Tag {
        Self { value, name: Some(TagName::Dynamic(name.into())) }
    }

    /// Creates a new CBOR tag with the given value and no name.
    pub const fn with_value(value: TagValue) -> Tag {
        Self { value, name: None }
    }

    /// Creates a new CBOR tag at compile time with the given value and associated name.
    pub const fn with_static_name(value: TagValue, name: &'static str) -> Tag {
        Self { value, name: Some(TagName::Static(name)) }
    }

    /// Returns the wrapped tag value.
    pub fn value(&self) -> TagValue {
        self.value
    }

    /// Returns the tag's associated name, if any.
    pub fn name(&self) -> Option<String> {
        match &self.name {
            Some(TagName::Static(name)) => Some(name.to_string()),
            Some(TagName::Dynamic(name)) => Some(name.clone()),
            None => None,
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Tag { }

impl hash::Hash for Tag {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(TagName::Static(name)) => write!(f, "{}", name),
            Some(TagName::Dynamic(name)) => write!(f, "{}", name),
            None => write!(f, "{}", self.value),
        }
    }
}

impl From<TagValue> for Tag {
    fn from(value: TagValue) -> Self {
        Tag::with_value(value)
    }
}

impl From<i32> for Tag {
    fn from(value: i32) -> Self {
        Tag::with_value(value as TagValue)
    }
}

impl From<usize> for Tag {
    fn from(value: usize) -> Self {
        Tag::with_value(value as TagValue)
    }
}
