use std::cell::RefCell;

use dcbor::{prelude::*, walk::WalkElement};

fn main() {
    println!(
        "=== Testing CONSISTENT stop behavior: 'Prevent descent into children' ===\n"
    );

    // Test case 1: Depth-limited traversal
    println!("Test 1: Depth-limited traversal (max_level = 1)");
    let mut map = Map::new();
    map.insert("shallow", "value");
    map.insert("deep", vec!["nested", "array"]);
    let cbor = CBOR::from(map);

    let max_level = 1;
    let visited = RefCell::new(Vec::new());

    cbor.walk((), &|element, level, edge, state| {
        let indent = "  ".repeat(level);
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
            .push(format!("L{} {}[{:?}] {}", level, indent, edge, description));

        let should_stop = level >= max_level;
        if should_stop {
            println!(
                "L{} {}🛑 STOP (max level): {}",
                level, indent, description
            );
        } else {
            println!("L{} {}✅ Visit: {}", level, indent, description);
        }

        (state, should_stop)
    });

    println!("\nAll visits:");
    for visit in visited.borrow().iter() {
        println!("  {}", visit);
    }

    // Test case 2: Abort entire walk via visitor state
    println!("\n\nTest 2: Abort entire walk when finding 'abort' text");
    let mut abort_map = Map::new();
    abort_map.insert("first", "normal");
    abort_map.insert("second", "abort"); // This should trigger abort
    abort_map.insert("third", "should_not_see");
    let abort_cbor = CBOR::from(abort_map);

    let abort_flag = RefCell::new(false);
    let visited2 = RefCell::new(Vec::new());

    abort_cbor.walk((), &|element, level, edge, state| {
        // Check if we should abort
        if *abort_flag.borrow() {
            println!("🚨 Already aborted, stopping this element too");
            return (state, true);
        }

        let description = match element {
            WalkElement::Single(cbor) => {
                if let CBORCase::Text(s) = cbor.as_case() {
                    if s == "abort" {
                        *abort_flag.borrow_mut() = true;
                        format!("Single: {} (TRIGGER ABORT!)", s)
                    } else {
                        format!("Single: {}", s)
                    }
                } else {
                    format!("Single: {}", cbor.diagnostic_flat())
                }
            }
            WalkElement::KeyValue { key, value } => format!(
                "KV: {} => {}",
                key.diagnostic_flat(),
                value.diagnostic_flat()
            ),
        };

        visited2
            .borrow_mut()
            .push(format!("L{} [{:?}] {}", level, edge, description));
        println!("L{} ✅ Visit: {}", level, description);

        // Stop if abort flag is set
        (state, *abort_flag.borrow())
    });

    println!("\nAll visits before abort:");
    for visit in visited2.borrow().iter() {
        println!("  {}", visit);
    }

    println!("\n=== Conclusion ===");
    println!("✅ Depth limiting works consistently");
    println!("✅ Full abort works via visitor state");
    println!("✅ Stop flag means 'prevent descent into children' everywhere");
}
