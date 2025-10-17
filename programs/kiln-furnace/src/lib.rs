use anchor_lang::prelude::*;
use thermal_core::thermodynamics::{calc_required_heat, ThermalConfig};

mod state;
mod errors;

pub use state::*;
pub use errors::*;

declare_id!("HiB644K5eP8nwJhgDaWrHWQVBsCCsuCpGArnLL9mggRe");

#[program]
pub mod kiln_furnace {
    use super::*;

    /// Ignite the furnace. Initializes optimal operating temperature.
    pub fn ignite(ctx: Context<IgniteFurnace>, initial_temp: u64) -> Result<()> {
        let furnace = &mut ctx.accounts.furnace;
        furnace.authority = ctx.accounts.authority.key();
        furnace.current_temp = initial_temp;
        furnace.max_rated_temp = 5000; // 5000K safety ceiling
        furnace.is_active = true;
        furnace.total_sintered_blocks = 0;
        furnace.bump = ctx.bumps.furnace;
        
        emit!(FurnaceIgnited {
            authority: furnace.authority,
            initial_temp,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("KILN IGNITION SEQUENCE INITIATED. Temp: {}K", initial_temp);
        Ok(())
    }

    /// The core Sintering process.
    /// Takes raw data, applies immense heat/pressure, and fuses it permanently.
    pub fn sinter_batch(
        ctx: Context<SinterBatch>, 
        raw_data_hash: [u8; 32], 
        pressure_level: u8
    ) -> Result<()> {
        let furnace = &mut ctx.accounts.furnace;
        let sintered_block = &mut ctx.accounts.sintered_block;
        let clock = Clock::get()?;

        require!(furnace.is_active, KilnError::FurnaceCold);
        require!(pressure_level >= 90, KilnError::InsufficientPressure);

        // Calculate required thermal energy for permanent fusion
        let required_heat = calc_required_heat(&raw_data_hash, pressure_level);
        require!(furnace.current_temp >= required_heat, KilnError::ThermalDeficiency);
        require!(furnace.current_temp <= furnace.max_rated_temp, KilnError::ThermalRunawayRisk);

        // Finalize the block structure (The immutable artifact)
        sintered_block.data_hash = raw_data_hash;
        sintered_block.sintered_at = clock.unix_timestamp;
        sintered_block.pressure_applied = pressure_level;
        sintered_block.final_temperature = furnace.current_temp;
        sintered_block.furnace = furnace.key();
        
        furnace.total_sintered_blocks += 1;

        emit!(BlockSintered {
            block_id: sintered_block.key(),
            data_hash: raw_data_hash,
            temperature: furnace.current_temp,
            pressure: pressure_level,
            slot: clock.slot,
        });

        msg!("BLOCK SINTERED SUCCESSFULLY. Hardened artifact created at slot {}.", clock.slot);
        Ok(())
    }
    
    /// Emergency shutdown procedure
    pub fn emergency_cooldown(ctx: Context<EmergencyCooldown>) -> Result<()> {
        let furnace = &mut ctx.accounts.furnace;
        require!(furnace.is_active, KilnError::FurnaceCold);
        
        furnace.is_active = false;
        furnace.current_temp = 0;
        
        msg!("EMERGENCY COOLDOWN ENGAGED. Furnace offline.");
        Ok(())
    }
}

// =============================================================================
// Account Contexts
// =============================================================================

#[derive(Accounts)]
pub struct IgniteFurnace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Furnace::LEN,
        seeds = [b"furnace", authority.key().as_ref()],
        bump
    )]
    pub furnace: Account<'info, Furnace>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SinterBatch<'info> {
    #[account(
        mut,
        seeds = [b"furnace", authority.key().as_ref()],
        bump = furnace.bump,
        has_one = authority
    )]
    pub furnace: Account<'info, Furnace>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + SinteredBlock::LEN,
        seeds = [b"sintered", furnace.key().as_ref(), &furnace.total_sintered_blocks.to_le_bytes()],
        bump
    )]
    pub sintered_block: Account<'info, SinteredBlock>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyCooldown<'info> {
    #[account(
        mut,
        seeds = [b"furnace", authority.key().as_ref()],
        bump = furnace.bump,
        has_one = authority
    )]
    pub furnace: Account<'info, Furnace>,
    
    pub authority: Signer<'info>,
}

// =============================================================================
// Events
// =============================================================================

#[event]
pub struct FurnaceIgnited {
    pub authority: Pubkey,
    pub initial_temp: u64,
    pub timestamp: i64,
}

#[event]
pub struct BlockSintered {
    pub block_id: Pubkey,
    pub data_hash: [u8; 32],
    pub temperature: u64,
    pub pressure: u8,
    pub slot: u64,
}
