use dcbor::*;
use indoc::indoc;

fn run(cbor: CBOR,
    description: &str,
    debug_description: &str,
    diagnostic: &str,
    diagnostic_annotated: &str,
    hex: &str,
    hex_annotated: &str)
{
    let mut known_tags = KnownTagsDict::new([]);
    known_tags.insert(Tag::new_with_name(1, "date"));

    assert_eq!(format!("{}", cbor), description);
    assert_eq!(format!("{:?}", cbor), debug_description);
    assert_eq!(format!("{}", cbor.diagnostic()), diagnostic);
    assert_eq!(format!("{}", cbor.diagnostic_opt(true, Some(&known_tags))), diagnostic_annotated);
    assert_eq!(cbor.hex(), hex);
    assert_eq!(cbor.hex_opt(true, Some(&known_tags)), hex_annotated);
}

#[test]
fn format_simple() {
    run(CBOR::FALSE,
        "false",
        "simple(false)",
        "false",
        "false",
        "f4",
        "f4 # false"
    );
    run(CBOR::TRUE,
        "true",
        "simple(true)",
        "true",
        "true",
        "f5",
        "f5 # true"
    );
    run(CBOR::NULL,
        "null",
        "simple(null)",
        "null",
        "null",
        "f6",
        "f6 # null"
    );
    run(Simple::new(100).cbor(),
        "simple(100)",
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
        "0",
        "00",
        "00 # unsigned(0)"
    );

    run(23.cbor(),
        "23",
        "unsigned(23)",
        "23",
        "23",
        "17",
        "17 # unsigned(23)"
    );

    run(65546.cbor(),
        "65546",
        "unsigned(65546)",
        "65546",
        "65546",
        "1a0001000a",
        "1a0001000a # unsigned(65546)"
    );

    run(1000000000.cbor(),
        "1000000000",
        "unsigned(1000000000)",
        "1000000000",
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
        "-1",
        "20",
        "20 # negative(-1)"
    );

    run((-1000).cbor(),
        "-1000",
        "negative(-1000)",
        "-1000",
        "-1000",
        "3903e7",
        "3903e7 # negative(-1000)"
    );

    run((-1000000).cbor(),
        "-1000000",
        "negative(-1000000)",
        "-1000000",
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
    let a = CBOR::tagged_value(100, "Hello").cbor();
    run(a,
        r#"100("Hello")"#,
        r#"tagged(100, text("Hello"))"#,
        r#"100("Hello")"#,
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
    run(Date::from_timestamp(-100).cbor(),
        "1(-100)",
        "tagged(1, negative(-100))",
        "1(-100)",
        "1(1969-12-31T23:58:20Z)   ; date",
        "c13863",
        indoc! {"
        c1      # tag(1)   ; date
           3863 # negative(-100)
        "}.trim()
    );

    run(Date::from_timestamp(1675854714).cbor(),
        "1(1675854714)",
        "tagged(1, unsigned(1675854714))",
        "1(1675854714)",
        "1(2023-02-08T11:11:54Z)   ; date",
        "c11a63e3837a",
        indoc! {"
        c1            # tag(1)   ; date
           1a63e3837a # unsigned(1675854714)
        "}.trim()
    );
}

#[test]
fn format_structure() {
    let encoded_cbor_hex = &"d83183015829536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e82d902c3820158402b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710ad902c3820158400f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900";
    let cbor = CBOR::from_hex(encoded_cbor_hex).unwrap();
    let description = "49([1, h'536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e', [707([1, h'2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a']), 707([1, h'0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900'])]])";
    let debug_description = "tagged(49, array([unsigned(1), bytes(536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e), array([tagged(707, array([unsigned(1), bytes(2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a)])), tagged(707, array([unsigned(1), bytes(0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900)]))])]))";
    let diagnostic = indoc! {"
    49(
       [
          1,
          h'536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e',
          [
             707(
                [
                   1,
                   h'2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a'
                ]
             ),
             707(
                [
                   1,
                   h'0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900'
                ]
             )
          ]
       ]
    )
    "}.trim();
    let hex = &cbor.hex();
    let dump_annotated = indoc! {r#"
    d8 31                                    # tag(49)
       83                                    # array(3)
          01                                 # unsigned(1)
          5829                               # bytes(41)
             536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e # "Some mysteries aren't meant to be solved."
          82                                 # array(2)
             d9 02c3                         # tag(707)
                82                           # array(2)
                   01                        # unsigned(1)
                   5840                      # bytes(64)
                      2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a
             d9 02c3                         # tag(707)
                82                           # array(2)
                   01                        # unsigned(1)
                   5840                      # bytes(64)
                      0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900
    "#}.trim();
    run(cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic,
        hex,
        dump_annotated
    );
}

#[test]
fn format_structure_2() {
    let encoded_cbor_hex = &"d9012ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e";
    let cbor = CBOR::from_hex(encoded_cbor_hex).unwrap();
    let description = r#"300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})"#;
    let debug_description = r#"tagged(300, map({0x01: (unsigned(1), bytes(59f2293a5bce7d4de59e71b4207ac5d2)), 0x02: (unsigned(2), tagged(1, unsigned(1614124800))), 0x03: (unsigned(3), text("Dark Purple Aqua Love")), 0x04: (unsigned(4), text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."))}))"#;
    let diagnostic = indoc! {r#"
    300(
       {
          1:
          h'59f2293a5bce7d4de59e71b4207ac5d2',
          2:
          1(1614124800),
          3:
          "Dark Purple Aqua Love",
          4:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
       }
    )
    "#}.trim();
    let diagnostic_annotated = indoc! {r#"
    300(
       {
          1:
          h'59f2293a5bce7d4de59e71b4207ac5d2',
          2:
          1(2021-02-24T00:00:00Z)   ; date,
          3:
          "Dark Purple Aqua Love",
          4:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
       }
    )
    "#}.trim();
    let hex = &cbor.hex();
    let dump_annotated = indoc! {r#"
    d9 012c                                  # tag(300)
       a4                                    # map(4)
          01                                 # unsigned(1)
          50                                 # bytes(16)
             59f2293a5bce7d4de59e71b4207ac5d2
          02                                 # unsigned(2)
          c1                                 # tag(1)   ; date
             1a60359700                      # unsigned(1614124800)
          03                                 # unsigned(3)
          75                                 # text(21)
             4461726b20507572706c652041717561204c6f7665 # "Dark Purple Aqua Love"
          04                                 # unsigned(4)
          78 7b                              # text(123)
             4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e # "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    "#}.trim();
    run(cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic_annotated,
        hex,
        dump_annotated
    );
}

#[test]
fn format_key_order() {
    let mut m = Map::new();
    m.insert_into(-1, 3);
    m.insert_into(vec![-1], 7);
    m.insert_into("z", 4);
    m.insert_into(10, 1);
    m.insert_into(false, 8);
    m.insert_into(100, 2);
    m.insert_into("aa", 5);
    m.insert_into(vec![100], 6);

    let cbor = m.cbor();
    let description = r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#;
    let debug_description = r#"map({0x0a: (unsigned(10), unsigned(1)), 0x1864: (unsigned(100), unsigned(2)), 0x20: (negative(-1), unsigned(3)), 0x617a: (text("z"), unsigned(4)), 0x626161: (text("aa"), unsigned(5)), 0x811864: (array([unsigned(100)]), unsigned(6)), 0x8120: (array([negative(-1)]), unsigned(7)), 0xf4: (simple(false), unsigned(8))})"#;
    let diagnostic = indoc! {r#"
    {
       10:
       1,
       100:
       2,
       -1:
       3,
       "z":
       4,
       "aa":
       5,
       [100]:
       6,
       [-1]:
       7,
       false:
       8
    }
    "#}.trim();
    let hex = &cbor.hex();
    let dump_annotated = indoc! {r#"
    a8         # map(8)
       0a      # unsigned(10)
       01      # unsigned(1)
       1864    # unsigned(100)
       02      # unsigned(2)
       20      # negative(-1)
       03      # unsigned(3)
       61      # text(1)
          7a   # "z"
       04      # unsigned(4)
       62      # text(2)
          6161 # "aa"
       05      # unsigned(5)
       81      # array(1)
          1864 # unsigned(100)
       06      # unsigned(6)
       81      # array(1)
          20   # negative(-1)
       07      # unsigned(7)
       f4      # false
       08      # unsigned(8)
    "#}.trim();
    run(cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic,
        hex,
        dump_annotated
    );
}
