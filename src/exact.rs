//! This module is based on the Swift `exactly` initializers.
//! See https://github.com/apple/swift/blob/main/stdlib/public/core/IntegerTypes.swift.gyb
//!
//! To expand the `.gyb` file, run:
//! ```bash
//! $ git clone https://github.com/apple/swift.git
//! $ cd swift
//! $ python3 ./utils/gyb.py -DCMAKE_SIZEOF_VOID_P=8 --line-directive '' -o output_file.swift stdlib/public/core/IntegerTypes.swift.gyb
//! ```

use half::f16;

pub trait ExactFrom {
    /// Creates a target numeric value from the given `f16`, if it can be represented exactly.
    ///
    /// If the value passed as `source` is not representable exactly, the result
    /// is `None`. For example, converting 21.0 will succeed, but 21.5 will fail:
    ///
    ///    use half::f16;
    ///    use exact::ExactFrom;
    ///    assert_eq!(i64::exact_from_f16(f16::from_f64(21.0)), Some(21));
    ///    assert_eq!(i64::exact_from_f16(f16::from_f64(21.5)), None);
    ///
    /// - Parameter source: The value to convert.
    fn exact_from_f16(source: f16) -> Option<Self> where Self: Sized;

    /// Creates a target numeric value from the given `f32`, if it can be represented exactly.
    ///
    /// If the value passed as `source` is not representable exactly, the result
    /// is `None`. For example, converting 21.0 will succeed, but 21.5 will fail:
    ///
    ///    use exact::ExactFrom;
    ///    assert_eq!(i64::exact_from_f32(21.0f32), Some(21));
    ///    assert_eq!(i64::exact_from_f32(21.5f32), None);
    ///
    /// - Parameter source: The value to convert.
    fn exact_from_f32(source: f32) -> Option<Self> where Self: Sized;

    /// Creates a target numeric value from the given `f64`, if it can be represented exactly.
    ///
    /// If the value passed as `source` is not representable exactly, the result
    /// is `None`. For example, converting 21.0 will succeed, but 21.5 will fail:
    ///
    ///    use exact::ExactFrom;
    ///    assert_eq!(i64::exact_from_f64(21.0), Some(21));
    ///    assert_eq!(i64::exact_from_f64(21.5), None);
    ///
    /// - Parameter source: The value to convert.
    fn exact_from_f64(source: f64) -> Option<Self> where Self: Sized;

    /// Creates a target numeric value from the given `u64`, if it can be represented exactly.
    ///
    /// If the value passed as `source` is not representable exactly, the result
    /// is `None`. For example, converting 21 to f64 will succeed, but 9223372036854775809 will fail:
    ///
    ///    use exact::ExactFrom;
    ///    assert_eq!(f64::exact_from_u64(21u64), Some(21.0));
    ///    assert_eq!(f64::exact_from_u64(u64::MAX), Some(1.8446744073709552e19));
    ///    assert_eq!(f64::exact_from_u64(9223372036854775809u64), None);
    ///
    /// - Parameter source: The value to convert.
    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized;

    /// Creates a target numeric value from the given `i64`, if it can be represented exactly.
    ///
    /// If the value passed as `source` is not representable exactly, the result
    /// is `None`. For example, converting 21 to f64 will succeed, but -9223372036854775809 will fail:
    ///
    ///   use exact::ExactFrom;
    ///   assert_eq!(f64::exact_from_i64(21i64), Some(21.0));
    ///   assert_eq!(f64::exact_from_i64(-21i64), Some(-21.0));
    ///   assert_eq!(f64::exact_from_i64(i64::MAX), Some(9.223372036854776e18));
    ///   assert_eq!(f64::exact_from_i64(i64::MIN), Some(-9.223372036854776e18));
    ///   assert_eq!(f64::exact_from_i64(-9223372036854775809i64), None);
    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized;
}

