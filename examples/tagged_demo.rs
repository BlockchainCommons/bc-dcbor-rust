use dcbor::prelude::*;
use dcbor::walk::WalkElement;

fn main() {
    // Create a tagged value with nested content
    let mut inner_map = Map::new();
    inner_map.insert("name", "Alice");
    inner_map.insert("age", 30);

    let tagged_cbor = CBOR::from(CBORCase::Tagged(
        Tag::from(100u64),
        CBOR::from(inner_map.clone())
    ));

    println!("Tagged CBOR: {}", tagged_cbor.diagnostic());
    println!("\nWalk output:");

    tagged_cbor.walk(0, &|element, depth, edge, count| {
        let indent = "  ".repeat(depth);
        match element {
            WalkElement::Single(cbor) => {
                println!("{}{}[{:?}] {}", indent, count, edge, cbor.diagnostic_flat());
            }
            WalkElement::KeyValue { key, value } => {
                println!("{}{}[{:?}] {}: {}", indent, count, edge,
                    key.diagnostic_flat(), value.diagnostic_flat());
            }
        }
        (count + 1, false)
    });

    println!("\n--- Comparison: Just the map without tagging ---");
    CBOR::from(inner_map).walk(0, &|element, depth, edge, count| {
        let indent = "  ".repeat(depth);
        match element {
            WalkElement::Single(cbor) => {
                println!("{}{}[{:?}] {}", indent, count, edge, cbor.diagnostic_flat());
            }
            WalkElement::KeyValue { key, value } => {
                println!("{}{}[{:?}] {}: {}", indent, count, edge,
                    key.diagnostic_flat(), value.diagnostic_flat());
            }
        }
        (count + 1, false)
    });
}
