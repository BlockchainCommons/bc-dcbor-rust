use std::cell::RefCell;

use dcbor::{prelude::*, walk::WalkElement};

fn main() {
    // Create a simple map with text keys and values
    let mut map = Map::new();
    map.insert("name", "Alice"); // Both text - should be easy to collect
    map.insert("age", 30); // Text key, number value
    map.insert("nested", vec![1, 2]); // Text key, array value
    let cbor = CBOR::from(map);

    println!("CBOR: {}", cbor.diagnostic_flat());
    println!("\n=== Current behavior (with has_nested_content check) ===");

    let texts = RefCell::new(Vec::new());
    cbor.walk((), &|element, depth, edge, state| {
        let indent = "  ".repeat(depth);
        match element {
            WalkElement::Single(cbor) => {
                println!(
                    "{}[{:?}] Single: {}",
                    indent,
                    edge,
                    cbor.diagnostic_flat()
                );
                if let CBORCase::Text(s) = cbor.as_case() {
                    texts.borrow_mut().push(format!("Single: {}", s));
                }
            }
            WalkElement::KeyValue { key, value } => {
                println!(
                    "{}[{:?}] KeyValue: {} => {}",
                    indent,
                    edge,
                    key.diagnostic_flat(),
                    value.diagnostic_flat()
                );

                // User has to manually check both key and value
                if let CBORCase::Text(s) = key.as_case() {
                    texts.borrow_mut().push(format!("KV-Key: {}", s));
                }
                if let CBORCase::Text(s) = value.as_case() {
                    texts.borrow_mut().push(format!("KV-Value: {}", s));
                }
            }
        }
        (state, false)
    });

    println!("\nCollected texts: {:?}", *texts.borrow());
    println!("Total texts found: {}", texts.borrow().len());
}
