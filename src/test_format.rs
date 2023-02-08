use crate::{CBOR, Simple, CBOREncodable};

fn run(cbor: CBOR,
    description: &str,
    debug_description: &str,
    diagnostic: &str,
    dump: &str,
    dump_annotated: &str)
{
    assert_eq!(format!("{}", cbor), description);
    assert_eq!(format!("{:?}", cbor), debug_description);
    assert_eq!(format!("{}", cbor.diagnostic()), diagnostic);
    assert_eq!(cbor.dump(), dump);
    assert_eq!(cbor.dump_opt(true, None), dump_annotated);
}

#[test]
fn format_int() {
    run(CBOR::FALSE,
        "false",
        "simple(false)",
        "false",
        "f4",
        "f4 # false"
    );
    run(CBOR::TRUE,
        "true",
        "simple(true)",
        "true",
        "f5",
        "f5 # true"
    );
    run(CBOR::NULL,
        "null",
        "simple(null)",
        "null",
        "f6",
        "f6 # null"
    );
    run(Simple::new(100).cbor(),
        "simple(100)",
        "simple(100)",
        "simple(100)",
        "f864",
        "f864 # simple(100)"
    );
}
