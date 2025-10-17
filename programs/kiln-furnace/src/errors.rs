use anchor_lang::prelude::*;

#[error_code]
pub enum KilnError {
    #[msg("Furnace is not active. Ignition required.")]
    FurnaceCold,
    #[msg("Insufficient pressure for permanent fusion. Minimum: 90 PSI")]
    InsufficientPressure,
    #[msg("Thermal energy below required sintering threshold.")]
    ThermalDeficiency,
    #[msg("Safety protocol engaged: Risk of thermal runaway. Max temp exceeded.")]
    ThermalRunawayRisk,
    #[msg("Raw data input contains volatile impurities.")]
    ContaminatedFeedstock,
    #[msg("Unauthorized access. Authority mismatch.")]
    UnauthorizedAccess,
    #[msg("Furnace capacity exceeded. Cool down required.")]
    CapacityExceeded,
}
