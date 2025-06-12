import_stdlib!();

use crate::{ Tag, TagValue, CBOR, Result };

/// A function type for summarizing CBOR values as human-readable strings.
///
/// `CBORSummarizer` is used to create custom formatters for tagged CBOR values.
/// It's a type alias for a thread-safe function pointer that takes a CBOR value
/// and returns a string representation or an error.
///
/// ## Purpose
///
/// In CBOR, tags provide semantic meaning to data structures. A `CBORSummarizer`
/// helps interpret and display these tagged values in a human-readable format.
/// This is particularly useful for debugging, logging, or displaying CBOR data
/// to users.
///
/// ## Thread Safety
///
/// The `CBORSummarizer` type is wrapped in an `Arc` (Atomic Reference Count) and
/// requires `Send + Sync` traits, making it safe to share between threads.
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
/// use std::sync::Arc;
///
/// // Create a custom summarizer for a date tag
/// let date_summarizer: CBORSummarizer = Arc::new(|cbor, _flat| {
///     // Extract timestamp from tagged CBOR
///     let timestamp: f64 = cbor.clone().try_into()?;
///
///     // Format timestamp as ISO date (simplified example)
///     Ok(format!("Date: {:.1} seconds since epoch", timestamp))
/// });
///
/// // Create a tags store
/// let mut tags = TagsStore::default();
///
/// // Register a tag for date (tag 1 is the standard CBOR tag for dates)
/// tags.insert(Tag::new(1, "date".to_string()));
///
/// // Register our summarizer for tag 1
/// tags.set_summarizer(1, date_summarizer);
/// ```
///
/// When this summarizer is used (for example in diagnostic output), it would
/// convert a tagged CBOR timestamp into a more readable date format.
pub type CBORSummarizer = Arc<dyn (Fn(CBOR, bool) -> Result<String>) + Send + Sync>;

/// A trait for types that can map between CBOR tags and their human-readable names.
///
/// The `TagsStoreTrait` provides a standardized interface for resolving CBOR tags
/// to human-readable names and vice versa. This is useful for debugging and
/// displaying CBOR data in a more understandable format.
///
/// ## Functionality
///
/// This trait defines methods for:
/// - Converting between tag numbers and human-readable names
/// - Looking up tags by name or value
/// - Retrieving summarizers for specific tags
///
/// ## Implementation
///
/// Implementers of this trait should maintain a bidirectional mapping between
/// tag values (numbers) and their corresponding names. The primary implementation
/// is [`TagsStore`], but other implementations can be created for specific needs.
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create a store that implements TagsStoreTrait
/// let mut tags = TagsStore::default();
///
/// // Register a tag with a human-readable name
/// tags.insert(Tag::new(1, "date".to_string()));
///
/// // Look up tag by number
/// let tag_name = tags.name_for_value(1);
/// assert_eq!(tag_name, "date");
///
/// // Look up tag by name
/// let tag = tags.tag_for_name("date").unwrap();
/// assert_eq!(tag.value(), 1);
/// ```
///
/// The trait also includes a helper method to handle the common case of optional tag stores.
pub trait TagsStoreTrait {
    fn assigned_name_for_tag(&self, tag: &Tag) -> Option<String>;
    fn name_for_tag(&self, tag: &Tag) -> String;
    fn tag_for_value(&self, value: TagValue) -> Option<Tag>;
    fn tag_for_name(&self, name: &str) -> Option<Tag>;
    fn name_for_value(&self, value: TagValue) -> String;
    fn summarizer(&self, tag: TagValue) -> Option<&CBORSummarizer>;

    fn name_for_tag_opt<T>(tag: &Tag, tags: Option<&T>) -> String
        where T: TagsStoreTrait, Self: Sized
    {
        match tags {
            None => tag.value().to_string(),
            Some(tags) => tags.name_for_tag(tag),
        }
    }
}

#[derive(Clone, Default)]
pub enum TagsStoreOpt<'a> {
    None,
    #[default]
    Global,
    Custom(&'a dyn TagsStoreTrait),
}


