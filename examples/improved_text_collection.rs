use std::cell::RefCell;

use dcbor::prelude::*;

fn main() {
    // Create a map with various types
    let mut map = Map::new();
    map.insert("name", "Alice");
    map.insert("age", 30);
    map.insert("nested", vec!["skill1", "skill2"]);
    let cbor = CBOR::from(map);

    println!("CBOR: {}", cbor.diagnostic_flat());
    println!("\n=== NEW: Simple text collection ===");

    let texts = RefCell::new(Vec::new());
    cbor.walk((), &|element, _depth, _edge, state| {
        // Now we can collect ALL text nodes with a simple pattern match!
        if let Some(cbor) = element.as_single() {
            if let CBORCase::Text(s) = cbor.as_case() {
                texts.borrow_mut().push(s.clone());
            }
        }
        (state, false)
    });

    println!("All text values found: {:?}", *texts.borrow());
    println!("Total: {}", texts.borrow().len());

    println!("\n=== Comparison: Key-value pairs are ALSO available ===");
    let kv_pairs = RefCell::new(Vec::new());
    cbor.walk((), &|element, _depth, _edge, state| {
        if let Some((key, value)) = element.as_key_value() {
            if let (CBORCase::Text(k), CBORCase::Text(v)) =
                (key.as_case(), value.as_case())
            {
                kv_pairs.borrow_mut().push(format!("{} -> {}", k, v));
            }
        }
        (state, false)
    });

    println!("Key-value text pairs: {:?}", *kv_pairs.borrow());

    println!("\n=== Benefits ===");
    println!("✅ Consistent: All elements visited individually");
    println!("✅ Ergonomic: Simple pattern matching on WalkElement::Single");
    println!("✅ Complete: No manual checking of key-value pairs required");
    println!(
        "✅ Flexible: Both individual elements AND semantic pairs available"
    );
}