impl ExactFrom for i16 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        if source <= -32768.0 || source >= 32768.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i16)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -32769.0 || source >= 32768.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i16)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -32769.0 || source >= 32768.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i16)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        if source > 32767 {
            return None;
        }
        Some(source as i16)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        if !(-32768..=32767).contains(&source) {
            return None;
        }
        Some(source as i16)
    }
}

impl ExactFrom for i32 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        // A Float16 value, if finite, is always in-range for 32- and 64-bit signed
        // integer types.

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i32)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -2147483904.0 || source >= 2147483648.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i32)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -2147483649.0 || source >= 2147483648.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i32)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        if source > 2147483647 {
            return None;
        }
        Some(source as i32)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        if !(-2147483648..=2147483647).contains(&source) {
            return None;
        }
        Some(source as i32)
    }
}

impl ExactFrom for i64 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        // A Float16 value, if finite, is always in-range for 32- and 64-bit signed
        // integer types.

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i64)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -9223373136366403584.0 || source >= 9223372036854775808.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i64)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -9223372036854777856.0 || source >= 9223372036854775808.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as i64)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        if source > 9223372036854775807 {
            return None;
        }
        Some(source as i64)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        Some(source)
    }
}

impl ExactFrom for u16 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        // A Float16 value, if greater than -1 and finite, is always in-range for
        // 16-, 32-, and 64-bit unsigned integer types.

        if source <= -1.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u16)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 65536.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u16)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 65536.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u16)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        if source > 65535 {
            return None;
        }
        Some(source as u16)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        if !(0..=65535).contains(&source) {
            return None;
        }
        Some(source as u16)
    }
}

impl ExactFrom for u32 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        // A Float16 value, if greater than -1 and finite, is always in-range for
        // 16-, 32-, and 64-bit unsigned integer types.

        if source <= -1.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u32)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 4294967296.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u32)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 4294967296.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u32)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        if source > 4294967295 {
            return None;
        }
        Some(source as u32)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        if !(0..=4294967295).contains(&source) {
            return None;
        }
        Some(source as u32)
    }
}

impl ExactFrom for u64 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        let source = source.to_f64();

        if !source.is_finite() {
            return None;
        }

        // A Float16 value, if greater than -1 and finite, is always in-range for
        // 16-, 32-, and 64-bit unsigned integer types.

        if source <= -1.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u64)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 18446744073709551616.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u64)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if !source.is_finite() {
            return None;
        }

        if source <= -1.0 || source >= 18446744073709551616.0 {
            return None;
        }

        if source.fract() != 0.0 {
            return None;
        }

        Some(source as u64)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        Some(source)
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        if source < 0 {
            return None;
        }
        Some(source as u64)
    }
}

impl ExactFrom for f16 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        if source.is_nan() {
            return Some(f16::NAN);
        }
        Some(source)
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if source.is_nan() {
            return Some(f16::NAN);
        }
        if source.is_infinite() {
            return Some(f16::from_f32(source));
        }
        if source > f16::MAX.to_f32() || source < f16::MIN.to_f32() {
            return None;
        }
        let f = f16::from_f32(source);
        if f.to_f32() == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if source.is_nan() {
            return Some(f16::NAN);
        }
        if source.is_infinite() {
            return Some(f16::from_f64(source));
        }
        if source > f16::MAX.to_f64() || source < f16::MIN.to_f64() {
            return None;
        }
        let f = f16::from_f64(source);
        if f.to_f64() == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        let f = f16::from_f64(source as f64);
        if f.is_infinite() {
            return None;
        }
        if f.to_f64() as u64 == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        let f = f16::from_f64(source as f64);
        if f.is_infinite() {
            return None;
        }
        if f.to_f64() as i64 == source {
            Some(f)
        } else {
            None
        }
    }
}

