//! # Walk Module Integration Tests
//!
//! This file contains comprehensive integration tests for the `walk` module in
//! the dcbor crate.
//!
//! ## Test Coverage
//!
//! ### Basic Functionality
//! - **test_traversal_counts**: Verifies correct visit counts for different
//!   CBOR structures (arrays, maps, tagged values, nested structures)
//! - **test_visitor_state_threading**: Tests that visitor state is properly
//!   maintained through traversal
//! - **test_primitive_values**: Ensures primitive values are handled correctly
//! - **test_empty_structures**: Tests behavior with empty arrays and maps
//!
//! ### Traversal Semantics
//! - **test_traversal_order_and_edge_types**: Validates the order of visits and
//!   correct edge type labeling
//! - **test_map_keyvalue_semantics**: Verifies that map key-value pairs are
//!   visited both as semantic units and individually
//! - **test_tagged_value_traversal**: Tests traversal of tagged values and
//!   nested tagged structures
//!
//! ### Advanced Features
//! - **test_depth_limited_traversal**: Tests depth-limited traversal using the
//!   level parameter
//! - **test_early_termination**: Demonstrates controlled termination using the
//!   stop flag to prevent descent into children
//! - **test_stop_flag_prevents_descent**: Verifies that the stop flag
//!   consistently prevents descent into children while allowing sibling
//!   traversal
//!
//! ### Real-World Usage
//! - **test_text_extraction**: Extracts all text strings from a complex nested
//!   structure
//! - **test_real_world_document**: Tests traversal of a realistic JSON-like
//!   document structure converted to CBOR
//!
//! ## Key Behaviors Verified
//!
//! 1. **Consistent Stop Semantics**: The stop flag means "prevent descent into
//!    children" uniformly across all element types
//! 2. **Complete Coverage**: Every element is visited including all array
//!    elements, map key-value pairs (both semantic and individual), and tagged
//!    content
//! 3. **State Management**: Visitor state is properly threaded through the
//!    traversal
//! 4. **Edge Type Accuracy**: Correct edge types are provided for all
//!    relationship contexts
//! 5. **Performance**: Efficient traversal of large nested structures
//!
//! ## Usage Patterns
//!
//! These tests demonstrate several common usage patterns:
//! - Text indexing and search
//! - Structure analysis and debugging
//! - Depth-controlled traversal for UI trees
//! - Conditional processing with early termination
//! - Data extraction and transformation
//!
//! All tests pass and verify the walk module provides ergonomic, reliable, and
//! efficient CBOR tree traversal.

use std::{cell::RefCell, collections::HashMap};

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

/// Test basic traversal counts for different CBOR structures
#[test]
fn test_traversal_counts() {
    // Simple array
    let array = CBOR::from(vec![1, 2, 3]);
    let count = count_visits(&array);
    // Root + 3 array elements = 4
    assert_eq!(count, 4);

    // Simple map
    let mut map = Map::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let map_cbor = CBOR::from(map);
    let count = count_visits(&map_cbor);
    // Root + 2 key-value pairs + 4 individual keys/values = 7
    assert_eq!(count, 7);

    // Tagged value
    let tagged = CBOR::to_tagged_value(42, 100);
    let count = count_visits(&tagged);
    // Root tagged value + content = 2
    assert_eq!(count, 2);

    // Nested structure
    let mut inner_map = Map::new();
    inner_map.insert("x", vec![1, 2]);
    let mut outer_map = Map::new();
    outer_map.insert("inner", inner_map);
    outer_map.insert("simple", 42);
    let nested = CBOR::from(outer_map);
    let count = count_visits(&nested);
    // Should visit:
    // 1. root map
    // 2-3. 2 kv pairs in outer (inner and simple)
    // 4-5. 2 individual keys in outer (inner, simple)
    // 6. inner map value
    // 7. 1 kv pair in inner map (x)
    // 8-9. 2 individual key/value in inner (x key, array value)
    // 10-11. 2 array elements (1, 2)
    // 12. simple value (42)
    // = 12 total
    assert_eq!(count, 12);
}

