use dcbor::prelude::*;
use dcbor::walk::{EdgeType, WalkElement};
use std::cell::RefCell;

fn main() {
    // Create a complex CBOR structure
    let mut map = Map::new();
    map.insert("name", "Alice");
    map.insert("age", 30);
    map.insert("hobbies", vec!["reading", "coding", "hiking"]);

    let mut nested_map = Map::new();
    nested_map.insert("city", "San Francisco");
    nested_map.insert("zip", 94102);
    map.insert("address", nested_map);

    let cbor = CBOR::from(map);

    println!("CBOR structure (flat diagnostic): {}", cbor.diagnostic_flat());
    println!("\nWalking the CBOR tree:");

    // Walk the structure and print each element
    let visitor = |element: &WalkElement, level: usize, edge: EdgeType, _state: ()| -> ((), bool) {
        let indent = "  ".repeat(level);
        let edge_label = edge.label().unwrap_or_else(|| "root".to_string());

        println!("{}[{}] {}", indent, edge_label, element.diagnostic_flat());
        ((), false) // Continue traversal
    };

    cbor.walk((), &visitor);

    // Example: Count different types of elements
    println!("\nCounting elements by type:");

    #[derive(Clone)]
    struct Counter {
        total: usize,
        maps: usize,
        arrays: usize,
        strings: usize,
        numbers: usize,
        key_value_pairs: usize,
    }

    let final_count = RefCell::new(Counter {
        total: 0,
        maps: 0,
        arrays: 0,
        strings: 0,
        numbers: 0,
        key_value_pairs: 0,
    });

    let counter_visitor = |element: &WalkElement, _level: usize, _edge: EdgeType, mut state: Counter| -> (Counter, bool) {
        state.total += 1;

        match element {
            WalkElement::KeyValue { .. } => state.key_value_pairs += 1,
            WalkElement::Single(cbor_elem) => {
                match cbor_elem.as_case() {
                    CBORCase::Map(_) => state.maps += 1,
                    CBORCase::Array(_) => state.arrays += 1,
                    CBORCase::Text(_) => state.strings += 1,
                    CBORCase::Unsigned(_) | CBORCase::Negative(_) => state.numbers += 1,
                    _ => {}
                }
            }
        }

        // Store the state for final reporting
        *final_count.borrow_mut() = state.clone();

        (state, false)
    };

    cbor.walk(Counter {
        total: 0,
        maps: 0,
        arrays: 0,
        strings: 0,
        numbers: 0,
        key_value_pairs: 0,
    }, &counter_visitor);

    let counts = final_count.borrow();
    println!("Total elements: {}", counts.total);
    println!("Maps: {}", counts.maps);
    println!("Arrays: {}", counts.arrays);
    println!("Strings: {}", counts.strings);
    println!("Numbers: {}", counts.numbers);
    println!("Key-value pairs: {}", counts.key_value_pairs);
}
