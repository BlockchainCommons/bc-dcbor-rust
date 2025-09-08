use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

fn main() {
    let mut map = Map::new();
    map.insert("first", "normal");
    map.insert("second", "abort");
    map.insert("third", "should_not_see");
    let cbor = CBOR::from(map);

    let visit_log = RefCell::new(Vec::<String>::new());
    let should_abort = RefCell::new(false);

    let visitor = |element: &WalkElement,
                   level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc =
            format!("L{} [{:?}] {}", level, edge, element.diagnostic_flat());

        // If we're already aborting, still log but return true to stop descent
        if *should_abort.borrow() {
            visit_log
                .borrow_mut()
                .push(format!("ABORT_STATE: {}", desc));
            return (state, true);
        }

        visit_log.borrow_mut().push(desc.clone());

        // Check if this triggers abort
        if let WalkElement::Single(cbor) = element {
            if let CBORCase::Text(text) = cbor.as_case() {
                if text == "abort" {
                    *should_abort.borrow_mut() = true;
                    println!("ABORT TRIGGERED at: {}", desc);
                }
            }
        }

        (state, false)
    };

    cbor.walk((), &visitor);

    let log = visit_log.borrow();
    for entry in log.iter() {
        println!("{}", entry);
    }

    let log_str = log.join(" | ");
    println!("\nFull log: {}", log_str);
    println!(
        "Contains 'should_not_see': {}",
        log_str.contains("should_not_see")
    );
}