/// A registry that maintains mappings between CBOR tags, their human-readable names,
/// and optional summarizers.
///
/// The `TagsStore` is the primary implementation of the [`TagsStoreTrait`], providing
/// a bidirectional mapping between CBOR tag values (numbers) and their human-readable
/// names. It also supports registering custom summarizers for specific tags.
///
/// ## Use Cases
///
/// The `TagsStore` serves several important purposes:
///
/// 1. **Readability**: Converting numeric tags to meaningful names in diagnostic output
/// 2. **Consistency**: Ensuring consistent tag usage throughout an application
/// 3. **Documentation**: Providing a central registry of all tags used in a system
/// 4. **Customization**: Supporting custom summarization of tagged values
///
/// ## Features
///
/// - Bidirectional lookup between tag values and names
/// - Prevention of duplicate registrations with conflicting names
/// - Optional registration of custom summarizers for specific tags
/// - Default implementation for an empty registry
///
/// ## Examples
///
/// ### Basic usage
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create a new TagsStore with a set of predefined tags
/// let mut tags = TagsStore::new([
///     Tag::new(1, "date".to_string()),
///     Tag::new(2, "positive_bignum".to_string()),
///     Tag::new(3, "negative_bignum".to_string())
/// ]);
///
/// // Look up a tag by its value
/// let date_tag = tags.tag_for_value(1).unwrap();
/// assert_eq!(date_tag.name().unwrap(), "date");
///
/// // Look up a tag name by value
/// let name = tags.name_for_value(2);
/// assert_eq!(name, "positive_bignum");
///
/// // Look up a tag by its name
/// let neg_tag = tags.tag_for_name("negative_bignum").unwrap();
/// assert_eq!(neg_tag.value(), 3);
/// ```
///
/// ### Adding summarizers
///
/// ```
/// use dcbor::prelude::*;
/// use std::sync::Arc;
///
/// // Create an empty tags store
/// let mut tags = TagsStore::default();
///
/// // Register a tag
/// tags.insert(Tag::new(1, "date".to_string()));
///
/// // Add a summarizer for the date tag
/// tags.set_summarizer(1, Arc::new(|cbor, _flat| {
///     // Try to convert CBOR to f64 for timestamp formatting
///     let timestamp: f64 = cbor.clone().try_into().unwrap_or(0.0);
///     Ok(format!("Timestamp: {}", timestamp))
/// }));
///
/// // Later, this summarizer can be retrieved and used
/// let date_summarizer = tags.summarizer(1);
/// assert!(date_summarizer.is_some());
/// ```
///
/// ## Implementation Notes
///
/// The `TagsStore` prevents registering the same tag value with different names,
/// which helps maintain consistency in CBOR tag usage. Attempting to register a
/// tag value that already exists with a different name will panic.
#[derive(Clone)]
pub struct TagsStore {
    tags_by_value: HashMap<TagValue, Tag>,
    tags_by_name: HashMap<String, Tag>,
    summarizers: HashMap<TagValue, CBORSummarizer>,
}

impl TagsStore {
    pub fn new<T>(tags: T) -> Self where T: IntoIterator<Item = Tag> {
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

    pub fn insert_all(&mut self, tags: Vec<Tag>) {
        for tag in tags {
            Self::_insert(tag, &mut self.tags_by_value, &mut self.tags_by_name);
        }
    }

    pub fn set_summarizer(&mut self, tag: TagValue, summarizer: CBORSummarizer) {
        self.summarizers.insert(tag, summarizer);
    }

    fn _insert(
        tag: Tag,
        tags_by_value: &mut HashMap<TagValue, Tag>,
        tags_by_name: &mut HashMap<String, Tag>
    ) {
        let name = tag.name().unwrap();
        assert!(!name.is_empty());
        let result = tags_by_value.insert(tag.value(), tag.clone());
        if let Some(old_value) = result {
            // if the names don't match, we have a problem
            let old_name = old_value.name().unwrap();
            if old_name != name {
                panic!(
                    "Attempt to register tag: {} '{}' with different name: '{}'",
                    tag.value(),
                    old_name,
                    name
                );
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

    fn tag_for_value(&self, value: TagValue) -> Option<Tag> {
        self.tags_by_value.get(&value).cloned()
    }

    fn name_for_value(&self, value: TagValue) -> String {
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
