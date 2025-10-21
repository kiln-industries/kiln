//! Thermodynamics calculation module for KILN sintering operations.
//! 
//! This module provides the core algorithms for calculating required thermal
//! energy based on data complexity and desired pressure levels.

use blake3::Hasher;

/// Thermal configuration constants for sintering operations
pub struct ThermalConfig {
    /// Base temperature required for any sintering (Kelvin)
    pub base_temp: u64,
    /// Temperature coefficient per pressure unit
    pub pressure_coefficient: f64,
    /// Entropy factor from data hash
    pub entropy_multiplier: f64,
}

impl Default for ThermalConfig {
    fn default() -> Self {
        Self {
            base_temp: 2000,           // 2000K base
            pressure_coefficient: 8.5, // 8.5K per pressure unit
            entropy_multiplier: 0.15,  // 15% entropy factor
        }
    }
}

/// Calculate the required heat for sintering a data block.
/// 
/// The formula combines:
/// - Base sintering temperature
/// - Pressure-dependent thermal requirement
/// - Data entropy factor (derived from hash distribution)
/// 
/// # Arguments
/// * `data_hash` - 32-byte Blake3 hash of the raw data
/// * `pressure` - Pressure level (0-255, minimum 90 for valid sintering)
/// 
/// # Returns
/// Required temperature in Kelvin (u64)
pub fn calc_required_heat(data_hash: &[u8; 32], pressure: u8) -> u64 {
    let config = ThermalConfig::default();
    
    // Calculate entropy factor from hash byte distribution
    let entropy = calculate_entropy(data_hash);
    
    // Base + (pressure * coefficient) + (entropy * multiplier * base)
    let pressure_component = (pressure as f64) * config.pressure_coefficient;
    let entropy_component = entropy * config.entropy_multiplier * (config.base_temp as f64);
    
    let total = (config.base_temp as f64) + pressure_component + entropy_component;
    
    total.round() as u64
}

/// Calculate Shannon entropy of the data hash.
/// Higher entropy = more random data = more energy required.
fn calculate_entropy(data: &[u8; 32]) -> f64 {
    let mut freq = [0u32; 256];
    for &byte in data.iter() {
        freq[byte as usize] += 1;
    }
    
    let len = data.len() as f64;
    let mut entropy = 0.0f64;
    
    for &count in freq.iter() {
        if count > 0 {
            let p = (count as f64) / len;
            entropy -= p * p.log2();
        }
    }
    
    // Normalize to 0-1 range (max entropy for 32 bytes is ~5 bits)
    (entropy / 5.0).min(1.0)
}

/// Heat dissipation rate calculation for cooldown operations
pub fn calc_cooldown_rate(current_temp: u64, ambient_temp: u64) -> u64 {
    // Newton's law of cooling approximation
    let delta = current_temp.saturating_sub(ambient_temp);
    let rate = (delta as f64 * 0.05).round() as u64;
    rate.max(10) // Minimum 10K per cycle
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_required_heat_minimum() {
        let hash = [0u8; 32];
        let heat = calc_required_heat(&hash, 90);
        assert!(heat >= 2000, "Heat should be at least base temp");
    }
    
    #[test]
    fn test_pressure_increases_heat() {
        let hash = [0u8; 32];
        let low_pressure = calc_required_heat(&hash, 90);
        let high_pressure = calc_required_heat(&hash, 255);
        assert!(high_pressure > low_pressure);
    }
}
