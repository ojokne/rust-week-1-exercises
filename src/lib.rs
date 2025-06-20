// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let raw_bytes = match hex::decode(raw_tx_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Hex decode error".to_string()),
    };

    if raw_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    let version_bytes: [u8; 4] = raw_bytes[0..4].try_into().unwrap();

    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
