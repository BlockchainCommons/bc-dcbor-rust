use std::cell::RefCell;

use dcbor::{
    prelude::*,
    walk::{EdgeType, WalkElement},
};

fn main() {
    let mut inner_map = Map::new();
    inner_map.insert("x", vec![1, 2]);
    let mut outer_map = Map::new();
    outer_map.insert("inner", inner_map);
    outer_map.insert("simple", 42);
    let nested = CBOR::from(outer_map);

    let visit_log = RefCell::new(Vec::<String>::new());
    let visitor = |element: &WalkElement,
                   level: usize,
                   edge: EdgeType,
                   state: ()|
     -> ((), bool) {
        let desc =
            format!("L{} [{:?}] {}", level, edge, element.diagnostic_flat());
        visit_log.borrow_mut().push(desc);
        (state, false)
    };

    nested.walk((), &visitor);

    let log = visit_log.borrow();
    for (i, entry) in log.iter().enumerate() {
        println!("{}. {}", i + 1, entry);
    }
    println!("Total count: {}", log.len());
}
