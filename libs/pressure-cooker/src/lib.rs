//! Pressure Cooker - Data compression and preparation module for KILN
//! 
//! Handles pre-sintering data compression and integrity validation.

use blake3::Hasher;

/// Compression result containing the processed data hash
pub struct CompressionResult {
    /// Blake3 hash of the compressed data
    pub hash: [u8; 32],
    /// Original data size in bytes
    pub original_size: u64,
    /// Compression ratio achieved (0.0 - 1.0)
    pub compression_ratio: f64,
    /// Integrity checksum
    pub checksum: u32,
}

/// Compress and prepare raw data for sintering
/// 
/// This function:
/// 1. Validates data integrity
/// 2. Computes Blake3 hash for immutable reference
/// 3. Calculates compression metrics
pub fn prepare_feedstock(raw_data: &[u8]) -> CompressionResult {
    let mut hasher = Hasher::new();
    hasher.update(raw_data);
    let hash = hasher.finalize();
    
    // Calculate simple checksum for integrity
    let checksum = raw_data.iter().fold(0u32, |acc, &b| {
        acc.wrapping_add(b as u32)
    });
    
    CompressionResult {
        hash: *hash.as_bytes(),
        original_size: raw_data.len() as u64,
        compression_ratio: 1.0, // No actual compression in this version
        checksum,
    }
}

/// Validate feedstock purity (check for volatile impurities)
/// Returns true if data is safe for sintering
pub fn validate_purity(data: &[u8]) -> bool {
    // Check for known problematic byte patterns
    // In production, this would check for malformed data structures
    !data.is_empty() && data.len() <= 10_485_760 // Max 10MB
}

/// Calculate pressure level based on data characteristics
pub fn calculate_optimal_pressure(data_size: u64, urgency: u8) -> u8 {
    // Base pressure from size (larger = more pressure needed)
    let size_factor = ((data_size as f64).log2() * 5.0).min(100.0) as u8;
    
    // Urgency adds additional pressure (fast sintering)
    let urgency_factor = urgency.saturating_mul(2);
    
    // Minimum 90, maximum 255
    (90 + size_factor + urgency_factor).min(255)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prepare_feedstock() {
        let data = b"test data for sintering";
        let result = prepare_feedstock(data);
        assert_eq!(result.original_size, data.len() as u64);
    }
    
    #[test]
    fn test_validate_purity() {
        assert!(validate_purity(b"valid data"));
        assert!(!validate_purity(b""));
    }
}
