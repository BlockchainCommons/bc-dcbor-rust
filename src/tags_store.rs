import_stdlib!();

use crate::{Tag, TagValue, CBOR};

pub type CBORSummarizer = Arc<dyn Fn(CBOR) -> anyhow::Result<String> + Send + Sync>;

/// A type that can map between tags and their names.
pub trait TagsStoreTrait {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String>;
    fn name_for_tag(&self, tag: &Tag) -> String;
    fn tag_for_value(&self, value: u64) -> Option<Tag>;
    fn tag_for_name(&self, name: &str) -> Option<Tag>;
    fn name_for_value(&self, value: u64) -> String;
    fn summarizer(&self, tag: TagValue) -> Option<&CBORSummarizer>;

    fn name_for_tag_opt<T>(tag: &Tag, tags: Option<&T>) -> String where T: TagsStoreTrait, Self: Sized {
        match tags {
            None => tag.value().to_string(),
            Some(tags) => tags.name_for_tag(tag)
        }
    }
}

/// A dictionary of mappings between tags and their names.
#[derive(Clone)]
pub struct TagsStore {
    tags_by_value: HashMap<u64, Tag>,
    tags_by_name: HashMap<String, Tag>,
    summarizers: HashMap<u64, CBORSummarizer>,
}

impl TagsStore {
    pub fn new<T>(tags: T) -> Self where T: IntoIterator<Item=Tag> {
        let mut tags_by_value = HashMap::new();
        let mut tags_by_name = HashMap::new();
        for tag in tags {
            Self::_insert(tag, &mut tags_by_value, &mut tags_by_name);
        }
        Self {
            tags_by_value,
            tags_by_name,
            summarizers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tag: Tag) {
        Self::_insert(tag, &mut self.tags_by_value, &mut self.tags_by_name);
    }

    pub fn set_summarizer(&mut self, tag: TagValue, summarizer: CBORSummarizer) {
        self.summarizers.insert(tag, summarizer);
    }

    fn _insert(tag: Tag, tags_by_value: &mut HashMap<u64, Tag>, tags_by_name: &mut HashMap<String, Tag>) {
        let name = tag.name().unwrap();
        assert!(!name.is_empty());
        let result = tags_by_value.insert(tag.value(), tag.clone());
        if let Some(old_value) = result {
            // if the names don't match, we have a problem
            let old_name = old_value.name().unwrap();
            if old_name != name {
                panic!("Attempt to register tag: {} '{}' with different name: '{}'", tag.value(), old_name, name);
            }
        }
        tags_by_name.insert(name, tag);
    }
}

impl TagsStoreTrait for TagsStore {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String> {
        self.tag_for_value(tag.value()).map(|tag| tag.name().unwrap())
    }

    fn name_for_tag(&self, tag: &Tag) -> String {
        self.assigned_name_for_tag(tag).unwrap_or_else(|| tag.value().to_string())
    }

    fn tag_for_name(&self, name: &str) -> Option<Tag> {
        self.tags_by_name.get(name).cloned()
    }

    fn tag_for_value(&self, value: u64) -> Option<Tag> {
        self.tags_by_value.get(&value).cloned()
    }

    fn name_for_value(&self, value: u64) -> String {
        self.tag_for_value(value)
            .and_then(|tag| tag.name())
            .unwrap_or_else(|| value.to_string())
    }

    fn summarizer(&self, tag: TagValue) -> Option<&CBORSummarizer> {
        self.summarizers.get(&tag)
    }
}

impl Default for TagsStore {
    fn default() -> Self {
        Self::new([])
    }
}
