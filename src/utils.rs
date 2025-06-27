use crate::hasher::{HashFormat, IDN45};

/// Validates if the original data matches a given IDN45 hash.
///
/// This function intelligently detects the hash format (`Standard`, `Uuid`, or `Short`)
/// and re-calculates the hash accordingly for comparison.
///
/// # Arguments
///
/// * `original_data` - The original, unhashed data.
/// * `given_hash` - The IDN45 hash string to validate against.
/// * `salt` - The optional salt that was used to create the original hash.
///
/// # Returns
///
/// `true` if the recalculated hash matches `given_hash`, `false` otherwise.
pub fn validate_hash(original_data: &[u8], given_hash: &str, salt: Option<&[u8]>) -> bool {
    let format_style = if given_hash.starts_with("IDN45-") && given_hash.len() == 30 {
        HashFormat::Uuid
    } else if given_hash.contains(':') && given_hash.len() == 12 {
        HashFormat::Short
    } else if given_hash.len() == 64 {
        HashFormat::Standard
    } else {
        return false; // Invalid format
    };

    let hasher = IDN45::new(Some(original_data), salt);
    let recalculated_hash = hasher.hexdigest(format_style);

    recalculated_hash == given_hash
}