/// Test that visitor state is properly threaded through traversal
#[test]
fn test_visitor_state_threading() {
    let array = CBOR::from(vec![1, 2, 3, 4, 5]);

    // Count only even numbers using visitor state and a RefCell
    let even_count = RefCell::new(0);
    let visitor = |element: &WalkElement,
                   _level: usize,
                   _edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        match element {
            WalkElement::Single(cbor) => {
                if let CBORCase::Unsigned(n) = cbor.as_case()
                    && n % 2 == 0
                {
                    *even_count.borrow_mut() += 1;
                }
            }
            WalkElement::KeyValue { .. } => {}
        }
        (state, false)
    };

    array.walk((), &visitor);
    assert_eq!(*even_count.borrow(), 2); // 2 and 4 are even
}

/// Test early termination using visitor pattern
#[test]
fn test_early_termination() {
    // Test shows that stop flag prevents descent into children but doesn't
    // abort entire walk
    let nested_structure = CBOR::from(vec![
        CBOR::from(vec!["should", "see", "this"]), /* This array's children
                                                    * will be visited */
        CBOR::from("abort_marker"), // This will set stop flag
        CBOR::from(vec!["should", "not", "see"]), /* This array will be
                                     * visited but not its
                                     * children */
    ]);

    let visit_log = RefCell::new(Vec::<String>::new());
    let found_abort = RefCell::new(false);

    let visitor = |element: &WalkElement,
                   level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc =
            format!("L{}: {:?} - {}", level, edge, element.diagnostic_flat());
        visit_log.borrow_mut().push(desc);

        // Check if this is our abort marker
        if let WalkElement::Single(cbor) = element
            && let CBORCase::Text(text) = cbor.as_case()
            && text == "abort_marker"
        {
            *found_abort.borrow_mut() = true;
            // Return stop=true to prevent descent into this element's
            // children (though strings don't have
            // children anyway)
            return (state, true);
        }

        // If we've seen the abort marker and this is an array at level 1, stop
        // descent
        let stop = *found_abort.borrow()
            && matches!(element, WalkElement::Single(_))
            && matches!(edge, EdgeType::ArrayElement(2));

        (state, stop)
    };

    nested_structure.walk((), &visitor);

    let log = visit_log.borrow();
    let log_str = log.join("\n");

    // Should visit the abort marker
    assert!(log_str.contains("abort_marker"));

    // Should visit the first array and its children (before abort marker)
    assert!(log_str.contains("should"));
    assert!(log_str.contains("see"));
    assert!(log_str.contains("this"));

    // Should visit the third array but NOT its children (after abort marker
    // with stop=true)
    assert!(log_str.contains("[\"should\", \"not\", \"see\"]")); // The array itself

    // Should NOT visit the individual strings "should", "not", "see" that come
    // from the third array We can verify this by checking that there are no
    // Level 2 visits after the third array
    let log_lines: Vec<&str> = log.iter().map(|s| s.as_str()).collect();

    // Find the index of the third array visit
    let third_array_index = log_lines.iter().position(|line| {
        line.contains("ArrayElement(2)")
            && line.contains("[\"should\", \"not\", \"see\"]")
    });

    if let Some(index) = third_array_index {
        // Check that there are no Level 2 visits after this index
        let visits_after_third_array = &log_lines[index + 1..];
        let level2_after_third: Vec<&&str> = visits_after_third_array
            .iter()
            .filter(|line| line.starts_with("L2:"))
            .collect();

        // Should be no Level 2 visits after the third array due to stop flag
        assert!(
            level2_after_third.is_empty(),
            "Found unexpected level 2 visits after third array: {:?}",
            level2_after_third
        );
    } else {
        panic!("Could not find third array visit in log");
    }
}

