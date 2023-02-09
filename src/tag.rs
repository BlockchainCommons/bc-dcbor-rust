use std::hash::Hash;

/// A CBOR tag.
#[derive(Debug, Clone)]
pub struct Tag {
    value: u64,
    name: Option<String>,
}

impl Tag {
    /// Creates a new CBOR tag with the given value and an optional associated name.
    pub fn new_opt(value: u64, name: Option<&str>) -> Tag {
        let name = match name {
            None => None,
            Some(name) => Some(name.to_string())
        };
        Tag { value, name }
    }

    /// Creates a new CBOR tag with the given value.
    pub const fn new(value: u64) -> Tag {
        Tag { value, name: None }
    }

    /// Returns the wrapped tag value.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Returns the tag's associated name, if any.
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Tag { }

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().clone().unwrap_or_else(|| self.value().to_string()))
    }
}

impl From<u64> for Tag {
    fn from(value: u64) -> Self {
        Tag::new(value)
    }
}

impl From<i32> for Tag {
    fn from(value: i32) -> Self {
        Tag::new(value as u64)
    }
}

impl From<usize> for Tag {
    fn from(value: usize) -> Self {
        Tag::new(value as u64)
    }
}
