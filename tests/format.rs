use dcbor::prelude::*;
use indoc::indoc;

#[allow(clippy::too_many_arguments)]
fn run(
    test_name: &str,
    cbor: CBOR,
    expected_description: &str,
    expected_debug_description: &str,
    expected_diagnostic: &str,
    expected_diagnostic_annotated: &str,
    expected_diagnostic_flat: &str,
    expected_summary: &str,
    expected_hex: &str,
    expected_hex_annotated: &str,
) {
    let description = format!("{}", cbor);
    if expected_description.is_empty() {
        println!("description:");
        println!("{}", description);
    } else {
        if description != expected_description {
            println!("description mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_description);
            println!("  actual  :\n{:?}", description);
        }
        assert_eq!(
            description, expected_description,
            "description in test '{}'",
            test_name
        );
    }

    let debug_description = format!("{:?}", cbor);
    if expected_debug_description.is_empty() {
        println!("debug_description:");
        println!("{}", debug_description);
    } else {
        if debug_description != expected_debug_description {
            println!("debug_description mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_debug_description);
            println!("  actual  :\n{:?}", debug_description);
        }
        assert_eq!(
            debug_description, expected_debug_description,
            "debug_description in test '{}'",
            test_name
        );
    }

    let diagnostic = cbor.diagnostic();
    if expected_diagnostic.is_empty() {
        println!("diagnostic:");
        println!("{}", diagnostic);
    } else {
        if diagnostic != expected_diagnostic {
            println!("diagnostic mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_diagnostic);
            println!("  actual  :\n{:?}", diagnostic);
        }
        assert_eq!(
            diagnostic, expected_diagnostic,
            "diagnostic in test '{}'",
            test_name
        );
    }

    let diagnostic_annotated = cbor.diagnostic_annotated();
    if expected_diagnostic_annotated.is_empty() {
        println!("diagnostic_annotated:");
        println!("{}", diagnostic_annotated);
    } else {
        if diagnostic_annotated != expected_diagnostic_annotated {
            println!("diagnostic_annotated mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_diagnostic_annotated);
            println!("  actual  :\n{:?}", diagnostic_annotated);
        }
        assert_eq!(
            diagnostic_annotated, expected_diagnostic_annotated,
            "diagnostic_annotated in test '{}'",
            test_name
        );
    }

    let diagnostic_flat = cbor.diagnostic_flat();
    if expected_diagnostic_flat.is_empty() {
        println!("diagnostic_flat:");
        println!("{}", diagnostic_flat);
    } else {
        if diagnostic_flat != expected_diagnostic_flat {
            println!("diagnostic_flat mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_diagnostic_flat);
            println!("  actual  :\n{:?}", diagnostic_flat);
        }
        assert_eq!(
            diagnostic_flat, expected_diagnostic_flat,
            "diagnostic_flat in test '{}'",
            test_name
        );
    }

    let summary = cbor.summary();
    if expected_summary.is_empty() {
        println!("summary:");
        println!("{}", summary);
    } else {
        if summary != expected_summary {
            println!("summary mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_summary);
            println!("  actual  :\n{:?}", summary);
        }
        assert_eq!(
            summary, expected_summary,
            "summary in test '{}'",
            test_name
        );
    }

    let hex = cbor.hex();
    if expected_hex.is_empty() {
        println!("hex:");
        println!("{}", hex);
    } else {
        if hex != expected_hex {
            println!("hex mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_hex);
            println!("  actual  :\n{:?}", hex);
        }
        assert_eq!(hex, expected_hex, "hex in test '{}'", test_name);
    }

    let hex_annotated = cbor.hex_annotated();
    if expected_hex_annotated.is_empty() {
        println!("hex_annotated:");
        println!("{}", hex_annotated);
    } else {
        if hex_annotated != expected_hex_annotated {
            println!("hex_annotated mismatch in test '{}':", test_name);
            println!("  expected:\n{:?}", expected_hex_annotated);
            println!("  actual  :\n{:?}", hex_annotated);
        }
        assert_eq!(
            hex_annotated, expected_hex_annotated,
            "hex_annotated in test '{}'",
            test_name
        );
    }
}

#[test]
fn format_simple_1() {
    run(
        "format_simple_1",
        CBOR::r#false(),
        "false",
        "simple(false)",
        "false",
        "false",
        "false",
        "false",
        "f4",
        "f4  # false",
    );
}

#[test]
fn format_simple_2() {
    run(
        "format_simple_2",
        CBOR::r#true(),
        "true",
        "simple(true)",
        "true",
        "true",
        "true",
        "true",
        "f5",
        "f5  # true",
    );
}

#[test]
fn format_simple_3() {
    run(
        "format_simple_3",
        CBOR::null(),
        "null",
        "simple(null)",
        "null",
        "null",
        "null",
        "null",
        "f6",
        "f6  # null",
    );
}

#[test]
fn format_unsigned() {
    run(
        "format_unsigned_0",
        (0).into(),
        "0",
        "unsigned(0)",
        "0",
        "0",
        "0",
        "0",
        "00",
        "00  # unsigned(0)",
    );

    run(
        "format_unsigned_23",
        (23).into(),
        "23",
        "unsigned(23)",
        "23",
        "23",
        "23",
        "23",
        "17",
        "17  # unsigned(23)",
    );

    run(
        "format_unsigned_65546",
        (65546).into(),
        "65546",
        "unsigned(65546)",
        "65546",
        "65546",
        "65546",
        "65546",
        "1a0001000a",
        "1a0001000a  # unsigned(65546)",
    );

    run(
        "format_unsigned_1000000000",
        (1000000000).into(),
        "1000000000",
        "unsigned(1000000000)",
        "1000000000",
        "1000000000",
        "1000000000",
        "1000000000",
        "1a3b9aca00",
        "1a3b9aca00  # unsigned(1000000000)",
    );
}

#[test]
fn format_negative() {
    run(
        "format_negative_neg1",
        (-1).into(),
        "-1",
        "negative(-1)",
        "-1",
        "-1",
        "-1",
        "-1",
        "20",
        "20  # negative(-1)",
    );

    run(
        "format_negative_neg1000",
        (-1000).into(),
        "-1000",
        "negative(-1000)",
        "-1000",
        "-1000",
        "-1000",
        "-1000",
        "3903e7",
        "3903e7  # negative(-1000)",
    );

    run(
        "format_negative_neg1000000",
        (-1000000).into(),
        "-1000000",
        "negative(-1000000)",
        "-1000000",
        "-1000000",
        "-1000000",
        "-1000000",
        "3a000f423f",
        "3a000f423f  # negative(-1000000)",
    );
}

#[test]
// expected-text-output-rubric:
#[rustfmt::skip]
fn format_string() {
    run("format_string",
        "Test".into(),
        r#""Test""#,
        r#"text("Test")"#,
        r#""Test""#,
        r#""Test""#,
        r#""Test""#,
        r#""Test""#,
        "6454657374",
        indoc! {r#"
            64              # text(4)
                54657374    # "Test"
        "#}.trim()
    )
}

#[test]
// expected-text-output-rubric:
#[rustfmt::skip]
fn format_simple_array() {
    run("format_simple_array",
        [1, 2, 3].into(),
        "[1, 2, 3]",
        "array([unsigned(1), unsigned(2), unsigned(3)])",
        "[1, 2, 3]",
        "[1, 2, 3]",
        "[1, 2, 3]",
        "[1, 2, 3]",
        "83010203",
        indoc! {r#"
            83      # array(3)
                01  # unsigned(1)
                02  # unsigned(2)
                03  # unsigned(3)
        "#}.trim()
    )
}

#[test]
// expected-text-output-rubric:
#[rustfmt::skip]
fn format_nested_array() {
    let a: CBOR = [1, 2, 3].into();
    let b = ["A", "B", "C"].into();
    let c = [a, b].into();
    run("format_nested_array",
        c,
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
        r#"[[1, 2, 3], ["A", "B", "C"]]"#,
        r#"[[1, 2, 3], ["A", "B", "C"]]"#,
        "828301020383614161426143",
        indoc! {r#"
            82              # array(2)
                83          # array(3)
                    01      # unsigned(1)
                    02      # unsigned(2)
                    03      # unsigned(3)
                83          # array(3)
                    61      # text(1)
                        41  # "A"
                    61      # text(1)
                        42  # "B"
                    61      # text(1)
                        43  # "C"
        "#}.trim()
    )
}

#[test]
// expected-text-output-rubric:
#[rustfmt::skip]
fn format_map() {
    let mut map = Map::new();
    map.insert(1, "A");
    map.insert(2, "B");
    run("format_map",
        map.into(),
        r#"{1: "A", 2: "B"}"#,
        r#"map({0x01: (unsigned(1), text("A")), 0x02: (unsigned(2), text("B"))})"#,
        r#"{1: "A", 2: "B"}"#,
        r#"{1: "A", 2: "B"}"#,
        r#"{1: "A", 2: "B"}"#,
        r#"{1: "A", 2: "B"}"#,
        "a2016141026142",
        indoc! {r#"
            a2          # map(2)
                01      # unsigned(1)
                61      # text(1)
                    41  # "A"
                02      # unsigned(2)
                61      # text(1)
                    42  # "B"
        "#}.trim()
    )
}

#[test]
// expected-text-output-rubric:
#[rustfmt::skip]
fn format_tagged() {
    let a = CBOR::to_tagged_value(100, "Hello");
    run("format_tagged",
        a,
        r#"100("Hello")"#,
        r#"tagged(100, text("Hello"))"#,
        r#"100("Hello")"#,
        r#"100("Hello")"#,
        r#"100("Hello")"#,
        r#"100("Hello")"#,
        "d8646548656c6c6f",
        indoc! {r#"
            d8 64               # tag(100)
                65              # text(5)
                    48656c6c6f  # "Hello"
        "#}.trim()
    )
}

#[test]
fn format_date() {
    dcbor::register_tags();
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run("format_date_negative",
        Date::from_timestamp(-100.0).into(),
        "date(-100)",
        "tagged(date, negative(-100))",
        "1(-100)",
        "1(-100)   / date /",
        "1(-100)",
        "1969-12-31T23:58:20Z",
        "c13863",
        indoc! {"
            c1          # tag(1) date
                3863    # negative(-100)
        "}.trim()
    );

    // expected-text-output-rubric:
    #[rustfmt::skip]
    run("format_date_positive",
        Date::from_timestamp(1647887071.0).into(),
        "date(1647887071)",
        "tagged(date, unsigned(1647887071))",
        "1(1647887071)",
        "1(1647887071)   / date /",
        "1(1647887071)",
        "2022-03-21T18:24:31Z",
        "c11a6238c2df",
        indoc! {"
            c1              # tag(1) date
                1a6238c2df  # unsigned(1647887071)
        "}.trim()
    );
}

#[test]
fn format_fractional_date() {
    dcbor::register_tags();
    // expected-text-output-rubric:
    #[rustfmt::skip]
    run("format_fractional_date",
        Date::from_timestamp(0.5).into(),
        "date(0.5)",
        "tagged(date, simple(0.5))",
        "1(0.5)",
        "1(0.5)   / date /",
        "1(0.5)",
        "1970-01-01",
        "c1f93800",
        indoc! {"
            c1          # tag(1) date
                f93800  # 0.5
        "}.trim()
    );
}

#[test]
fn format_structure() {
    let encoded_cbor_hex = &"d83183015829536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e82d902c3820158402b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710ad902c3820158400f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900";
    let cbor = CBOR::try_from_hex(encoded_cbor_hex).unwrap();
    let description = "49([1, h'536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e', [707([1, h'2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a']), 707([1, h'0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900'])]])";
    let debug_description = "tagged(49, array([unsigned(1), bytes(536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e), array([tagged(707, array([unsigned(1), bytes(2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a)])), tagged(707, array([unsigned(1), bytes(0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900)]))])]))";
    // expected-text-output-rubric:
    #[rustfmt::skip]
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
    let diagnostic_flat = "49([1, h'536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e', [707([1, h'2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a']), 707([1, h'0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900'])]])";
    let hex = "d83183015829536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e82d902c3820158402b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710ad902c3820158400f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900";
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let hex_annotated = indoc! {r#"
        d8 31                                   # tag(49)
            83                                  # array(3)
                01                              # unsigned(1)
                5829                            # bytes(41)
                    536f6d65206d7973746572696573206172656e2774206d65616e7420746f20626520736f6c7665642e # "Some mysteries aren't meant to be solved."
                82                              # array(2)
                    d9 02c3                     # tag(707)
                        82                      # array(2)
                            01                  # unsigned(1)
                            5840                # bytes(64)
                                2b9238e19eafbc154b49ec89edd4e0fb1368e97332c6913b4beb637d1875824f3e43bd7fb0c41fb574f08ce00247413d3ce2d9466e0ccfa4a89b92504982710a
                    d9 02c3                     # tag(707)
                        82                      # array(2)
                            01                  # unsigned(1)
                            5840                # bytes(64)
                                0f9c7af36804ffe5313c00115e5a31aa56814abaa77ff301da53d48613496e9c51a98b36d55f6fb5634fdb0123910cfa4904f1c60523df41013dc3749b377900
    "#}.trim();
    run(
        "format_structure",
        cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic,
        diagnostic_flat,
        diagnostic_flat,
        hex,
        hex_annotated,
    );
}

#[test]
fn format_structure_2() {
    let encoded_cbor_hex = &"d9012ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e";
    let cbor = CBOR::try_from_hex(encoded_cbor_hex).unwrap();
    let description = r#"300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})"#;
    let debug_description = r#"tagged(300, map({0x01: (unsigned(1), bytes(59f2293a5bce7d4de59e71b4207ac5d2)), 0x02: (unsigned(2), tagged(1, unsigned(1614124800))), 0x03: (unsigned(3), text("Dark Purple Aqua Love")), 0x04: (unsigned(4), text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."))}))"#;
    // expected-text-output-rubric:
    #[rustfmt::skip]
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
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let diagnostic_annotated = indoc! {r#"
        300(
            {
                1:
                h'59f2293a5bce7d4de59e71b4207ac5d2',
                2:
                1(1614124800),   / date /
                3:
                "Dark Purple Aqua Love",
                4:
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
            }
        )
    "#}.trim();
    let diagnostic_flat = r#"300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})"#;
    let summary = r#"300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 2021-02-24, 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})"#;
    let hex = "d9012ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e";
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let hex_annotated = indoc! {r#"
        d9 012c                                 # tag(300)
            a4                                  # map(4)
                01                              # unsigned(1)
                50                              # bytes(16)
                    59f2293a5bce7d4de59e71b4207ac5d2
                02                              # unsigned(2)
                c1                              # tag(1) date
                    1a60359700                  # unsigned(1614124800)
                03                              # unsigned(3)
                75                              # text(21)
                    4461726b20507572706c652041717561204c6f7665 # "Dark Purple Aqua Love"
                04                              # unsigned(4)
                78 7b                           # text(123)
                    4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e # "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    "#}.trim();
    run(
        "format_structure_2",
        cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic_annotated,
        diagnostic_flat,
        summary,
        hex,
        hex_annotated,
    );
}

#[test]
fn format_key_order() {
    let mut m = Map::new();
    m.insert(-1, 3);
    m.insert(vec![-1], 7);
    m.insert("z", 4);
    m.insert(10, 1);
    m.insert(false, 8);
    m.insert(100, 2);
    m.insert("aa", 5);
    m.insert(vec![100], 6);

    let cbor: CBOR = m.into();
    let description = r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#;
    let debug_description = r#"map({0x0a: (unsigned(10), unsigned(1)), 0x1864: (unsigned(100), unsigned(2)), 0x20: (negative(-1), unsigned(3)), 0x617a: (text("z"), unsigned(4)), 0x626161: (text("aa"), unsigned(5)), 0x811864: (array([unsigned(100)]), unsigned(6)), 0x8120: (array([negative(-1)]), unsigned(7)), 0xf4: (simple(false), unsigned(8))})"#;
    // expected-text-output-rubric:
    #[rustfmt::skip]
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
    let diagnostic_flat = r#"{10: 1, 100: 2, -1: 3, "z": 4, "aa": 5, [100]: 6, [-1]: 7, false: 8}"#;
    let hex = "a80a011864022003617a046261610581186406812007f408";
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let hex_annotated = indoc! {r#"
        a8              # map(8)
            0a          # unsigned(10)
            01          # unsigned(1)
            1864        # unsigned(100)
            02          # unsigned(2)
            20          # negative(-1)
            03          # unsigned(3)
            61          # text(1)
                7a      # "z"
            04          # unsigned(4)
            62          # text(2)
                6161    # "aa"
            05          # unsigned(5)
            81          # array(1)
                1864    # unsigned(100)
            06          # unsigned(6)
            81          # array(1)
                20      # negative(-1)
            07          # unsigned(7)
            f4          # false
            08          # unsigned(8)
    "#}.trim();
    run(
        "format_key_order",
        cbor,
        description,
        debug_description,
        diagnostic,
        diagnostic,
        diagnostic_flat,
        diagnostic_flat,
        hex,
        hex_annotated,
    );
}