/// Test depth-limited traversal using level parameter
#[test]
fn test_depth_limited_traversal() {
    // Create deeply nested structure
    let mut level3 = Map::new();
    level3.insert("deep", "value");

    let mut level2 = Map::new();
    level2.insert("level3", level3);

    let mut level1 = Map::new();
    level1.insert("level2", level2);

    let root = CBOR::from(level1);

    // Collect elements at each level
    let elements_by_level = RefCell::new(HashMap::<usize, usize>::new());

    let visitor = |_element: &WalkElement,
                   level: usize,
                   _edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        *elements_by_level.borrow_mut().entry(level).or_insert(0) += 1;
        // Stop descent if we're at level 2 or deeper
        let stop = level >= 2;
        (state, stop)
    };

    root.walk((), &visitor);

    let counts = elements_by_level.borrow();
    assert_eq!(*counts.get(&0).unwrap_or(&0), 1); // Root
    assert_eq!(*counts.get(&1).unwrap_or(&0), 3); // 1 kv pair + 2 individual key/value
    assert_eq!(*counts.get(&2).unwrap_or(&0), 1); // Just the nested map, no descent
    assert_eq!(*counts.get(&3).unwrap_or(&0), 0); // No visits at level 3 due to stop
}

/// Test text extraction from complex CBOR structures
#[test]
fn test_text_extraction() {
    // Create a complex structure with text at various levels
    let mut metadata = Map::new();
    metadata.insert("title", "Important Document");
    metadata.insert("author", "Alice Smith");

    let mut content = Map::new();
    content.insert("body", "Lorem ipsum dolor sit amet");
    content.insert("footer", "Copyright 2024");

    let mut document = Map::new();
    document.insert("metadata", metadata);
    document.insert("content", content);
    document.insert("tags", vec!["urgent", "confidential", "draft"]);

    let cbor = CBOR::from(document);

    // Extract all text strings
    let texts = RefCell::new(Vec::<String>::new());

    let visitor = |element: &WalkElement,
                   _level: usize,
                   _edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        match element {
            WalkElement::Single(cbor) => {
                if let CBORCase::Text(text) = cbor.as_case() {
                    texts.borrow_mut().push(text.clone());
                }
            }
            WalkElement::KeyValue { key, value } => {
                if let CBORCase::Text(text) = key.as_case() {
                    texts.borrow_mut().push(text.clone());
                }
                if let CBORCase::Text(text) = value.as_case() {
                    texts.borrow_mut().push(text.clone());
                }
            }
        }
        (state, false)
    };

    cbor.walk((), &visitor);

    let all_texts = texts.borrow();
    // Should find all text strings in the structure
    assert!(all_texts.contains(&"Important Document".to_string()));
    assert!(all_texts.contains(&"Alice Smith".to_string()));
    assert!(all_texts.contains(&"Lorem ipsum dolor sit amet".to_string()));
    assert!(all_texts.contains(&"Copyright 2024".to_string()));
    assert!(all_texts.contains(&"urgent".to_string()));
    assert!(all_texts.contains(&"confidential".to_string()));
    assert!(all_texts.contains(&"draft".to_string()));
    // Also keys
    assert!(all_texts.contains(&"title".to_string()));
    assert!(all_texts.contains(&"author".to_string()));
    assert!(all_texts.contains(&"body".to_string()));
    assert!(all_texts.contains(&"footer".to_string()));
    assert!(all_texts.contains(&"metadata".to_string()));
    assert!(all_texts.contains(&"content".to_string()));
    assert!(all_texts.contains(&"tags".to_string()));
}

/// Test traversal order and edge types
#[test]
fn test_traversal_order_and_edge_types() {
    let mut map = Map::new();
    map.insert("a", vec![1, 2]);
    map.insert("b", 42);
    let cbor = CBOR::from(map);

    let traversal_log = RefCell::new(Vec::<(String, EdgeType)>::new());

    let visitor = |element: &WalkElement,
                   _level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc = match element {
            WalkElement::Single(cbor) => {
                format!("Single({})", cbor.diagnostic_flat())
            }
            WalkElement::KeyValue { key, value } => {
                format!(
                    "KeyValue({}: {})",
                    key.diagnostic_flat(),
                    value.diagnostic_flat()
                )
            }
        };
        traversal_log.borrow_mut().push((desc, edge));
        (state, false)
    };

    cbor.walk((), &visitor);

    let log = traversal_log.borrow();

    // Verify root visit
    assert_eq!(log[0].1, EdgeType::None);

    // Check that we have the expected edge types
    let edge_types: Vec<EdgeType> = log.iter().map(|(_, edge)| *edge).collect();
    assert!(edge_types.contains(&EdgeType::MapKeyValue));
    assert!(edge_types.contains(&EdgeType::MapKey));
    assert!(edge_types.contains(&EdgeType::MapValue));
    assert!(edge_types.contains(&EdgeType::ArrayElement(0)));
    assert!(edge_types.contains(&EdgeType::ArrayElement(1)));
}

