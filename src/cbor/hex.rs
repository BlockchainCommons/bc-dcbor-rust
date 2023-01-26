/// A utility function for decoding a hexadecimal string to a buffer of bytes.
///
/// Panics if the string is not well-formed, lower case hex with no spaces or
/// other characters.
pub fn hex_to_bytes<T>(hex: T) -> Vec<u8> where T: AsRef<str> {
    let hex = hex.as_ref();
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap();
        bytes.push(byte);
    }
    bytes
}

/// A utility function for encoding a buffer of bytes as a hexadecimal string.
pub fn bytes_to_hex<T>(bytes: T) -> String where T: AsRef<[u8]> {
    let mut hex = String::new();
    for byte in bytes.as_ref() {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}
