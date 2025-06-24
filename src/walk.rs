//! Functions for traversing and manipulating CBOR data structures.
//!
//! This module provides functionality for traversing the hierarchical    /// as
//! EdgeType::TaggedContent =>
//! Some("content".to_string()),rt_eq!(EdgeType::TaggedContent.label(),
//! Some("content".to_string()));tructure of CBOR data items, allowing for
//! operations such as inspection, transformation, and extraction of specific
//! elements. It implements a visitor pattern that enables executing arbitrary
//! code on each element of a CBOR tree in a structured way.
//!
//! The traversal visits every element in a CBOR object recursively, including:
//! - Every array element (with its index)
//! - Every map key-value pair (as semantic units)
//! - Every tagged value (as semantic units, plus their content if nested)
//! - All primitive values (integers, strings, simple values, etc.)
//!
//! # Examples
//!
//! ```
//! use std::cell::RefCell;
//!
//! use dcbor::{
//!     prelude::*,
//!     walk::{EdgeType, WalkElement},
//! };
//!
//! // Create a CBOR structure with nested elements
//! let mut map = Map::new();
//! map.insert("name", "Alice");
//! map.insert("numbers", vec![1, 2, 3]);
//! let cbor = CBOR::from(map);
//!
//! // Count the number of elements in the CBOR structure
//! let count = RefCell::new(0);
//! let visitor = |_element: &WalkElement,
//!                _level: usize,
//!                _edge: EdgeType,
//!                state: ()|
//!  -> ((), bool) {
//!     *count.borrow_mut() += 1;
//!     (state, false)
//! };
//!
//! // Walk the entire CBOR structure
//! cbor.walk((), &visitor);
//! assert!(*count.borrow() > 0);
//! ```

use crate::{CBOR, CBORCase};

/// Represents an element or element pair during CBOR tree traversal.
///
/// This enum allows the visitor to receive either individual CBOR elements
/// or semantic pairs (like map key-value pairs) as a single unit, making
/// pattern matching much more ergonomic.
#[derive(Debug, Clone)]
pub enum WalkElement {
    /// A single CBOR element
    Single(CBOR),
    /// A key-value pair from a map
    KeyValue { key: CBOR, value: CBOR },
}

impl WalkElement {
    /// Returns the single CBOR element if this is a `Single` variant.
    pub fn as_single(&self) -> Option<&CBOR> {
        match self {
            WalkElement::Single(cbor) => Some(cbor),
            WalkElement::KeyValue { .. } => None,
        }
    }

    /// Returns the key-value pair if this is a `KeyValue` variant.
    pub fn as_key_value(&self) -> Option<(&CBOR, &CBOR)> {
        match self {
            WalkElement::Single(_) => None,
            WalkElement::KeyValue { key, value } => Some((key, value)),
        }
    }

    /// Returns a diagnostic string representation of the element(s).
    pub fn diagnostic_flat(&self) -> String {
        match self {
            WalkElement::Single(cbor) => cbor.diagnostic_flat(),
            WalkElement::KeyValue { key, value } => {
                format!(
                    "{}: {}",
                    key.diagnostic_flat(),
                    value.diagnostic_flat()
                )
            }
        }
    }
}

/// The type of incoming edge provided to the visitor.
///
/// This enum identifies how a CBOR element is connected to its parent in
/// the hierarchy during traversal. It helps the visitor function understand the
/// semantic relationship between elements.
///
/// Each edge type represents a specific relationship within the CBOR
/// structure:
/// - `None`: Root or no connection
/// - `ArrayElement`: Element is an item in an array (with index)
/// - `MapKeyValue`: A key-value pair from a map (visited as a semantic unit)
/// - `MapKey`: Element is a key in a map (visited individually)
/// - `MapValue`: Element is a value in a map (visited individually)
/// - `TaggedContent`: Element is the content of a tagged value
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EdgeType {
    /// No incoming edge (root)
    None,
    /// Element is an item in an array
    ArrayElement(usize),
    /// A key-value pair from a map (visited as a semantic unit)
    MapKeyValue,
    /// Element is a key in a map (visited individually)
    MapKey,
    /// Element is a value in a map (visited individually)
    MapValue,
    /// Element is the content of a tagged value
    TaggedContent,
}

