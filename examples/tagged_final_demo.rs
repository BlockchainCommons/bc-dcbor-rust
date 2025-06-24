use dcbor::{prelude::*, walk::WalkElement};

fn main() {
    // Create a more complex structure with tagged values
    let mut person_map = Map::new();
    person_map.insert("name", "Alice");
    person_map.insert("age", 30);

    // Create a tagged UUID (tag 37 for UUID)
    let uuid_tagged = CBOR::from(CBORCase::Tagged(
        Tag::from(37u64),
        CBOR::from("f47ac10b-58cc-4372-a567-0e02b2c3d479"),
    ));
    person_map.insert("id", uuid_tagged);

    // Create a tagged timestamp (tag 0 for date/time string)
    let timestamp_tagged = CBOR::from(CBORCase::Tagged(
        Tag::from(0u64),
        CBOR::from("2023-10-27T14:30:15.123Z"),
    ));
    person_map.insert("created", timestamp_tagged);

    let cbor = CBOR::from(person_map);

    println!("CBOR structure: {}", cbor.diagnostic());
    println!("\nWalk output:");

    cbor.walk(0, &|element, depth, edge, count| {
        let indent = "  ".repeat(depth);
        match element {
            WalkElement::Single(cbor) => {
                let type_info = match cbor.as_case() {
                    CBORCase::Tagged(tag, _) => {
                        format!(" (tagged with {})", tag.value())
                    }
                    CBORCase::Text(_) => " (text)".to_string(),
                    CBORCase::Unsigned(_) => " (number)".to_string(),
                    CBORCase::Map(_) => " (map)".to_string(),
                    _ => "".to_string(),
                };
                println!(
                    "{}{}[{:?}] {}{}",
                    indent,
                    count,
                    edge,
                    cbor.diagnostic_flat(),
                    type_info
                );
            }
            WalkElement::KeyValue { key, value } => {
                println!(
                    "{}{}[{:?}] {}: {}",
                    indent,
                    count,
                    edge,
                    key.diagnostic_flat(),
                    value.diagnostic_flat()
                );
            }
        }
        (count + 1, false)
    });

    println!("\n=== Benefits of the new design ===");
    println!("✅ Tagged values are treated as semantic units");
    println!("✅ Tag information is still accessible via cbor.tag()");
    println!("✅ Less noise in the output");
    println!(
        "✅ Content is still traversed recursively if it has nested structure"
    );
    println!("✅ Consistent with map key-value pair handling");
}
