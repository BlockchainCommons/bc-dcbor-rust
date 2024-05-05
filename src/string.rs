import_stdlib!();

use anyhow::bail;

use crate::{CBOR, CBORDecodable, CBORError, CBORCase};

// impl CBOREncodable for &str {
//     fn cbor(&self) -> CBOR {
//         CBORCase::Text(self.to_string()).into()
//     }

//     fn cbor_data(&self) -> Vec<u8> {
//         let mut buf = self.len().encode_varint(MajorType::Text);
//         for byte in self.bytes() {
//             buf.push(byte);
//         }
//         buf
//     }
// }

// impl CBOREncodable for String {
//     fn cbor(&self) -> CBOR {
//         CBORCase::Text(self.clone()).into()
//     }

//     fn cbor_data(&self) -> Vec<u8> {
//         self.as_str().cbor_data()
//     }
// }

impl CBORDecodable for String {
    fn from_cbor(cbor: &CBOR) -> anyhow::Result<Self> {
        match cbor.case() {
            CBORCase::Text(s) => Ok(s.clone()),
            _ => bail!(CBORError::WrongType),
        }
    }
}

impl From<&str> for CBOR {
    fn from(value: &str) -> Self {
        CBORCase::Text(value.to_string()).into()
    }
}

impl From<String> for CBOR {
    fn from(value: String) -> Self {
        CBORCase::Text(value.clone()).into()
    }
}

impl From<CBOR> for String {
    fn from(value: CBOR) -> Self {
        Self::from_cbor(&value).unwrap()
    }
}
