mod util;

pub mod cbor;

#[cfg(test)]
mod test {
    use crate::{cbor::{tagged::Tagged, cbor::CBOREncodable, decode::decode_cbor}, util::hex::IntoHex};

    #[test]
    fn encode_cbor() {
        let alice = Tagged::new(200, Tagged::new(24, "Alice"));
        let knows = Tagged::new(200, Tagged::new(24, "knows"));
        let bob = Tagged::new(200, Tagged::new(24, "Bob"));
        let knows_bob = Tagged::new(200, Tagged::new(221, [knows, bob]));
        let envelope = Tagged::new(200, [alice, knows_bob]);
        let cbor = envelope.cbor();
        assert_eq!(format!("{}", cbor), r#"200([200(24("Alice")), 200(221([200(24("knows")), 200(24("Bob"))]))])"#);
        let bytes = cbor.encode_cbor();
        assert_eq!(format!("{}", bytes.hex()), "d8c882d8c8d81865416c696365d8c8d8dd82d8c8d818656b6e6f7773d8c8d81863426f62");
        let decoded_cbor = decode_cbor(&bytes).unwrap();
        assert_eq!(cbor, decoded_cbor);
    }
}
