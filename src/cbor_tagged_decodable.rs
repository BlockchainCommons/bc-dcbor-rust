use anyhow::{bail, Result, Error};

use crate::{CBOR, CBORError, CBORTagged, CBORCase};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: TryFrom<CBOR> + CBORTagged {
    /// Creates an instance of this type by decoding it from untagged CBOR.
    fn from_untagged_cbor(cbor: CBOR) -> Result<Self> where Self: Sized;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: CBOR) -> Result<Self> where Self: Sized {
        match cbor.into_case() {
            CBORCase::Tagged(tag, item) => {
                let cbor_tags = Self::cbor_tags();
                if cbor_tags.iter().any(|t| *t == tag) {
                    Self::from_untagged_cbor(item)
                } else {
                    bail!(CBORError::WrongTag(cbor_tags[0].clone(), tag))
                }
            },
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: impl AsRef<[u8]>) -> Result<Self> where Self: Sized {
        Self::from_tagged_cbor(CBOR::try_from_data(data).map_err(Error::msg)?)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: impl AsRef<[u8]>) -> Result<Self> where Self: Sized {
        Self::from_untagged_cbor(CBOR::try_from_data(data).map_err(Error::msg)?)
    }
}

// pub trait CBORDisplay: CBORTaggedDecodable + std::fmt::Display {
//     fn format_untagged_cbor(cbor: CBOR) -> Result<String> {
//         let instance = Self::from_untagged_cbor(cbor)?;
//         Ok(instance.to_string())
//     }
// }

// impl<T> CBORDisplay for T where T: CBORTaggedDecodable + std::fmt::Display {}
