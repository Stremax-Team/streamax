use crate::core::{Error, Result};

// Basic numeric operations with overflow checking
pub fn checked_add(a: u256, b: u256) -> Option<u256> {
    a.checked_add(b)
}

pub fn checked_sub(a: u256, b: u256) -> Option<u256> {
    a.checked_sub(b)
}

pub fn checked_mul(a: u256, b: u256) -> Option<u256> {
    a.checked_mul(b)
}

pub fn checked_div(a: u256, b: u256) -> Option<u256> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Extended math operations
pub fn pow(base: u256, exp: u32) -> Option<u256> {
    base.checked_pow(exp)
}

pub fn sqrt(value: u256) -> u256 {
    if value == 0 {
        return 0;
    }
    
    let mut x = value;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + value / x) / 2;
    }
    
    x
}

// Bitwise operations
pub fn count_ones(value: u256) -> u32 {
    value.count_ones()
}

pub fn count_zeros(value: u256) -> u32 {
    value.count_zeros()
}

pub fn leading_zeros(value: u256) -> u32 {
    value.leading_zeros()
}

pub fn trailing_zeros(value: u256) -> u32 {
    value.trailing_zeros()
}

// Numeric conversions
pub fn to_bytes_be(value: u256) -> [u8; 32] {
    value.to_be_bytes()
}

pub fn from_bytes_be(bytes: &[u8; 32]) -> u256 {
    u256::from_be_bytes(*bytes)
}

pub fn to_bytes_le(value: u256) -> [u8; 32] {
    value.to_le_bytes()
}

pub fn from_bytes_le(bytes: &[u8; 32]) -> u256 {
    u256::from_le_bytes(*bytes)
}

// Math constants
pub const MAX_U256: u256 = u256::MAX;
pub const MIN_U256: u256 = u256::MIN;