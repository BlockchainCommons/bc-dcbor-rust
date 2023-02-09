use std::collections::HashMap;

use crate::Tag;

/// A type that can return the name for a tag.
pub trait KnownTags {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String>;
    fn name_for_tag(&self, tag: &Tag) -> String;

    fn name_for_tag_opt<T>(tag: &Tag, known_tags: Option<&T>) -> String where T: KnownTags, Self: Sized {
        match known_tags {
            None => tag.value().to_string(),
            Some(tags) => tags.name_for_tag(tag)
        }
    }
}

/// A concrete type that maps from tags to their known names.
///
/// Higher-level libraries may implement their own concrete types similar to or
/// incorporating this type and implementing the `KnownTags` trait, in which
/// case those types can also be passed to the methods that format CBOR as text.
pub struct KnownTagsDict(HashMap<Tag, String>);

impl KnownTagsDict {
    /// Creates a new, empty `KnownTagsDict`.
    pub fn new() -> KnownTagsDict {
        KnownTagsDict(HashMap::new())
    }

    /// Creates a new `KnownTagsDict` from the provided array of tags.
    ///
    /// Each of the provided tags must have an assigned name, or this function
    /// panics.
    pub fn from_tags(tags: &[Tag]) -> KnownTagsDict {
        let mut dict: HashMap<Tag, String> = HashMap::new();
        for tag in tags {
            Self::_insert(tag, &mut dict);
        }
        KnownTagsDict(dict)
    }

    /// Inserts a new `Tag` into the dictionary.
    ///
    /// The provided `Tag` must have an assigned name, or this function panics.
    pub fn insert(&mut self, tag: &Tag) {
        Self::_insert(tag, &mut self.0);
    }

    fn _insert(tag: &Tag, dict: &mut HashMap<Tag, String>) {
        let name = tag.name().to_owned().unwrap();
        assert!(!name.is_empty());
        dict.insert(tag.clone(), name);
    }
}

impl KnownTags for KnownTagsDict {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String> {
        return match self.0.get(tag) {
            None => None,
            Some(name) => Some(name.to_string())
        }
    }
    fn name_for_tag(&self, tag: &Tag) -> String {
        self.assigned_name_for_tag(tag).unwrap_or_else(|| tag.value().to_string())
    }
}
