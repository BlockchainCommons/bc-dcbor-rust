use crate::Tag;

pub trait CBORTagged {
    /// The CBOR tag assocated with this type.
    const CBOR_TAG: Tag;
}
