use crate::core::{Error, Result};

// Safe numeric conversions
pub fn to_u256(value: &[u8]) -> Option<u256> {
    if value.len() != 32 {
        return None;
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(value);
    Some(u256::from_be_bytes(bytes))
}

pub fn to_u128(value: &[u8]) -> Option<u128> {
    if value.len() != 16 {
        return None;
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(value);
    Some(u128::from_be_bytes(bytes))
}

pub fn to_u64(value: &[u8]) -> Option<u64> {
    if value.len() != 8 {
        return None;
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(value);
    Some(u64::from_be_bytes(bytes))
}

// Address conversions
pub fn to_address(value: &[u8]) -> Option<[u8; 20]> {
    if value.len() != 20 {
        return None;
    }
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(value);
    Some(bytes)
}

pub fn address_to_bytes(addr: &[u8; 20]) -> Vec<u8> {
    addr.to_vec()
}

// String conversions
pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn from_hex_string(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).ok()?;
        bytes.push(byte);
    }
    Some(bytes)
}

// Base64 conversions
pub fn to_base64(bytes: &[u8]) -> String {
    base64::encode(bytes)
}

pub fn from_base64(base64: &str) -> Option<Vec<u8>> {
    base64::decode(base64).ok()
}

// JSON conversions
pub fn to_json<T: serde::Serialize>(value: &T) -> Option<String> {
    serde_json::to_string(value).ok()
}

pub fn from_json<T: serde::de::DeserializeOwned>(json: &str) -> Option<T> {
    serde_json::from_str(json).ok()
} 