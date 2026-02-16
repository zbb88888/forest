// Utility math functions (BigInt wrappers etc)
#[allow(dead_code)]
pub fn safe_multiply(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}
