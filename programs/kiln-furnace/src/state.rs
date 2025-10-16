use anchor_lang::prelude::*;

/// The Furnace - core thermal processing unit
#[account]
pub struct Furnace {
    /// The authority who controls this furnace
    pub authority: Pubkey,        // 32 bytes
    /// Current operating temperature in Kelvin
    pub current_temp: u64,        // 8 bytes
    /// Maximum rated temperature (safety ceiling)
    pub max_rated_temp: u64,      // 8 bytes
    /// Total number of blocks sintered by this furnace
    pub total_sintered_blocks: u64, // 8 bytes
    /// Whether the furnace is currently active
    pub is_active: bool,          // 1 byte
    /// PDA bump seed
    pub bump: u8,                 // 1 byte
    /// Reserved for future upgrades
    pub _reserved: [u8; 64],      // 64 bytes
}

impl Furnace {
    // 32 + 8 + 8 + 8 + 1 + 1 + 64 = 122 bytes
    pub const LEN: usize = 122;
}

/// A permanently sintered data block - immutable on-chain artifact
#[account]
#[derive(Default)]
pub struct SinteredBlock {
    /// Blake3 hash of the original raw data
    pub data_hash: [u8; 32],      // 32 bytes
    /// Unix timestamp when sintering completed
    pub sintered_at: i64,         // 8 bytes
    /// Pressure level applied (0-255 scale, min 90 for valid sintering)
    pub pressure_applied: u8,     // 1 byte
    /// Temperature at point of fusion (Kelvin)
    pub final_temperature: u64,   // 8 bytes
    /// Reference to the furnace that created this block
    pub furnace: Pubkey,          // 32 bytes
    /// Block sequence number within the furnace
    pub sequence_id: u64,         // 8 bytes
}

impl SinteredBlock {
    // 32 + 8 + 1 + 8 + 32 + 8 = 89 bytes
    pub const LEN: usize = 89;
}
