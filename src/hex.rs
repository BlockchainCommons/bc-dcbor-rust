pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap();
        bytes.push(byte);
    }
    bytes
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}
