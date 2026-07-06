//! Source file decoding: tolerant handling of BOMs and UTF-16 encodings.
//!
//! `.hyp` files written on Windows (e.g. by PowerShell or Notepad) are often
//! UTF-16 encoded or carry a UTF-8 byte-order mark. Instead of failing with
//! "stream did not contain valid UTF-8", the toolchain decodes these
//! transparently.

/// Decode raw source bytes into a string.
///
/// Supported encodings, detected in order:
/// - UTF-8 with BOM (`EF BB BF`, stripped)
/// - UTF-16 LE / UTF-16 BE with BOM (`FF FE` / `FE FF`)
/// - UTF-16 LE without BOM (heuristic: ASCII text has NUL high bytes)
/// - Plain UTF-8
pub fn decode_source(bytes: &[u8]) -> Result<String, String> {
    if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        return String::from_utf8(rest.to_vec())
            .map_err(|e| format!("Invalid UTF-8 after byte-order mark: {}", e));
    }

    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        return decode_utf16(rest, u16::from_le_bytes);
    }

    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        return decode_utf16(rest, u16::from_be_bytes);
    }

    // BOM-less UTF-16 LE heuristic: ASCII source encoded as UTF-16 LE has a
    // NUL as every second byte. Real UTF-8 source never contains NUL bytes.
    if looks_like_utf16_le(bytes) {
        return decode_utf16(bytes, u16::from_le_bytes);
    }

    String::from_utf8(bytes.to_vec()).map_err(|e| format!("Source is not valid UTF-8: {}", e))
}

fn looks_like_utf16_le(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || !bytes.len().is_multiple_of(2) {
        return false;
    }
    let sample = &bytes[..bytes.len().min(256)];
    let nul_high_bytes = sample
        .chunks_exact(2)
        .filter(|pair| pair[0] != 0 && pair[1] == 0)
        .count();
    // If the clear majority of code units are ASCII-with-NUL-high-byte,
    // this is almost certainly UTF-16 LE text.
    nul_high_bytes * 2 > sample.len() / 2
}

fn decode_utf16(bytes: &[u8], read_u16: fn([u8; 2]) -> u16) -> Result<String, String> {
    if !bytes.len().is_multiple_of(2) {
        return Err("UTF-16 source has an odd number of bytes".to_string());
    }
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| read_u16([pair[0], pair[1]]))
        .collect();
    String::from_utf16(&units).map_err(|e| format!("Invalid UTF-16 source: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_utf8() {
        assert_eq!(decode_source(b"Focus {} Relax").unwrap(), "Focus {} Relax");
    }

    #[test]
    fn test_utf8_with_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice("Focus".as_bytes());
        assert_eq!(decode_source(&bytes).unwrap(), "Focus");
    }

    #[test]
    fn test_utf16_le_with_bom() {
        let mut bytes = vec![0xFF, 0xFE];
        for unit in "Focus ✨".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_source(&bytes).unwrap(), "Focus ✨");
    }

    #[test]
    fn test_utf16_be_with_bom() {
        let mut bytes = vec![0xFE, 0xFF];
        for unit in "Relax".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        assert_eq!(decode_source(&bytes).unwrap(), "Relax");
    }

    #[test]
    fn test_utf16_le_without_bom() {
        let mut bytes = Vec::new();
        for unit in "Focus { observe 1; } Relax".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        assert_eq!(decode_source(&bytes).unwrap(), "Focus { observe 1; } Relax");
    }

    #[test]
    fn test_invalid_utf8_is_rejected() {
        let error = decode_source(&[0xC3, 0x28, 0x41, 0x42, 0x43]).unwrap_err();
        assert!(error.contains("not valid UTF-8"));
    }
}
