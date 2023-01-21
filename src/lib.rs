mod util;

pub mod cbor;

#[cfg(test)]
mod test {
    use crate::{cbor::{tagged::Tagged, cbor::IntoCBOR}, util::hex::IntoHex};

    #[test]
    fn cbor_encode() {
        let alice = Tagged::new(200, Tagged::new(24, "Alice"));
        let knows = Tagged::new(200, Tagged::new(24, "knows"));
        let bob = Tagged::new(200, Tagged::new(24, "Bob"));
        let knows_bob = Tagged::new(200, Tagged::new(221, [knows, bob]));
        let envelope = Tagged::new(200, [alice, knows_bob]);
        println!("{}", envelope.cbor());
        println!("{}", envelope.cbor().encode().hex());
    }
}