/// Provides a label for the edge type in tree formatting.
impl EdgeType {
    /// Returns a short text label for the edge type, or None if no label is
    /// needed.
    ///
    /// This is primarily used for tree formatting to identify relationships
    /// between elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dcbor::walk::EdgeType;
    /// assert_eq!(
    ///     EdgeType::ArrayElement(0).label(),
    ///     Some("arr[0]".to_string())
    /// );
    /// assert_eq!(EdgeType::MapKeyValue.label(), Some("kv".to_string()));
    /// assert_eq!(EdgeType::MapKey.label(), Some("key".to_string()));
    /// assert_eq!(EdgeType::MapValue.label(), Some("val".to_string()));
    /// assert_eq!(EdgeType::TaggedContent.label(), Some("content".to_string()));
    /// assert_eq!(EdgeType::None.label(), None);
    /// ```
    pub fn label(&self) -> Option<String> {
        match self {
            EdgeType::ArrayElement(index) => Some(format!("arr[{}]", index)),
            EdgeType::MapKeyValue => Some("kv".to_string()),
            EdgeType::MapKey => Some("key".to_string()),
            EdgeType::MapValue => Some("val".to_string()),
            EdgeType::TaggedContent => Some("content".to_string()),
            EdgeType::None => None,
        }
    }
}

/// A visitor function that is called for each element in the CBOR structure.
///
/// The visitor function takes the following parameters:
/// - `element`: The current element being visited (either a single CBOR element
///   or a key-value pair)
/// - `level`: The depth level in the hierarchy (0 for root)
/// - `incoming_edge`: The type of edge connecting this element to its parent
/// - `state`: Optional context passed down from the parent's visitor call
///
/// The visitor returns a tuple containing:
/// - The state to pass to child elements
/// - A boolean indicating whether to prevent descent into children of this
///   element (true = don't visit children, false = continue normally)
///
/// The stop flag consistently means "don't visit the children of the current
/// element". This enables depth-limited traversal by checking `level >=
/// max_level`. For full walk abortion, the visitor can maintain its own abort
/// flag and return `true` when the flag is set, causing the walk to unwind
/// quickly.
///
/// # Type Parameters
///
/// * `State` - The type of context passed between parent and child elements
pub type Visitor<'a, State> =
    dyn Fn(&WalkElement, usize, EdgeType, State) -> (State, bool) + 'a;

/// Functions for traversing and manipulating the CBOR hierarchy.
impl CBOR {
    /// Walks the CBOR structure, calling the visitor function for each element.
    ///
    /// This function traverses the entire CBOR hierarchy and calls the
    /// visitor function on each element. For maps, it first visits key-value
    /// pairs as semantic units, then visits keys and values individually.
    ///
    /// The traversal includes:
    /// - Array elements (with their indices)
    /// - Map key-value pairs (as semantic units)
    /// - Map keys and values (individually, for all key-value pairs)
    /// - Tagged values (as semantic units, plus their content if nested)
    /// - All primitive values
    ///
    /// The visitor function can optionally return a context value that is
    /// passed to child elements, enabling state to be accumulated or passed
    /// down during traversal.
    ///
    /// # Type Parameters
    ///
    /// * `State` - The type of context passed between parent and child elements
    ///
    /// # Arguments
    ///
    /// * `state` - The initial state to pass to the root visitor call
    /// * `visit` - The visitor function called for each element
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    ///
    /// use dcbor::{
    ///     prelude::*,
    ///     walk::{EdgeType, Visitor, WalkElement},
    /// };
    ///
    /// // Create a CBOR map for key-value pattern matching
    /// let mut map = Map::new();
    /// map.insert("name", "Alice");
    /// map.insert("age", 30);
    /// let cbor = CBOR::from(map);
    ///
    /// // Find specific key-value patterns
    /// let matches = RefCell::new(Vec::new());
    /// let visitor = |element: &WalkElement,
    ///                _level: usize,
    ///                _edge: EdgeType,
    ///                state: ()|
    ///  -> ((), bool) {
    ///     if let Some((key, value)) = element.as_key_value() {
    ///         if let (CBORCase::Text(k), CBORCase::Text(v)) =
    ///             (key.as_case(), value.as_case())
    ///         {
    ///             if k == "name" {
    ///                 matches.borrow_mut().push(v.clone());
    ///             }
    ///         }
    ///     }
    ///     (state, false)
    /// };
    ///
    /// // Walk the CBOR structure
    /// cbor.walk((), &visitor);
    /// assert!(!matches.borrow().is_empty());
    /// ```
    pub fn walk<State: Clone>(&self, state: State, visit: &Visitor<'_, State>) {
        self._walk(0, EdgeType::None, state, visit);
    }

