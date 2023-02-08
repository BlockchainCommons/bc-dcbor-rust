use crate::CBOR;

fn run(cbor: CBOR, description: &str, debug_description: &str, diagnostic: &str) {
    assert_eq!(format!("{}", cbor), description);
    assert_eq!(format!("{:?}", cbor), debug_description);
    assert_eq!(format!("{}", cbor.diagnostic()), diagnostic);
}

#[test]
fn format_int() {
    run(CBOR::NULL, "null", "value(null)", "null");
    run(CBOR::TRUE, "true", "value(true)", "true");
    run(CBOR::FALSE, "false", "value(false)", "false");
}