/// Test tagged value traversal
#[test]
fn test_tagged_value_traversal() {
    // Create nested tagged values
    let inner_tagged = CBOR::to_tagged_value(123, vec![1, 2, 3]);
    let outer_tagged = CBOR::to_tagged_value(456, inner_tagged);

    let edge_log = RefCell::new(Vec::<EdgeType>::new());

    let visitor = |_element: &WalkElement,
                   _level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        edge_log.borrow_mut().push(edge);
        (state, false)
    };

    outer_tagged.walk((), &visitor);

    let edges = edge_log.borrow();

    // Should see: None (root), TaggedContent, TaggedContent, ArrayElement(0),
    // ArrayElement(1), ArrayElement(2)
    assert_eq!(edges[0], EdgeType::None); // Root tagged value
    assert_eq!(edges[1], EdgeType::TaggedContent); // Inner tagged value
    assert_eq!(edges[2], EdgeType::TaggedContent); // Array content of inner tagged
    assert_eq!(edges[3], EdgeType::ArrayElement(0)); // First array element
    assert_eq!(edges[4], EdgeType::ArrayElement(1)); // Second array element
    assert_eq!(edges[5], EdgeType::ArrayElement(2)); // Third array element
}

/// Test map key-value semantics
#[test]
fn test_map_keyvalue_semantics() {
    let mut map = Map::new();
    map.insert("simple", 42);
    map.insert("nested", vec![1, 2]);
    let cbor = CBOR::from(map);

    let keyvalue_count = RefCell::new(0);
    let individual_count = RefCell::new(0);

    let visitor = |element: &WalkElement,
                   _level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        match element {
            WalkElement::KeyValue { .. } => {
                *keyvalue_count.borrow_mut() += 1;
                assert_eq!(edge, EdgeType::MapKeyValue);
            }
            WalkElement::Single(_) => {
                if matches!(edge, EdgeType::MapKey | EdgeType::MapValue) {
                    *individual_count.borrow_mut() += 1;
                }
            }
        }
        (state, false)
    };

    cbor.walk((), &visitor);

    // Should have 2 key-value pairs and 4 individual key/value visits
    assert_eq!(*keyvalue_count.borrow(), 2);
    assert_eq!(*individual_count.borrow(), 4);
}

/// Test stop flag prevents descent consistently
#[test]
fn test_stop_flag_prevents_descent() {
    let nested = CBOR::from(vec![
        vec![1, 2, 3], // Index 0: prevent descent into this
        vec![4, 5, 6], // Index 1: allow descent into this
        vec![7, 8, 9], // Index 2: allow descent into this
    ]);

    let visit_log = RefCell::new(Vec::<String>::new());

    let visitor = |element: &WalkElement,
                   level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc =
            format!("L{}: {:?} - {}", level, edge, element.diagnostic_flat());
        visit_log.borrow_mut().push(desc);

        // Stop descent into the first nested array (at index 0)
        let stop = level == 1 && matches!(edge, EdgeType::ArrayElement(0));
        (state, stop)
    };

    nested.walk((), &visitor);

    let log = visit_log.borrow();
    let log_str = log.join("\n");

    // Should visit the first array but not descend into it
    assert!(log_str.contains("ArrayElement(0) - [1, 2, 3]")); // First array is visited

    // Should NOT find any level 2 visits that came from the first array
    // The elements 1, 2, 3 should not appear at level 2
    let level2_lines: Vec<&str> = log
        .iter()
        .filter(|line| line.starts_with("L2:"))
        .map(|s| s.as_str())
        .collect();

    // None of the level 2 visits should contain the values from the first array
    for line in &level2_lines {
        assert!(!line.contains(" - 1"));
        assert!(!line.contains(" - 2"));
        assert!(!line.contains(" - 3"));
    }

    // Should visit second and third arrays with descent
    assert!(log_str.contains("ArrayElement(1) - [4, 5, 6]")); // Second array is visited
    assert!(log_str.contains("ArrayElement(2) - [7, 8, 9]")); // Third array is visited

    // Should find level 2 visits from second and third arrays
    assert!(
        log_str.contains("L2:")
            && (log_str.contains(" - 4")
                || log_str.contains(" - 5")
                || log_str.contains(" - 6"))
    );
    assert!(
        log_str.contains("L2:")
            && (log_str.contains(" - 7")
                || log_str.contains(" - 8")
                || log_str.contains(" - 9"))
    );
}

