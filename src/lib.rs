// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Check if input is at least 8 hex characters (4 bytes)
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }
    let version_hex = &raw_tx_hex[0..8];
    // Decode hex to bytes
    let version_bytes = match hex::decode(version_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Hex decode error".to_string()),
    };
    if version_bytes.len() != 4 {
        return Err("Version field must be 4 bytes long".to_string());
    }
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);
    Ok(version)
}

