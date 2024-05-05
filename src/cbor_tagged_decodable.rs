use anyhow::bail;

use crate::{CBOR, CBORError, CBORTagged, CBORCase};

/// A type that can be decoded from CBOR with a specific tag.
///
/// Typically types that implement this trait will only provide the `CBOR_TAG`
/// associated constant and implement the `from_untagged_cbor` function.
pub trait CBORTaggedDecodable: TryFrom<CBOR> + CBORTagged {
    /// Creates an instance of this type by decoding it from untagged CBOR.
    fn from_untagged_cbor(cbor: CBOR) -> anyhow::Result<Self> where Self: Sized;

    /// Creates an instance of this type by decoding it from tagged CBOR.
    fn from_tagged_cbor(cbor: CBOR) -> anyhow::Result<Self> where Self: Sized {
        match cbor.case() {
            CBORCase::Tagged(tag, item) => {
                let cbor_tags = Self::cbor_tags();
                if cbor_tags.iter().any(|t| t == tag) {
                    Self::from_untagged_cbor(*item.clone())
                } else {
                    bail!(CBORError::WrongTag(cbor_tags[0].clone(), tag.clone()))
                }
            },
            _ => bail!(CBORError::WrongType)
        }
    }

    /// Creates an instance of this type by decoding it from binary encoded tagged CBOR.
    fn from_tagged_cbor_data(data: impl AsRef<[u8]>) -> anyhow::Result<Self> where Self: Sized {
        Self::from_tagged_cbor(CBOR::from_data(data).map_err(anyhow::Error::msg)?)
    }

    /// Creates an instance of this type by decoding it from binary encoded untagged CBOR.
    fn from_untagged_cbor_data(data: impl AsRef<[u8]>) -> anyhow::Result<Self> where Self: Sized {
        Self::from_untagged_cbor(CBOR::from_data(data).map_err(anyhow::Error::msg)?)
    }
}
