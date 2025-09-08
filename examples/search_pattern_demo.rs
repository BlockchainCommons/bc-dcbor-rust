use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

fn main() {
    // Create a complex CBOR structure similar to what SearchPattern might
    // encounter
    let mut root_map = Map::new();

    // Add some basic values
    root_map.insert("name", "SearchPattern Test");
    root_map.insert("version", 1);

    // Add nested structures
    let mut user_map = Map::new();
    user_map.insert("id", 12345);
    user_map.insert("email", "test@example.com");
    user_map.insert("roles", vec!["admin", "user"]);
    root_map.insert("user", user_map);

    // Add an array with mixed content
    let mixed_array: Vec<CBOR> = vec![
        CBOR::from("string"),
        CBOR::from(42),
        CBOR::from(vec![1, 2, 3]),
    ];
    root_map.insert("mixed", mixed_array);

    let cbor = CBOR::from(root_map);

    println!("CBOR structure: {}", cbor.diagnostic_flat());
    println!();

    // Example 1: Find all string values
    println!("=== Finding all string values ===");
    let string_values = RefCell::new(Vec::new());

    let string_finder = |element: &WalkElement,
                         _level: usize,
                         _edge: EdgeType,
                         _state: ()|
     -> ((), bool) {
        if let Some(cbor_elem) = element.as_single() {
            if let CBORCase::Text(s) = cbor_elem.as_case() {
                string_values.borrow_mut().push(s.clone());
            }
        }
        ((), false) // Continue traversal
    };

    cbor.walk((), &string_finder);

    for (i, s) in string_values.borrow().iter().enumerate() {
        println!("  String {}: \"{}\"", i + 1, s);
    }
    println!();

    // Example 2: Find all numeric values
    println!("=== Finding all numeric values ===");
    let numeric_values = RefCell::new(Vec::new());

    let number_finder = |element: &WalkElement,
                         _level: usize,
                         edge: EdgeType,
                         _state: ()|
     -> ((), bool) {
        if let Some(cbor_elem) = element.as_single() {
            match cbor_elem.as_case() {
                CBORCase::Unsigned(n) => {
                    let edge_desc =
                        edge.label().unwrap_or_else(|| "root".to_string());
                    numeric_values
                        .borrow_mut()
                        .push(format!("Unsigned {} at {}", n, edge_desc));
                }
                CBORCase::Negative(n) => {
                    let edge_desc =
                        edge.label().unwrap_or_else(|| "root".to_string());
                    numeric_values
                        .borrow_mut()
                        .push(format!("Negative {} at {}", n, edge_desc));
                }
                _ => {}
            }
        }
        ((), false)
    };

    cbor.walk((), &number_finder);

    for (i, desc) in numeric_values.borrow().iter().enumerate() {
        println!("  Number {}: {}", i + 1, desc);
    }
    println!();

    // Example 3: Search for specific patterns (with the new WalkElement API,
    // this is much easier!)
    println!("=== Searching for specific key-value patterns ===");
    let matches = RefCell::new(Vec::new());

    let pattern_searcher = |element: &WalkElement,
                            _level: usize,
                            _edge: EdgeType,
                            _state: ()|
     -> ((), bool) {
        // The new WalkElement API makes key-value pattern matching much more
        // ergonomic!
        if let Some((key, value)) = element.as_key_value() {
            if let CBORCase::Text(key_str) = key.as_case() {
                if key_str == "email"
                    && matches!(value.as_case(), CBORCase::Text(_))
                {
                    matches.borrow_mut().push(format!(
                        "Found email: {}",
                        value.diagnostic_flat()
                    ));
                } else if key_str == "id"
                    && matches!(value.as_case(), CBORCase::Unsigned(_))
                {
                    matches
                        .borrow_mut()
                        .push(format!("Found ID: {}", value.diagnostic_flat()));
                }
            }
        }
        ((), false)
    };

    cbor.walk((), &pattern_searcher);

    for (i, m) in matches.borrow().iter().enumerate() {
        println!("  Match {}: {}", i + 1, m);
    }
    println!();

    // Example 4: Count elements by depth (useful for understanding structure)
    println!("=== Element count by depth ===");
    let depth_counts = RefCell::new(std::collections::HashMap::new());

    let depth_counter = |_element: &WalkElement,
                         level: usize,
                         _edge: EdgeType,
                         _state: ()|
     -> ((), bool) {
        let mut counts = depth_counts.borrow_mut();
        *counts.entry(level).or_insert(0) += 1;
        ((), false)
    };

    cbor.walk((), &depth_counter);

    let mut depths: Vec<_> = depth_counts
        .borrow()
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect();
    depths.sort_by_key(|&(depth, _)| depth);

    for (depth, count) in depths {
        println!("  Depth {}: {} elements", depth, count);
    }
}
