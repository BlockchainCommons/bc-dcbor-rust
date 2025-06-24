use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::WalkElement,
};

fn main() {
    println!("=== Testing stop flag behavior ===\n");

    // Create a nested structure to test stop behavior
    let mut inner_map = Map::new();
    inner_map.insert("inner_key", "inner_value");

    let mut map = Map::new();
    map.insert("first", "stop_here"); // Will trigger stop
    map.insert("second", inner_map); // Should not be visited if stop works
    map.insert("third", vec![1, 2, 3]); // Should not be visited if stop works
    let cbor = CBOR::from(map);

    println!("CBOR structure: {}", cbor.diagnostic_flat());

    // Test 1: Stop on visiting a single element
    println!("\n=== Test 1: Stop on single element ===");
    let visited = RefCell::new(Vec::new());
    cbor.walk((), &|element, depth, edge, state| {
        let indent = "  ".repeat(depth);
        let description = match element {
            WalkElement::Single(cbor) => {
                format!("Single: {}", cbor.diagnostic_flat())
            }
            WalkElement::KeyValue { key, value } => format!(
                "KV: {} => {}",
                key.diagnostic_flat(),
                value.diagnostic_flat()
            ),
        };

        visited
            .borrow_mut()
            .push(format!("{}[{:?}] {}", indent, edge, description));

        // Stop when we see "stop_here" as a single element
        let should_stop = if let Some(single) = element.as_single() {
            matches!(single.as_case(), CBORCase::Text(s) if s == "stop_here")
        } else {
            false
        };

        if should_stop {
            println!("{}🛑 STOPPING at: {}", indent, description);
        } else {
            println!("{}✅ Visited: {}", indent, description);
        }

        (state, should_stop)
    });

    println!("\nAll visits in order:");
    for visit in visited.borrow().iter() {
        println!("  {}", visit);
    }

    // Test 2: Stop on visiting a key-value pair
    println!("\n=== Test 2: Stop on key-value pair ===");
    let visited2 = RefCell::new(Vec::new());
    cbor.walk((), &|element, depth, edge, state| {
        let indent = "  ".repeat(depth);
        let description = match element {
            WalkElement::Single(cbor) => {
                format!("Single: {}", cbor.diagnostic_flat())
            }
            WalkElement::KeyValue { key, value } => format!(
                "KV: {} => {}",
                key.diagnostic_flat(),
                value.diagnostic_flat()
            ),
        };

        visited2
            .borrow_mut()
            .push(format!("{}[{:?}] {}", indent, edge, description));

        // Stop when we see a key-value pair with "stop_here"
        let should_stop = if let Some((_key, value)) = element.as_key_value() {
            matches!(value.as_case(), CBORCase::Text(s) if s == "stop_here")
        } else {
            false
        };

        if should_stop {
            println!("{}🛑 STOPPING at: {}", indent, description);
        } else {
            println!("{}✅ Visited: {}", indent, description);
        }

        (state, should_stop)
    });

    println!("\nAll visits in order:");
    for visit in visited2.borrow().iter() {
        println!("  {}", visit);
    }
}
