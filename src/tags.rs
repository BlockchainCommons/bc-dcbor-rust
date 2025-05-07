import_stdlib!();

use crate::{ CBORTaggedDecodable, Date, Tag, TagValue, TagsStore, TagsStoreTrait };

#[doc(hidden)]
pub use paste::paste;

pub struct LazyTagsStore {
    init: Once,
    data: Mutex<Option<TagsStore>>,
}

#[cfg(feature = "std")]
impl LazyTagsStore {
    pub fn get(&self) -> MutexGuard<'_, Option<TagsStore>> {
        self.init.call_once(|| {
            let m = TagsStore::new([]);
            *self.data.lock().unwrap() = Some(m);
        });
        self.data.lock().unwrap()
    }
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
impl LazyTagsStore {
    pub fn get(&self) -> MutexGuard<'_, Option<TagsStore>> {
        self.init.call_once(|| {
            let m = TagsStore::new([]);
            *self.data.lock() = Some(m);
        });
        self.data.lock()
    }
}

pub static GLOBAL_TAGS: LazyTagsStore = LazyTagsStore {
    init: Once::new(),
    data: Mutex::new(None),
};

/// A macro for accessing the global tags store in a read-only manner.
///
/// This macro provides safe access to a global [`TagsStore`] instance that persists
/// for the entire application lifetime. It executes the provided closure with a
/// reference to the tags store.
///
/// ## Use Cases
///
/// Use this macro when you need to:
/// - Look up tag names or values
/// - Access tag summarizers
/// - Format diagnostic output with tag information
/// - Resolve tags in any read-only operation
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
/// use std::sync::Arc;
///
/// // Use with_tags to access the global tags store in a read-only manner
/// let tag_name = with_tags!(|tags: &TagsStore| {
///     // First, let's register a tag in the global store for demonstration
///     if tags.tag_for_value(42).is_none() {
///         // We don't have direct mutable access here, but in a real app
///         // you would use with_tags_mut! to register tags first
///         // This is just for demonstration purposes
///     }
///
///     // Look up a tag name by its value (could be "date" if standard tags are registered)
///     // or the numeric value as a string if not registered
///     tags.name_for_value(42)
/// });
///
/// // Verify we got some kind of tag name or value
/// assert!(!tag_name.is_empty());
///
/// // Use with diagnostic output
/// let cbor = CBOR::from(42);
/// // We can format the CBOR value using tag information from the store
/// with_tags!(|tags| {
///     let _formatted = cbor.diagnostic_opt(true, false, false, Some(tags));
///     // In a real app, we would use the formatted output
/// });
/// ```
///
/// ## Thread Safety
///
/// This macro is thread-safe. The global tags store is protected by a mutex,
/// and this macro acquires a read lock on that mutex.
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

/// A macro for accessing the global tags store in a mutable manner.
///
/// This macro provides safe mutable access to a global [`TagsStore`] instance
/// that persists for the entire application lifetime. It executes the provided
/// closure with a mutable reference to the tags store.
///
/// ## Use Cases
///
/// Use this macro when you need to:
/// - Register new tags
/// - Add or modify tag summarizers
/// - Update the global tags store in any way
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
/// use std::sync::Arc;
///
/// // Register a custom tag in the global tags store
/// with_tags_mut!(|tags: &mut TagsStore| {
///     // Register a tag with value 100 and name "custom-tag"
///     tags.insert(Tag::new(100, "custom-tag".to_string()));
///
///     // Add a summarizer for the tag
///     tags.set_summarizer(100, Arc::new(|cbor| -> dcbor::Result<String> {
///         Ok(format!("Custom tag content: {:?}", cbor))
///     }));
/// });
///
/// // Later, you can read the registered tag
/// let tag_name = with_tags!(|tags: &TagsStore| {
///     tags.name_for_value(100)
/// });
///
/// assert_eq!(tag_name, "custom-tag");
/// ```
///
/// ## Thread Safety
///
/// This macro is thread-safe. The global tags store is protected by a mutex,
/// and this macro acquires a write lock on that mutex. If multiple threads
/// attempt to modify the tags store simultaneously, they will be serialized.
///
/// ## Caution
///
/// Be careful not to create deadlocks by nesting calls to `with_tags_mut!` or
/// holding the mutex lock for extended periods of time.
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

#[macro_export]
macro_rules! const_cbor_tag {
    ($value:expr, $const_name:ident, $name:expr) => {
        $crate::paste! {
            pub const [<TAG_ $const_name>]: u64 = $value;
            pub const [<TAG_NAME_ $const_name>]: &str = $name;
        }
    };
}

#[macro_export]
macro_rules! cbor_tag {
    ($const_name:ident) => {
        $crate::paste! {
            $crate::Tag::new([<TAG_ $const_name>], [<TAG_NAME_ $const_name>])
        }
    };
}

const_cbor_tag!(1, DATE, "date");

pub fn register_tags_in(tags_store: &mut TagsStore) {
    let tags = vec![cbor_tag!(DATE)];
    tags_store.insert_all(tags);
    tags_store.set_summarizer(
        TAG_DATE,
        Arc::new(|untagged_cbor| { Ok(format!("{}", Date::from_untagged_cbor(untagged_cbor)?)) })
    );
}

pub fn register_tags() {
    with_tags_mut!(|tags_store: &mut TagsStore| {
        register_tags_in(tags_store);
    });
}

/// Converts a slice of tag values to their corresponding [`Tag`] objects.
///
/// This function looks up each tag value in the global tag registry and returns
/// a vector of complete `Tag` objects. For any tag values that aren't registered
/// in the global registry, it creates a basic `Tag` with just the value (no name).
///
/// ## Purpose
///
/// This utility function is particularly useful when:
/// - You have a list of numeric tag values (e.g., from raw CBOR decoding)
/// - You need to convert those values to complete `Tag` objects with names
/// - You want to handle both registered and unregistered tags gracefully
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // First, let's make sure we have a tag in the global registry for testing
/// with_tags_mut!(|tags: &mut TagsStore| {
///     // Register a tag for demonstration purposes
///     tags.insert(Tag::new(1, "test-tag".to_string()));
/// });
///
/// // A list of tag values - some may be registered, others may not
/// let tag_values = &[1, 42, 999];
///
/// // Convert to Tag objects
/// let tags = tags_for_values(tag_values);
///
/// // The first tag (value 1) should be registered with our name
/// assert_eq!(tags[0].value(), 1);
/// assert!(tags[0].name().is_some());
///
/// // Unregistered tags will have a value but no name
/// assert_eq!(tags[1].value(), 42);
/// assert_eq!(tags[2].value(), 999);
/// ```
///
/// ## Implementation Details
///
/// This function uses the [`with_tags!`] macro internally to access the global
/// tags registry. For each value:
/// 1. It looks up the tag using `tag_for_value`
/// 2. If found, it uses the registered tag with its name
/// 3. If not found, it falls back to creating a basic tag with just the value
pub fn tags_for_values(values: &[TagValue]) -> Vec<Tag> {
    with_tags!(|tags: &TagsStore| {
        values
            .iter()
            .map(|value| tags.tag_for_value(*value).unwrap_or_else(|| Tag::with_value(*value)))
            .collect()
    })
}
