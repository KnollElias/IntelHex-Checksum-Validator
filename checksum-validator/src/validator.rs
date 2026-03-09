#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationResult {
    pub byte_count: u8,
    pub record_type: u8,
    pub checksum: u8,
    pub computed_checksum: u8,
}

pub fn validate_intel_hex_record(record: &str) -> Result<ValidationResult, String> {
    let trimmed = record.trim();
    if trimmed.is_empty() {
        return Err("record is empty".to_string());
    }

    let payload = trimmed.strip_prefix(':').unwrap_or(trimmed);

    if payload.len() % 2 != 0 {
        return Err("record has an odd number of hex characters".to_string());
    }

    let bytes = parse_hex_bytes(payload)?;

    if bytes.len() < 5 {
        return Err("record is too short".to_string());
    }

    let byte_count = bytes[0] as usize;
    let expected_len = 5 + byte_count;
    if bytes.len() != expected_len {
        return Err(format!(
            "length mismatch: byte count is {byte_count}, but record contains {} data bytes",
            bytes.len().saturating_sub(5)
        ));
    }

    let record_type = bytes[3];
    let checksum = bytes[bytes.len() - 1];
    let sum_without_checksum = bytes[..bytes.len() - 1]
        .iter()
        .fold(0u8, |acc, value| acc.wrapping_add(*value));
    let computed_checksum = (!sum_without_checksum).wrapping_add(1);

    if checksum != computed_checksum {
        return Err(format!(
            "checksum mismatch: expected 0x{computed_checksum:02X}, got 0x{checksum:02X}"
        ));
    } else {
        println!("Checksum is correct!");
    }

    Ok(ValidationResult {
        byte_count: byte_count as u8,
        record_type,
        checksum,
        computed_checksum,
    })
}

fn parse_hex_bytes(payload: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(payload.len() / 2);

    for index in (0..payload.len()).step_by(2) {
        let pair = &payload[index..index + 2];
        let value = u8::from_str_radix(pair, 16)
            .map_err(|_| format!("invalid hex byte '{pair}' at index {index}"))?;
        bytes.push(value);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::validate_intel_hex_record;

    #[test]
    fn validates_known_good_record() {
        let record = ":10010000214601360121470136007EFE09D2190140";
        let result = validate_intel_hex_record(record).expect("should be valid");
        assert_eq!(result.checksum, 0x40);
        assert_eq!(result.computed_checksum, 0x40);
        assert_eq!(result.byte_count, 0x10);
        assert_eq!(result.record_type, 0x00);
    }

    #[test]
    fn rejects_bad_checksum() {
        let record = ":10010000214601360121470136007EFE09D2190141";
        let err = validate_intel_hex_record(record).expect_err("should be invalid");
        assert!(err.contains("checksum mismatch"));
    }

    #[test]
    fn accepts_record_without_colon_prefix() {
        let record = "10010000214601360121470136007EFE09D2190140";
        let result = validate_intel_hex_record(record).expect("should be valid");
        assert_eq!(result.checksum, 0x40);
    }

    #[test]
    fn rejects_non_hex_input() {
        let err = validate_intel_hex_record(":10GG0000214601360121470136007EFE09D2190140")
            .expect_err("should be invalid");
        assert!(err.contains("invalid hex byte"));
    }
}