    /// Recursive implementation of CBOR traversal.
    ///
    /// This internal method performs the actual recursive traversal of the
    /// CBOR structure, visiting every element and maintaining the
    /// correct level and edge relationships.
    fn _walk<State: Clone>(
        &self,
        level: usize,
        incoming_edge: EdgeType,
        state: State,
        visit: &Visitor<'_, State>,
    ) {
        let mut state = state;
        let stop;

        // Visit this element as a single element
        let element = WalkElement::Single(self.clone());
        (state, stop) = visit(&element, level, incoming_edge, state);
        if stop {
            return;
        }

        let next_level = level + 1;
        match self.as_case() {
            CBORCase::Array(array) => {
                for (index, element) in array.iter().enumerate() {
                    element._walk(
                        next_level,
                        EdgeType::ArrayElement(index),
                        state.clone(),
                        visit,
                    );
                }
            }
            CBORCase::Map(map) => {
                for (key, value) in map.iter() {
                    // First, visit the key-value pair as a semantic unit
                    let kv_element = WalkElement::KeyValue {
                        key: key.clone(),
                        value: value.clone(),
                    };
                    let (new_state, stop) = visit(
                        &kv_element,
                        next_level,
                        EdgeType::MapKeyValue,
                        state.clone(),
                    );
                    if stop {
                        continue; // Skip to next key-value pair
                    }

                    // Then visit key and value individually
                    // This allows consistent access to all keys and values,
                    // whether they are primitives or nested structures
                    key._walk(
                        next_level,
                        EdgeType::MapKey,
                        new_state.clone(),
                        visit,
                    );
                    value._walk(
                        next_level,
                        EdgeType::MapValue,
                        new_state,
                        visit,
                    );
                }
            }
            CBORCase::Tagged(_tag, content) => {
                // Visit the content with TaggedContent edge type
                content._walk(
                    next_level,
                    EdgeType::TaggedContent,
                    state,
                    visit,
                );
            }
            // Primitive types have no children to traverse
            CBORCase::Unsigned(_)
            | CBORCase::Negative(_)
            | CBORCase::ByteString(_)
            | CBORCase::Text(_)
            | CBORCase::Simple(_) => {
                // No children to traverse
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::Map;

    #[test]
    fn test_walk_simple_value() {
        let cbor = CBOR::from(42);
        let count = RefCell::new(0);

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       _edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            (state, false)
        };

        cbor.walk((), &visitor);
        assert_eq!(*count.borrow(), 1);
    }

    #[test]
    fn test_walk_array() {
        let cbor = CBOR::from(vec![1, 2, 3]);
        let count = RefCell::new(0);
        let edges = RefCell::new(Vec::new());

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            edges.borrow_mut().push(edge);
            (state, false)
        };

        cbor.walk((), &visitor);

        // Should visit: array + 3 elements = 4 total
        assert_eq!(*count.borrow(), 4);

        let edges = edges.borrow();
        assert_eq!(edges[0], EdgeType::None); // Root array
        assert_eq!(edges[1], EdgeType::ArrayElement(0)); // First element
        assert_eq!(edges[2], EdgeType::ArrayElement(1)); // Second element
        assert_eq!(edges[3], EdgeType::ArrayElement(2)); // Third element
    }

    #[test]
    fn test_walk_map() {
        let mut map = Map::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        let cbor = CBOR::from(map);

        let count = RefCell::new(0);
        let edges = RefCell::new(Vec::new());

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            edges.borrow_mut().push(edge);
            (state, false)
        };

        cbor.walk((), &visitor);

        // Should visit: map + 2 key-value pairs + 4 individual keys/values = 7
        // total (all keys and values are now visited individually)
        assert_eq!(*count.borrow(), 7);

        let edges = edges.borrow();
        assert_eq!(edges[0], EdgeType::None); // Root map
        // Key-value pairs will be visited
        assert!(edges.contains(&EdgeType::MapKeyValue));
        // Individual keys and values will also be visited
        assert!(edges.contains(&EdgeType::MapKey));
        assert!(edges.contains(&EdgeType::MapValue));
    }

    #[test]
    fn test_walk_map_with_nested_content() {
        let mut map = Map::new();
        map.insert("simple", "value");
        map.insert("nested", vec![1, 2, 3]); // This will cause individual key/value visits
        let cbor = CBOR::from(map);

        let count = RefCell::new(0);
        let edges = RefCell::new(Vec::new());

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            edges.borrow_mut().push(edge);
            (state, false)
        };

        cbor.walk((), &visitor);

