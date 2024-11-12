use std::sync::{ Once, Mutex };

use crate::{CBORTaggedDecodable, Date, Tag, TagValue, TagsStore, TagsStoreTrait};

#[derive(Debug)]
pub struct LazyTagsStore {
    init: Once,
    data: Mutex<Option<TagsStore>>,
}

impl LazyTagsStore {
    pub fn get(&self) -> std::sync::MutexGuard<'_, Option<TagsStore>> {
        self.init.call_once(|| {
            let m = TagsStore::new([]);
            *self.data.lock().unwrap() = Some(m);
        });
        self.data.lock().unwrap()
    }
}

pub static GLOBAL_TAGS: LazyTagsStore = LazyTagsStore {
    init: Once::new(),
    data: Mutex::new(None),
};

/// A macro for accessing the global tags store.
#[macro_export]
macro_rules! with_tags {
    ($action:expr) => {
        {
        let binding = $crate::GLOBAL_TAGS.get();
        let tags = binding.as_ref().unwrap();
        #[allow(clippy::redundant_closure_call)]
        $action(tags)
        }
    };
}

/// A macro for accessing the global tags store mutably.
#[macro_export]
macro_rules! with_tags_mut {
    ($action:expr) => {
        {
        let mut binding = $crate::GLOBAL_TAGS.get();
        let tags = binding.as_mut().unwrap();
        #[allow(clippy::redundant_closure_call)]
        $action(tags)
        }
    };
}

pub const TAG_DATE: TagValue = 1;

pub fn register_tags() {
    let tags = vec![
        (TAG_DATE, "date"),
    ];
    with_tags_mut!(|tags_store: &mut TagsStore| {
        for tag in tags.into_iter() {
            tags_store.insert(Tag::new(tag.0, tag.1));
        }
        tags_store.set_summarizer(TAG_DATE, |value| {
            Ok(format!("{}", Date::from_untagged_cbor(value)?))
        });
    });
}

pub fn tags_for_values(values: &[TagValue]) -> Vec<Tag> {
    with_tags!(|tags: &TagsStore| {
        values.iter().map(|value| tags.tag_for_value(*value).unwrap_or_else(|| Tag::with_value(*value))).collect()
    })
}
