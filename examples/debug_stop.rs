use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

fn main() {
    let nested_structure = CBOR::from(vec![
        CBOR::from(vec!["should", "see", "this"]),
        CBOR::from("abort_marker"),
        CBOR::from(vec!["should", "not", "see"]),
    ]);

    let visit_log = RefCell::new(Vec::<String>::new());
    let found_abort = RefCell::new(false);

    let visitor = |element: &WalkElement,
                   level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc =
            format!("L{}: {:?} - {}", level, edge, element.diagnostic_flat());
        visit_log.borrow_mut().push(desc);

        // Check if this is our abort marker
        if let WalkElement::Single(cbor) = element
            && let CBORCase::Text(text) = cbor.as_case()
            && text == "abort_marker"
        {
            *found_abort.borrow_mut() = true;
            println!("Found abort marker!");
            return (state, true);
        }

        // If we've seen the abort marker and this is an array at level 1, stop
        // descent
        let stop = *found_abort.borrow()
            && matches!(element, WalkElement::Single(_))
            && matches!(edge, EdgeType::ArrayElement(2));

        if stop {
            println!("Stopping descent for: {}", element.diagnostic_flat());
        }

        (state, stop)
    };

    nested_structure.walk((), &visitor);

    let log = visit_log.borrow();
    for entry in log.iter() {
        println!("{}", entry);
    }

    println!("\nLevel 2 visits from ArrayElement(2):");
    let level2_visits: Vec<&str> = log
        .iter()
        .filter(|line| {
            line.starts_with("L2:") && line.contains("ArrayElement(2)")
        })
        .map(|s| s.as_str())
        .collect();

    for visit in &level2_visits {
        println!("  {}", visit);
    }

    println!("\nLevel2 visits empty: {}", level2_visits.is_empty());
}
