use std::hash::Hash;

#[derive(Debug, Clone)]
enum TagName {
    Static(&'static str),
    Dynamic(String),
}

/// A CBOR tag.
#[derive(Debug, Clone)]
pub struct Tag {
    value: u64,
    name: Option<TagName>,
}

impl Tag {
    /// Creates a new CBOR tag with the given value and no name.
    pub const fn new(value: u64) -> Tag {
        Tag { value, name: None }
    }

    /// Creates a new CBOR tag with the given value and an optional associated name.
    pub fn new_with_name<T: Into<String>>(value: u64, name: T) -> Tag {
        Tag { value, name: Some(TagName::Dynamic(name.into())) }
    }

    pub const fn new_with_static_name(value: u64, name: &'static str) -> Tag {
        Tag { value, name: Some(TagName::Static(name)) }
    }

    /// Returns the wrapped tag value.
    pub fn value(&self) -> u64 {
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

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// impl std::fmt::Debug for Tag {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(name) = self.name() {
//             return f.write_str(&format!("Tag(value: {} name: {})", self.value, name));
//         } else {
//             return f.write_str(&format!("Tag(value: {})", self.value));
//         }
//     }
// }

// impl Clone for Tag {
//     fn clone(&self) -> Self {
//         Tag { value: self.value, name: self.name.clone() }
//     }
// }

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
