use sha2::{Digest, Sha256};

pub(super) type PasswordHash = [u8; 32];

pub(super) fn password_hash(password: &[u8]) -> PasswordHash {
    Sha256::digest(password).into()
}

pub(super) fn parse_password_hash(value: &str) -> Result<PasswordHash, String> {
    if value.len() != 64 {
        return Err("ERR ACL password hash must be exactly 64 hexadecimal characters".to_string());
    }

    let mut hash = [0u8; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let pair = std::str::from_utf8(chunk)
            .map_err(|_| "ERR ACL password hash contains non-ASCII data".to_string())?;
        hash[index] = u8::from_str_radix(pair, 16)
            .map_err(|_| "ERR ACL password hash contains non-hexadecimal characters".to_string())?;
    }

    Ok(hash)
}

pub(super) fn format_password_hash(hash: &PasswordHash) -> String {
    let mut out = String::with_capacity(64);
    for byte in hash {
        out.push(nibble_to_hex(byte >> 4));
        out.push(nibble_to_hex(byte & 0x0f));
    }
    out
}

fn nibble_to_hex(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => unreachable!("nibble out of range"),
    }
}
