import_stdlib!();

use crate::{ CBOR, Simple, Error, Result, CBORCase };

/// # Boolean Values in dCBOR
///
/// dCBOR supports boolean values through the major type 7 (simple values),
/// where `false` is encoded as 0xf4 and `true` as 0xf5.
///
/// Per the dCBOR specification, only a limited set of simple values are valid:
/// - Boolean values (`true`, `false`)
/// - `null`
/// - Floating point values
///
/// ## Examples
///
/// ```
/// use dcbor::prelude::*;
///
/// // Create CBOR from boolean values using `into()`
/// let cbor_true: CBOR = true.into();
/// let cbor_false: CBOR = false.into();
///
/// // Use in collections
/// let array: Vec<CBOR> = vec![true.into(), false.into(), 42.into()];
/// let cbor_array: CBOR = array.into();
///
/// // Maps can use boolean keys
/// let mut map = Map::new();
/// map.insert(true, "this is true");
/// map.insert(false, "this is false");
///
/// // Convert back to boolean
/// let value: bool = cbor_true.try_into().unwrap();
/// assert_eq!(value, true);
/// ```
impl From<bool> for CBOR {
    fn from(value: bool) -> Self {
        match value {
            false => CBORCase::Simple(Simple::False).into(),
            true => CBORCase::Simple(Simple::True).into(),
        }
    }
}

impl TryFrom<CBOR> for bool {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        match cbor.into_case() {
            CBORCase::Simple(Simple::False) => Ok(false),
            CBORCase::Simple(Simple::True) => Ok(true),
            _ => {
                Err(Error::WrongType)
            }
        }
    }
}
