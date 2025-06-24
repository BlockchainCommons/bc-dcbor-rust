use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

fn main() {
    println!("=== DCBOR Walk Module Demonstration ===\n");

    // Create a complex nested CBOR structure
    let mut person = Map::new();
    person.insert("name", "Alice Smith");
    person.insert("age", 30);
    person.insert("email", "alice@example.com");

    let mut address = Map::new();
    address.insert("street", "123 Main St");
    address.insert("city", "San Francisco");
    address.insert("zip", 94102);
    person.insert("address", address);

    let mut profile = Map::new();
    profile.insert("skills", vec!["Rust", "CBOR", "Cryptography"]);
    profile.insert("years_experience", 5);
    person.insert("profile", profile);

    let cbor = CBOR::from(person);

    println!("CBOR structure:");
    println!("{}\n", cbor.diagnostic_flat());

    // Demo 1: Ergonomic key-value pattern matching (the main benefit of the new
    // API)
    println!("=== Demo 1: Key-Value Pattern Matching ===");
    let found_patterns = RefCell::new(Vec::new());

    let pattern_matcher = |element: &WalkElement,
                           _level: usize,
                           _edge: EdgeType,
                           _state: ()|
     -> ((), bool) {
        // The WalkElement enum makes key-value pattern matching incredibly
        // ergonomic!
        if let Some((key, value)) = element.as_key_value() {
            if let CBORCase::Text(key_str) = key.as_case() {
                match key_str.as_str() {
                    "email" => {
                        if let CBORCase::Text(email) = value.as_case() {
                            found_patterns
                                .borrow_mut()
                                .push(format!("📧 Email: {}", email));
                        }
                    }
                    "name" => {
                        if let CBORCase::Text(name) = value.as_case() {
                            found_patterns
                                .borrow_mut()
                                .push(format!("👤 Name: {}", name));
                        }
                    }
                    "city" => {
                        if let CBORCase::Text(city) = value.as_case() {
                            found_patterns
                                .borrow_mut()
                                .push(format!("🏙️  City: {}", city));
                        }
                    }
                    "zip" => {
                        if let CBORCase::Unsigned(zip) = value.as_case() {
                            found_patterns
                                .borrow_mut()
                                .push(format!("📮 ZIP: {}", zip));
                        }
                    }
                    _ => {}
                }
            }
        }
        ((), false) // Continue traversal
    };

    cbor.walk((), &pattern_matcher);

    for pattern in found_patterns.borrow().iter() {
        println!("  {}", pattern);
    }
    println!();

    // Demo 2: Tree structure visualization with element types
    println!("=== Demo 2: Tree Structure Visualization ===");
    let tree_visitor = |element: &WalkElement,
                        level: usize,
                        edge: EdgeType,
                        _state: ()|
     -> ((), bool) {
        let indent = "  ".repeat(level);
        let edge_label = edge.label().unwrap_or_else(|| "root".to_string());

        let element_type = match element {
            WalkElement::Single(cbor) => match cbor.as_case() {
                CBORCase::Map(_) => "🗂️  Map",
                CBORCase::Array(_) => "📚 Array",
                CBORCase::Text(_) => "📝 Text",
                CBORCase::Unsigned(_) => "🔢 Number",
                CBORCase::Tagged(_, _) => "🏷️  Tagged",
                _ => "❓ Other",
            },
            WalkElement::KeyValue { .. } => "🔗 Key-Value",
        };

        println!(
            "{}[{}] {} {}",
            indent,
            edge_label,
            element_type,
            element.diagnostic_flat()
        );
        ((), false)
    };

    cbor.walk((), &tree_visitor);
    println!();

    // Demo 3: Collecting statistics with state
    println!("=== Demo 3: Statistics Collection ===");

    #[derive(Clone, Debug)]
    struct Stats {
        total_elements: usize,
        key_value_pairs: usize,
        text_values: usize,
        numeric_values: usize,
        nested_structures: usize,
        max_depth: usize,
    }

    let stats = RefCell::new(Stats {
        total_elements: 0,
        key_value_pairs: 0,
        text_values: 0,
        numeric_values: 0,
        nested_structures: 0,
        max_depth: 0,
    });

    let stats_collector = |element: &WalkElement,
                           level: usize,
                           _edge: EdgeType,
                           state: Stats|
     -> (Stats, bool) {
        let mut new_stats = state;
        new_stats.total_elements += 1;
        new_stats.max_depth = new_stats.max_depth.max(level);

        match element {
            WalkElement::KeyValue { .. } => new_stats.key_value_pairs += 1,
            WalkElement::Single(cbor) => match cbor.as_case() {
                CBORCase::Text(_) => new_stats.text_values += 1,
                CBORCase::Unsigned(_) | CBORCase::Negative(_) => {
                    new_stats.numeric_values += 1
                }
                CBORCase::Map(_) | CBORCase::Array(_) => {
                    new_stats.nested_structures += 1
                }
                _ => {}
            },
        }

        // Update the shared stats for final reporting
        *stats.borrow_mut() = new_stats.clone();
        (new_stats, false)
    };

    cbor.walk(
        Stats {
            total_elements: 0,
            key_value_pairs: 0,
            text_values: 0,
            numeric_values: 0,
            nested_structures: 0,
            max_depth: 0,
        },
        &stats_collector,
    );

    let final_stats = stats.borrow();
    println!("  📊 Total elements: {}", final_stats.total_elements);
    println!("  🔗 Key-value pairs: {}", final_stats.key_value_pairs);
    println!("  📝 Text values: {}", final_stats.text_values);
    println!("  🔢 Numeric values: {}", final_stats.numeric_values);
    println!("  🗂️  Nested structures: {}", final_stats.nested_structures);
    println!("  📏 Maximum depth: {}", final_stats.max_depth);
    println!();

    // Demo 4: Early termination for search
    println!("=== Demo 4: Early Termination Search ===");
    let search_results = RefCell::new(Vec::new());

    let search_for_email = |element: &WalkElement,
                            _level: usize,
                            _edge: EdgeType,
                            _state: ()|
     -> ((), bool) {
        if let Some((key, value)) = element.as_key_value() {
            if let (CBORCase::Text(k), CBORCase::Text(v)) =
                (key.as_case(), value.as_case())
            {
                if k.as_str() == "email" {
                    search_results
                        .borrow_mut()
                        .push(format!("Found email: {}", v));
                    return ((), true); // Stop traversal after finding the first email
                }
            }
        }
        ((), false)
    };

    cbor.walk((), &search_for_email);

    for result in search_results.borrow().iter() {
        println!("  ✅ {}", result);
    }
    println!("  ⏹️  Stopped traversal early after finding target\n");

    // Demo 5: Comparing old vs new API benefits
    println!("=== Demo 5: API Benefits Summary ===");
    println!("🎯 Key-Value Pattern Matching:");
    println!("  ✅ NEW: Direct access to (key, value) pairs");
    println!(
        "  ❌ OLD: Required external state management to correlate keys and values"
    );
    println!();
    println!("🔍 Element Type Detection:");
    println!("  ✅ NEW: Single enum match on WalkElement");
    println!("  ❌ OLD: Always had to check CBOR type first");
    println!();
    println!("📝 Code Simplicity:");
    println!("  ✅ NEW: ~50% less code for pattern matching");
    println!("  ❌ OLD: Complex state tracking for map traversal");
    println!();
    println!("🏗️  Suitable for dcbor-pattern crate: ✅");
}