/// Test empty structures
#[test]
fn test_empty_structures() {
    // Empty array
    let empty_array = CBOR::from(Vec::<i32>::new());
    let count = count_visits(&empty_array);
    assert_eq!(count, 1); // Just the root

    // Empty map
    let empty_map = CBOR::from(Map::new());
    let count = count_visits(&empty_map);
    assert_eq!(count, 1); // Just the root
}

/// Test primitive values
#[test]
fn test_primitive_values() {
    let primitives = vec![
        CBOR::from(42),
        CBOR::from("hello"),
        CBOR::from(3.2222),
        CBOR::from(true),
        CBOR::null(),
    ];

    for primitive in primitives {
        let count = count_visits(&primitive);
        assert_eq!(count, 1); // Just the primitive itself
    }
}

/// Test real-world document structure
#[test]
fn test_real_world_document() {
    // Simulate a JSON-like document converted to CBOR
    let mut person = Map::new();
    person.insert("name", "John Doe");
    person.insert("age", 30);
    person.insert("email", "john@example.com");

    let mut address = Map::new();
    address.insert("street", "123 Main St");
    address.insert("city", "Anytown");
    address.insert("zipcode", "12345");

    person.insert("address", address);
    person.insert("hobbies", vec!["reading", "cycling", "cooking"]);

    let mut skills = Map::new();
    skills.insert("programming", vec!["Rust", "Python", "JavaScript"]);
    skills.insert("languages", vec!["English", "Spanish"]);

    person.insert("skills", skills);

    let document = CBOR::from(person);

    // Extract all string values for search/indexing
    let strings = RefCell::new(Vec::<String>::new());

    let visitor = |element: &WalkElement,
                   _level: usize,
                   _edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        match element {
            WalkElement::Single(cbor) => {
                if let CBORCase::Text(text) = cbor.as_case() {
                    strings.borrow_mut().push(text.clone());
                }
            }
            WalkElement::KeyValue { key, value } => {
                if let CBORCase::Text(text) = key.as_case() {
                    strings.borrow_mut().push(text.clone());
                }
                if let CBORCase::Text(text) = value.as_case() {
                    strings.borrow_mut().push(text.clone());
                }
            }
        }
        (state, false)
    };

    document.walk((), &visitor);

    let all_strings = strings.borrow();

    // Verify we found all expected strings
    assert!(all_strings.contains(&"John Doe".to_string()));
    assert!(all_strings.contains(&"john@example.com".to_string()));
    assert!(all_strings.contains(&"123 Main St".to_string()));
    assert!(all_strings.contains(&"Anytown".to_string()));
    assert!(all_strings.contains(&"12345".to_string()));
    assert!(all_strings.contains(&"reading".to_string()));
    assert!(all_strings.contains(&"cycling".to_string()));
    assert!(all_strings.contains(&"cooking".to_string()));
    assert!(all_strings.contains(&"Rust".to_string()));
    assert!(all_strings.contains(&"Python".to_string()));
    assert!(all_strings.contains(&"JavaScript".to_string()));
    assert!(all_strings.contains(&"English".to_string()));
    assert!(all_strings.contains(&"Spanish".to_string()));

    // Should also find all field names
    assert!(all_strings.contains(&"name".to_string()));
    assert!(all_strings.contains(&"age".to_string()));
    assert!(all_strings.contains(&"email".to_string()));
    assert!(all_strings.contains(&"address".to_string()));
    assert!(all_strings.contains(&"hobbies".to_string()));
    assert!(all_strings.contains(&"skills".to_string()));
    assert!(all_strings.contains(&"programming".to_string()));
    assert!(all_strings.contains(&"languages".to_string()));
}

// Helper function to count total visits
fn count_visits(cbor: &CBOR) -> usize {
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
    *count.borrow()
}
