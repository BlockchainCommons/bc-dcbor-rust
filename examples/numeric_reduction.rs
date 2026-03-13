use dcbor::{Simple, prelude::*};

//   Key behaviors demonstrated:

// - Integer extraction works across all integer and float types that can hold
//   the value.
// - Float with no fractional part (like 42.0) gets numerically reduced to an
//   integer by dCBOR's canonical encoding rules. Because it's stored as an
//   integer, it extracts successfully as both float and integer types.
// - Float with a fractional part (like 1.5) stays as a float in CBOR. It can be
//   extracted to a different float precision (f32) if the value is exactly
//   representable, but extracting to an integer type fails with WrongType since
//   the integer TryFrom only matches Unsigned/Negative CBOR cases.
// - The last section shows how as_case() lets you pattern-match on the three
//   numeric CBORCase variants directly: Unsigned(u64), Negative(u64) (which
//   stores -1 - n), and Simple(Simple::Float(f64)). Everything else hits the _
//   => panic! arm. Note that 42.0f64 prints as unsigned(42) because dCBOR's
//   numeric reduction already converted it to an integer at encoding time.

fn main() {
    // Encode an integer and extract to several precisions.
    let cbor: CBOR = 42.into();
    let a = u8::try_from_cbor(&cbor).unwrap();
    let b = i32::try_from_cbor(&cbor).unwrap();
    let c = u64::try_from_cbor(&cbor).unwrap();
    let d = f64::try_from_cbor(&cbor).unwrap();
    println!("{a} {b} {c} {d}");
    // 42 42 42 42

    // Encode a float with no fractional part.
    // dCBOR numeric reduction encodes 42.0 as integer 42.
    let cbor: CBOR = 42.0f64.into();
    let as_f64 = f64::try_from_cbor(&cbor).unwrap();
    let as_f32 = f32::try_from_cbor(&cbor).unwrap();
    let as_int = i32::try_from_cbor(&cbor).unwrap();
    println!("{as_f64} {as_f32} {as_int}");
    // 42 42 42

    // Encode a float with a fractional part.
    let cbor: CBOR = 1.5f64.into();
    let as_f32 = f32::try_from_cbor(&cbor).unwrap(); // OK
    let as_int = i32::try_from_cbor(&cbor); // fails: WrongType
    println!("{as_f32} {as_int:?}");
    // 1.5 Err(WrongType)

    // Interrogate the CBOR type directly.
    print_numeric(&42.into()); // unsigned(42)
    print_numeric(&(-7).into()); // negative(-7)
    print_numeric(&42.0f64.into()); // unsigned(42) — numeric reduction
    print_numeric(&1.5f64.into()); // float(1.5)
}

fn print_numeric(cbor: &CBOR) {
    match cbor.as_case() {
        CBORCase::Unsigned(n) => println!("unsigned({n})"),
        CBORCase::Negative(n) => println!("negative({})", -1 - (*n as i128)),
        CBORCase::Simple(Simple::Float(n)) => println!("float({n})"),
        _ => panic!("not numeric"),
    }
}
