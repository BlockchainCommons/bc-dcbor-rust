use std::collections::HashMap;

use crate::Tag;

/// A type that can return the name for a tag.
pub trait KnownTags {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String>;
    fn name_for_tag(&self, tag: &Tag) -> String;
}

pub fn name_for_tag<T>(tag: &Tag, known_tags: Option<&T>) -> String where T: KnownTags {
    match known_tags {
        None => tag.value().to_string(),
        Some(tags) => tags.name_for_tag(tag)
    }
}

pub struct KnownTagsDict(HashMap<Tag, String>);

impl KnownTagsDict {
    pub fn new() -> KnownTagsDict {
        KnownTagsDict(HashMap::new())
    }

    pub fn from_tags(tags: &[Tag]) -> KnownTagsDict {
        let mut dict: HashMap<Tag, String> = HashMap::new();
        for tag in tags {
            Self::_insert(tag, &mut dict);
        }
        KnownTagsDict(dict)
    }

    pub fn insert(&mut self, tag: &Tag) {
        Self::_insert(tag, &mut self.0);
    }

    pub fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String> {
        return match self.0.get(tag) {
            None => None,
            Some(name) => Some(name.to_string())
        }
    }

    pub fn name_for_tag(&self, tag: &Tag) -> String {
        self.assigned_name_for_tag(tag).unwrap_or_else(|| tag.value().to_string())
    }

    fn _insert(tag: &Tag, dict: &mut HashMap<Tag, String>) {
        let name = tag.name().to_owned().unwrap();
        assert!(!name.is_empty());
        dict.insert(tag.clone(), name);
    }
}