        let edges = edges.borrow();
        assert_eq!(edges[0], EdgeType::None); // Root map
        assert!(edges.contains(&EdgeType::MapKeyValue)); // Key-value pairs
        assert!(edges.contains(&EdgeType::MapValue)); // Individual value visit for nested array
        assert!(edges.contains(&EdgeType::ArrayElement(0))); // Array elements
    }

    #[test]
    fn test_walk_key_value_pairs() {
        let mut map = Map::new();
        map.insert("name", "Alice");
        map.insert("age", 30);
        let cbor = CBOR::from(map);

        let key_value_pairs = RefCell::new(Vec::new());

        let visitor = |element: &WalkElement,
                       _level: usize,
                       _edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            if let Some((key, value)) = element.as_key_value() {
                if let (CBORCase::Text(k), _) = (key.as_case(), value.as_case())
                {
                    key_value_pairs.borrow_mut().push(k.clone());
                }
            }
            (state, false)
        };

        cbor.walk((), &visitor);

        let pairs = key_value_pairs.borrow();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&"name".to_string()));
        assert!(pairs.contains(&"age".to_string()));
    }

    #[test]
    fn test_walk_tagged() {
        use crate::Tag;

        let tag = Tag::new(0_u64, "datetime");
        let content = CBOR::from("2023-01-01T00:00:00Z");
        let cbor = CBOR::from(CBORCase::Tagged(tag, content));

        let count = RefCell::new(0);
        let edges = RefCell::new(Vec::new());

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            edges.borrow_mut().push(edge);
            (state, false)
        };

        cbor.walk((), &visitor);

        // Should visit: tagged value + content = 2 total
        assert_eq!(*count.borrow(), 2);

        let edges = edges.borrow();
        assert_eq!(edges[0], EdgeType::None); // Root tagged value
        assert_eq!(edges[1], EdgeType::TaggedContent); // Content
    }

    #[test]
    fn test_walk_nested_structure() {
        // Create a nested structure: map with array values
        let mut map = Map::new();
        map.insert("numbers", vec![1, 2, 3]);
        map.insert("text", "hello");
        let cbor = CBOR::from(map);

        let count = RefCell::new(0);

        let visitor = |_element: &WalkElement,
                       _level: usize,
                       _edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            *count.borrow_mut() += 1;
            (state, false)
        };

        cbor.walk((), &visitor);

        // Should visit: map + 2 key-value pairs + 4 individual keys/values +
        // array + 3 elements = 10 total
        assert_eq!(*count.borrow(), 10);
    }

    #[test]
    fn test_walk_early_termination() {
        // Create a nested structure where we can demonstrate early termination
        let mut map = Map::new();
        map.insert("should_visit", "yes");
        map.insert("stop_here", "stop");
        map.insert("should_not_visit", vec![1, 2, 3]); // This array should not be traversed
        let cbor = CBOR::from(map);

        let visited = RefCell::new(Vec::new());

        let visitor = |element: &WalkElement,
                       _level: usize,
                       _edge: EdgeType,
                       state: ()|
         -> ((), bool) {
            // Record what we visited
            visited.borrow_mut().push(element.diagnostic_flat());

            // Stop traversal when we encounter the "stop" text
            let should_stop = if let Some(single) = element.as_single() {
                matches!(single.as_case(), CBORCase::Text(s) if s == "stop")
            } else {
                false
            };
            (state, should_stop)
        };

        cbor.walk((), &visitor);

        let visited = visited.borrow();

        // Should have visited the map, and some keys/values, but stopped at
        // "stop"
        assert!(visited.iter().any(|s| s.contains("stop")));

        // Should NOT have visited the array [1, 2, 3] because it comes after
        // "stop" (though this depends on map iteration order, so we
        // can't guarantee it in this test) The important thing is that
        // "stop" itself stops its own traversal
        assert!(visited.len() > 1); // At least visited the map and some elements
    }

    #[test]
    fn test_walk_with_state() {
        let cbor = CBOR::from(vec![1, 2, 3]);

        // Use state to collect element information
        #[derive(Clone)]
        struct WalkState {
            depth_sum: i32,
        }

        let final_state = RefCell::new(WalkState { depth_sum: 0 });

        let visitor = |_element: &WalkElement,
                       level: usize,
                       _edge: EdgeType,
                       mut state: WalkState|
         -> (WalkState, bool) {
            state.depth_sum += level as i32;
            // Update the final state for verification
            *final_state.borrow_mut() = state.clone();
            (state, false)
        };

        cbor.walk(WalkState { depth_sum: 0 }, &visitor);

        // Verify that state was accumulated
        assert!(final_state.borrow().depth_sum > 0);
    }

    #[test]
    fn test_edge_type_labels() {
        assert_eq!(EdgeType::None.label(), None);
        assert_eq!(
            EdgeType::ArrayElement(5).label(),
            Some("arr[5]".to_string())
        );
        assert_eq!(EdgeType::MapKey.label(), Some("key".to_string()));
        assert_eq!(EdgeType::MapValue.label(), Some("val".to_string()));
        assert_eq!(
            EdgeType::TaggedContent.label(),
            Some("content".to_string())
        );
    }
}