impl ExactFrom for f32 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        if source.is_nan() {
            return Some(f32::NAN);
        }
        Some(source.to_f32())
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if source.is_nan() {
            return Some(f32::NAN);
        }
        Some(source)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if source.is_nan() {
            return Some(f32::NAN);
        }
        let f = source as f32;
        if f as f64 == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        let f = source as f32;
        if f as u64 == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        let f = source as f32;
        if f as i64 == source {
            Some(f)
        } else {
            None
        }
    }
}

impl ExactFrom for f64 {
    fn exact_from_f16(source: f16) -> Option<Self> {
        if source.is_nan() {
            return Some(f64::NAN);
        }
        Some(source.to_f64())
    }

    fn exact_from_f32(source: f32) -> Option<Self> {
        if source.is_nan() {
            return Some(f64::NAN);
        }
        Some(source as f64)
    }

    fn exact_from_f64(source: f64) -> Option<Self> {
        if source.is_nan() {
            return Some(f64::NAN);
        }
        Some(source)
    }

    fn exact_from_u64(source: u64) -> Option<Self> where Self: Sized {
        let f = source as f64;
        if f as u64 == source {
            Some(f)
        } else {
            None
        }
    }

    fn exact_from_i64(source: i64) -> Option<Self> where Self: Sized {
        let f = source as f64;
        if f as i64 == source {
            Some(f)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_exact_i16() {
        assert_eq!(i16::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(i16::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(i16::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(i16::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(i16::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(i16::exact_from_f32(21.0f32), Some(21));
        assert_eq!(i16::exact_from_f32(21.5f32), None);
        assert_eq!(i16::exact_from_f32(f32::NAN), None);
        assert_eq!(i16::exact_from_f32(f32::INFINITY), None);
        assert_eq!(i16::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(i16::exact_from_f64(21.0), Some(21));
        assert_eq!(i16::exact_from_f64(21.5), None);
        assert_eq!(i16::exact_from_f64(f64::NAN), None);
        assert_eq!(i16::exact_from_f64(f64::INFINITY), None);
        assert_eq!(i16::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(i16::exact_from_u64(21u64), Some(21));
        assert_eq!(i16::exact_from_u64(u64::MAX), None);
        assert_eq!(i16::exact_from_u64(65536u64), None);

        assert_eq!(i16::exact_from_i64(21i64), Some(21));
        assert_eq!(i16::exact_from_i64(-21i64), Some(-21));
        assert_eq!(i16::exact_from_i64(i64::MAX), None);
        assert_eq!(i16::exact_from_i64(i64::MIN), None);
        assert_eq!(i16::exact_from_i64(-65536i64), None);
    }

    #[test]
    fn test_exact_i32() {
        assert_eq!(i32::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(i32::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(i32::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(i32::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(i32::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(i32::exact_from_f32(21.0f32), Some(21));
        assert_eq!(i32::exact_from_f32(21.5f32), None);
        assert_eq!(i32::exact_from_f32(f32::NAN), None);
        assert_eq!(i32::exact_from_f32(f32::INFINITY), None);
        assert_eq!(i32::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(i32::exact_from_f64(21.0), Some(21));
        assert_eq!(i32::exact_from_f64(21.5), None);
        assert_eq!(i32::exact_from_f64(f64::NAN), None);
        assert_eq!(i32::exact_from_f64(f64::INFINITY), None);
        assert_eq!(i32::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(i32::exact_from_u64(21u64), Some(21));
        assert_eq!(i32::exact_from_u64(u64::MAX), None);
        assert_eq!(i32::exact_from_u64(4294967296u64), None);

        assert_eq!(i32::exact_from_i64(21i64), Some(21));
        assert_eq!(i32::exact_from_i64(-21i64), Some(-21));
        assert_eq!(i32::exact_from_i64(i64::MAX), None);
        assert_eq!(i32::exact_from_i64(i64::MIN), None);
        assert_eq!(i32::exact_from_i64(-4294967296i64), None);
    }

    #[test]
    fn test_exact_i64() {
        assert_eq!(i64::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(i64::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(i64::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(i64::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(i64::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(i64::exact_from_f32(21.0f32), Some(21));
        assert_eq!(i64::exact_from_f32(21.5f32), None);
        assert_eq!(i64::exact_from_f32(f32::NAN), None);
        assert_eq!(i64::exact_from_f32(f32::INFINITY), None);
        assert_eq!(i64::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(i64::exact_from_f64(21.0), Some(21));
        assert_eq!(i64::exact_from_f64(21.5), None);
        assert_eq!(i64::exact_from_f64(f64::NAN), None);
        assert_eq!(i64::exact_from_f64(f64::INFINITY), None);
        assert_eq!(i64::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(i64::exact_from_u64(21u64), Some(21));
        assert_eq!(i64::exact_from_u64(u64::MAX), None);
        assert_eq!(i64::exact_from_u64(9223372036854775809u64), None);

        assert_eq!(i64::exact_from_i64(21i64), Some(21));
        assert_eq!(i64::exact_from_i64(-21i64), Some(-21));
        assert_eq!(i64::exact_from_i64(i64::MAX), Some(9223372036854775807));
        assert_eq!(i64::exact_from_i64(i64::MIN), Some(-9223372036854775808));
    }

    #[test]
    fn test_exact_u16() {
        assert_eq!(u16::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(u16::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(u16::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(u16::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(u16::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(u16::exact_from_f32(21.0f32), Some(21));
        assert_eq!(u16::exact_from_f32(21.5f32), None);
        assert_eq!(u16::exact_from_f32(f32::NAN), None);
        assert_eq!(u16::exact_from_f32(f32::INFINITY), None);
        assert_eq!(u16::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(u16::exact_from_f64(21.0), Some(21));
        assert_eq!(u16::exact_from_f64(21.5), None);
        assert_eq!(u16::exact_from_f64(f64::NAN), None);
        assert_eq!(u16::exact_from_f64(f64::INFINITY), None);
        assert_eq!(u16::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(u16::exact_from_u64(21u64), Some(21));
        assert_eq!(u16::exact_from_u64(u64::MAX), None);
        assert_eq!(u16::exact_from_u64(65536u64), None);

        assert_eq!(u16::exact_from_i64(21i64), Some(21));
        assert_eq!(u16::exact_from_i64(-21i64), None);
        assert_eq!(u16::exact_from_i64(i64::MAX), None);
        assert_eq!(u16::exact_from_i64(i64::MIN), None);
        assert_eq!(u16::exact_from_i64(-65536i64), None);
    }

    #[test]
    fn test_exact_u32() {
        assert_eq!(u32::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(u32::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(u32::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(u32::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(u32::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(u32::exact_from_f32(21.0f32), Some(21));
        assert_eq!(u32::exact_from_f32(21.5f32), None);
        assert_eq!(u32::exact_from_f32(f32::NAN), None);
        assert_eq!(u32::exact_from_f32(f32::INFINITY), None);
        assert_eq!(u32::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(u32::exact_from_f64(21.0), Some(21));
        assert_eq!(u32::exact_from_f64(21.5), None);
        assert_eq!(u32::exact_from_f64(f64::NAN), None);
        assert_eq!(u32::exact_from_f64(f64::INFINITY), None);
        assert_eq!(u32::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(u32::exact_from_u64(21u64), Some(21));
        assert_eq!(u32::exact_from_u64(u64::MAX), None);
        assert_eq!(u32::exact_from_u64(4294967296u64), None);

        assert_eq!(u32::exact_from_i64(21i64), Some(21));
        assert_eq!(u32::exact_from_i64(-21i64), None);
        assert_eq!(u32::exact_from_i64(i64::MAX), None);
        assert_eq!(u32::exact_from_i64(i64::MIN), None);
        assert_eq!(u32::exact_from_i64(-4294967296i64), None);
    }

    #[test]
    fn test_exact_u64() {
        assert_eq!(u64::exact_from_f16(f16::from_f64(21.0)), Some(21));
        assert_eq!(u64::exact_from_f16(f16::from_f64(21.5)), None);
        assert_eq!(u64::exact_from_f16(f16::from_f64(f64::NAN)), None);
        assert_eq!(u64::exact_from_f16(f16::from_f64(f64::INFINITY)), None);
        assert_eq!(u64::exact_from_f16(f16::from_f64(f64::NEG_INFINITY)), None);

        assert_eq!(u64::exact_from_f32(21.0f32), Some(21));
        assert_eq!(u64::exact_from_f32(21.5f32), None);
        assert_eq!(u64::exact_from_f32(f32::NAN), None);
        assert_eq!(u64::exact_from_f32(f32::INFINITY), None);
        assert_eq!(u64::exact_from_f32(f32::NEG_INFINITY), None);

        assert_eq!(u64::exact_from_f64(21.0), Some(21));
        assert_eq!(u64::exact_from_f64(21.5), None);
        assert_eq!(u64::exact_from_f64(f64::NAN), None);
        assert_eq!(u64::exact_from_f64(f64::INFINITY), None);
        assert_eq!(u64::exact_from_f64(f64::NEG_INFINITY), None);

        assert_eq!(u64::exact_from_u64(21u64), Some(21));
        assert_eq!(u64::exact_from_u64(u64::MAX), Some(18446744073709551615));

        assert_eq!(u64::exact_from_i64(21i64), Some(21));
        assert_eq!(u64::exact_from_i64(-21i64), None);
        assert_eq!(u64::exact_from_i64(i64::MAX), Some(9223372036854775807));
        assert_eq!(u64::exact_from_i64(i64::MIN), None);
    }

    #[test]
    fn test_exact_f16() {
        assert_eq!(f16::exact_from_f16(f16::from_f64(21.0)), Some(f16::from_f64(21.0)));
        assert_eq!(f16::exact_from_f16(f16::from_f64(21.5)), Some(f16::from_f64(21.5)));
        assert!(f16::exact_from_f16(f16::NAN).unwrap().is_nan());
        assert_eq!(f16::exact_from_f16(f16::INFINITY), Some(f16::INFINITY));
        assert_eq!(f16::exact_from_f16(f16::NEG_INFINITY), Some(f16::NEG_INFINITY));

        assert_eq!(f16::exact_from_f32(21.0f32), Some(f16::from_f64(21.0)));
        assert_eq!(f16::exact_from_f32(21.5f32), Some(f16::from_f64(21.5)));
        assert!(f16::exact_from_f32(f32::NAN).unwrap().is_nan());
        assert_eq!(f16::exact_from_f32(f32::INFINITY), Some(f16::INFINITY));
        assert_eq!(f16::exact_from_f32(f32::NEG_INFINITY), Some(f16::NEG_INFINITY));

        assert_eq!(f16::exact_from_f64(21.0), Some(f16::from_f64(21.0)));
        assert_eq!(f16::exact_from_f64(21.5), Some(f16::from_f64(21.5)));
        assert!(f16::exact_from_f64(f64::NAN).unwrap().is_nan());
        assert_eq!(f16::exact_from_f64(f64::INFINITY), Some(f16::from_f64(f64::INFINITY)));
        assert_eq!(f16::exact_from_f64(f64::NEG_INFINITY), Some(f16::from_f64(f64::NEG_INFINITY)));

        assert_eq!(f16::exact_from_u64(21u64), Some(f16::from_f64(21.0)));
        assert_eq!(f16::exact_from_u64(u64::MAX), None);
        assert_eq!(f16::exact_from_u64(65536u64), None);

        assert_eq!(f16::exact_from_i64(21i64), Some(f16::from_f64(21.0)));
        assert_eq!(f16::exact_from_i64(-21i64), Some(f16::from_f64(-21.0)));
        assert_eq!(f16::exact_from_i64(i64::MAX), None);
        assert_eq!(f16::exact_from_i64(i64::MIN), None);
        assert_eq!(f16::exact_from_i64(-65536i64), None);
    }

    #[test]
    fn test_exact_f32() {
        assert_eq!(f32::exact_from_f16(f16::from_f64(21.0)), Some(21.0f32));
        assert_eq!(f32::exact_from_f16(f16::from_f64(21.5)), Some(21.5f32));
        assert!(f32::exact_from_f16(f16::NAN).unwrap().is_nan());
        assert_eq!(f32::exact_from_f16(f16::INFINITY), Some(f32::INFINITY));
        assert_eq!(f32::exact_from_f16(f16::NEG_INFINITY), Some(f32::NEG_INFINITY));

        assert_eq!(f32::exact_from_f32(21.0f32), Some(21.0f32));
        assert_eq!(f32::exact_from_f32(21.5f32), Some(21.5f32));
        assert!(f32::exact_from_f32(f32::NAN).unwrap().is_nan());
        assert_eq!(f32::exact_from_f32(f32::INFINITY), Some(f32::INFINITY));
        assert_eq!(f32::exact_from_f32(f32::NEG_INFINITY), Some(f32::NEG_INFINITY));

        assert_eq!(f32::exact_from_f64(21.0), Some(21.0f32));
        assert_eq!(f32::exact_from_f64(21.5), Some(21.5f32));
        assert!(f32::exact_from_f64(f64::NAN).unwrap().is_nan());
        assert_eq!(f32::exact_from_f64(f64::INFINITY), Some(f32::INFINITY));
        assert_eq!(f32::exact_from_f64(f64::NEG_INFINITY), Some(f32::NEG_INFINITY));

        assert_eq!(f32::exact_from_u64(21u64), Some(21.0f32));
        assert_eq!(f32::exact_from_u64(u64::MAX), Some(18446744073709551616.0f32));
        assert_eq!(f32::exact_from_u64(9223372036854775809u64), None);

        assert_eq!(f32::exact_from_i64(21i64), Some(21.0f32));
        assert_eq!(f32::exact_from_i64(-21i64), Some(-21.0f32));
        assert_eq!(f32::exact_from_i64(i64::MAX), Some(9223372036854775808.0f32));
        assert_eq!(f32::exact_from_i64(i64::MIN), Some(-9223372036854775808.0f32));
        assert_eq!(f32::exact_from_i64(-9223372036854775807i64), None);
    }

    #[test]
    fn test_exact_f64() {
        assert_eq!(f64::exact_from_f16(f16::from_f64(21.0)), Some(21.0));
        assert_eq!(f64::exact_from_f16(f16::from_f64(21.5)), Some(21.5));
        assert!(f64::exact_from_f16(f16::NAN).unwrap().is_nan());
        assert_eq!(f64::exact_from_f16(f16::INFINITY), Some(f64::INFINITY));
        assert_eq!(f64::exact_from_f16(f16::NEG_INFINITY), Some(f64::NEG_INFINITY));

        assert_eq!(f64::exact_from_f32(21.0f32), Some(21.0));
        assert_eq!(f64::exact_from_f32(21.5f32), Some(21.5));
        assert!(f64::exact_from_f32(f32::NAN).unwrap().is_nan());
        assert_eq!(f64::exact_from_f32(f32::INFINITY), Some(f64::INFINITY));
        assert_eq!(f64::exact_from_f32(f32::NEG_INFINITY), Some(f64::NEG_INFINITY));

        assert_eq!(f64::exact_from_f64(21.0), Some(21.0));
        assert_eq!(f64::exact_from_f64(21.5), Some(21.5));
        assert!(f64::exact_from_f64(f64::NAN).unwrap().is_nan());
        assert_eq!(f64::exact_from_f64(f64::INFINITY), Some(f64::INFINITY));
        assert_eq!(f64::exact_from_f64(f64::NEG_INFINITY), Some(f64::NEG_INFINITY));

        assert_eq!(f64::exact_from_u64(21u64), Some(21.0));
        assert_eq!(f64::exact_from_u64(u64::MAX), Some(18446744073709551616.0));
        assert_eq!(f64::exact_from_u64(9223372036854775809u64), None);

        assert_eq!(f64::exact_from_i64(21i64), Some(21.0));
        assert_eq!(f64::exact_from_i64(-21i64), Some(-21.0));
        assert_eq!(f64::exact_from_i64(i64::MAX), Some(9223372036854775807.0));
        assert_eq!(f64::exact_from_i64(i64::MIN), Some(-9223372036854775808.0));
        assert_eq!(f64::exact_from_i64(-9223372036854775807i64), None);
    }

    #[test]
    fn test_exact_u64_from_f64() {
        fn test_value(n: f64, i: Option<u64>) {
            assert_eq!(u64::exact_from_f64(n), i);
        }

        test_value(1234.0, Some(1234));
        test_value(-1234.0, None);
        test_value(18446744073709550000.0, Some(18446744073709549568));
        test_value(18446744073709552000.0, None);
        test_value(0.0, Some(0));
        test_value(-0.0, Some(0));
        test_value(0.5, None);
        test_value(-0.5, None);
        test_value(f64::NAN, None);
        test_value(f64::INFINITY, None);
        test_value(f64::NEG_INFINITY, None);

        // Largest and smallest positive integers exactly representable in f64
        test_value(9007199254740991.0, Some(9007199254740991)); // 2^53 - 1
        test_value(1.0, Some(1));

        // Subnormal numbers
        test_value(5e-324, None); // Smallest positive subnormal f64 number

        // Boundary conditions
        test_value(u64::MAX as f64, None); // Largest u64 as f64 - cannot be exactly converted back
        test_value((u64::MAX - 1) as f64, None); // Close to u64::MAX, precision loss expected
        test_value((u64::MAX - 2) as f64, None); // Close to u64::MAX, precision loss expected

        // Precision limits
        test_value(1.0000000000000002, None); // 1 + smallest increment > 1 in f64
        test_value(4503599627370495.5, None); // 2^52 - 0.5

        // Special floating-point values
        test_value(f64::MIN_POSITIVE, None); // Smallest positive normal f64 number FAILING
        test_value(f64::MAX, None); // Largest positive f64 number
    }

    #[test]
    fn test_exact_i64_from_f64_exact() {
        fn test_value(n: f64, i: Option<i64>) {
            assert_eq!(i64::exact_from_f64(n), i);
        }

        test_value(0.0, Some(0));
        test_value(-0.0, Some(0));
        test_value(0.5, None);
        test_value(-0.5, None);
        test_value(1234.0, Some(1234));
        test_value(-1234.0, Some(-1234));
        test_value(f64::NAN, None);
        test_value(f64::INFINITY, None);
        test_value(f64::NEG_INFINITY, None);

        // Max and Min i64 values
        test_value(i64::MAX as f64, None);
        test_value(i64::MIN as f64, Some(i64::MIN));

        // Subnormal numbers
        test_value(1e-308, None); // A typical subnormal number
        test_value(-1e-308, None);

        // Boundary values
        test_value((i64::MAX as f64) + 1.0, None);
        // test_value((i64::MIN as f64) - 1.0, None);

        // Power of two
        test_value(1024.0, Some(1024));
        test_value(-1024.0, Some(-1024));

        // Non-integer floating points
        test_value(1234.56, None);
        test_value(-1234.56, None);

        // Large integers
        test_value(9007199254740991.0, Some(9007199254740991)); // Largest exact integer in f64
        test_value(-9007199254740991.0, Some(-9007199254740991));

        test_value(-9223372036854774784.0, Some(-9223372036854774784)); // Most negative double that converts to int64.
    }
}
