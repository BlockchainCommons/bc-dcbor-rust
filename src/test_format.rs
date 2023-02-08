use crate::{CBOR, Simple, CBOREncodable, Map, Tagged, Date};
use indoc::indoc;

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
fn format_simple() {
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

#[test]
fn format_unsigned() {
    run(0.cbor(),
        "0",
        "unsigned(0)",
        "0",
        "00",
        "00 # unsigned(0)"
    );

    run(23.cbor(),
        "23",
        "unsigned(23)",
        "23",
        "17",
        "17 # unsigned(23)"
    );

    run(65546.cbor(),
        "65546",
        "unsigned(65546)",
        "65546",
        "1a0001000a",
        "1a0001000a # unsigned(65546)"
    );

    run(1000000000.cbor(),
        "1000000000",
        "unsigned(1000000000)",
        "1000000000",
        "1a3b9aca00",
        "1a3b9aca00 # unsigned(1000000000)"
    );
}

#[test]
fn format_negative() {
    run((-1).cbor(),
        "-1",
        "negative(-1)",
        "-1",
        "20",
        "20 # negative(-1)"
    );

    run((-1000).cbor(),
        "-1000",
        "negative(-1000)",
        "-1000",
        "3903e7",
        "3903e7 # negative(-1000)"
    );

    run((-1000000).cbor(),
        "-1000000",
        "negative(-1000000)",
        "-1000000",
        "3a000f423f",
        "3a000f423f # negative(-1000000)"
    );
}

#[test]
fn format_string() {
    run("Test".cbor(),
        r#""Test""#,
        r#"text("Test")"#,
        r#""Test""#,
        "6454657374",
        indoc! {r#"
        64          # text(4)
           54657374 # "Test"
        "#}.trim()
    )
}

#[test]
fn format_simple_array() {
    run([1, 2, 3].cbor(),
        "[1, 2, 3]",
        "array([unsigned(1), unsigned(2), unsigned(3)])",
        "[1, 2, 3]",
        "83010203",
        indoc! {r#"
        83    # array(3)
           01 # unsigned(1)
           02 # unsigned(2)
           03 # unsigned(3)
        "#}.trim()
    )
}

#[test]
fn format_nested_array() {
    let a = [1, 2, 3].cbor();
    let b = ["A", "B", "C"].cbor();
    let c = [a, b].cbor();
    run(c,
        r#"[[1, 2, 3], ["A", "B", "C"]]"#,
        r#"array([array([unsigned(1), unsigned(2), unsigned(3)]), array([text("A"), text("B"), text("C")])])"#,
        indoc! {r#"
        [
           [1, 2, 3],
           ["A", "B", "C"]
        ]
        "#}.trim(),
        "828301020383614161426143",
        indoc! {r#"
        82          # array(2)
           83       # array(3)
              01    # unsigned(1)
              02    # unsigned(2)
              03    # unsigned(3)
           83       # array(3)
              61    # text(1)
                 41 # "A"
              61    # text(1)
                 42 # "B"
              61    # text(1)
                 43 # "C"
        "#}.trim()
    )
}

#[test]
fn format_map() {
    let mut map = Map::new();
    map.insert_into(1, "A");
    map.insert_into(2, "B");
    run(map.cbor(),
        r#"{1: "A", 2: "B"}"#,
        r#"map({0x01: (unsigned(1), text("A")), 0x02: (unsigned(2), text("B"))})"#,
        r#"{1: "A", 2: "B"}"#,
        "a2016141026142",
        indoc! {r#"
        a2       # map(2)
           01    # unsigned(1)
           61    # text(1)
              41 # "A"
           02    # unsigned(2)
           61    # text(1)
              42 # "B"
        "#}.trim()
    )
}

#[test]
fn format_tagged() {
    let a = Tagged::new(100, "Hello").cbor();
    run(a,
        r#"100("Hello")"#,
        r#"tagged(100, text("Hello"))"#,
        r#"100("Hello")"#,
        "d8646548656c6c6f",
        indoc! {r#"
        d8 64            # tag(100)
           65            # text(5)
              48656c6c6f # "Hello"
        "#}.trim()
    )
}

#[test]
fn format_date() {
    let date = Date::from_timestamp(1675854714).cbor();
    run(date,
        "1(1675854714)",
        "tagged(1, unsigned(1675854714))",
        "1(1675854714)",
        "c11a63e3837a",
        indoc! {"
        c1            # tag(1)
           1a63e3837a # unsigned(1675854714)
        "}.trim()
    )
}
