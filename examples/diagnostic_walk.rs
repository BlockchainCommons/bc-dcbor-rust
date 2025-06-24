use dcbor::{prelude::*, walk::{EdgeType, WalkElement}};

fn main() {
    // Test with various CBOR structures to show diagnostic_flat works during
    // traversal
    let test_cases = vec![
        ("Simple array", CBOR::from(vec![1, 2, 3])),
        ("Simple map", {
            let mut map = Map::new();
            map.insert("key", "value");
            CBOR::from(map)
        }),
        ("Nested structure", {
            let mut map = Map::new();
            map.insert("numbers", vec![1, 2, 3]);
            map.insert("text", "hello");
            CBOR::from(map)
        }),
    ];

    for (name, cbor) in test_cases {
        println!("=== {} ===", name);
        println!("Full diagnostic: {}", cbor.diagnostic_flat());

        println!("Walking elements:");
        let visitor = |element: &WalkElement,
                       level: usize,
                       edge: EdgeType,
                       _state: ()|
         -> ((), bool) {
            let indent = "  ".repeat(level);
            let edge_label = edge.label().unwrap_or_else(|| "root".to_string());
            println!(
                "{}[{}] {}",
                indent,
                edge_label,
                element.diagnostic_flat()
            );
            ((), false)
        };

        cbor.walk((), &visitor);
        println!();
    }
}
