use anchor_lang::prelude::*;

// RULE: #[account] handles serialization. #[derive(Default)] simplifies initialization.
#[account]
#[derive(Default)] 
pub struct Market {
    pub authority: Pubkey, 
    pub collateral_mint: Pubkey, 
    pub asset_mint: Pubkey,
    pub total_collateral_deposited: u64,
    pub total_asset_borrowed: u64,
    pub utilization_rate_params: [u128; 3], 
    pub collateral_factor: u64, 
    pub reserved: [u8; 64], 
}

impl Market {
    // RULE: Total size must be deterministic.
    pub const LEN: usize = 240; 
}

#[account]
#[derive(Default)] 
pub struct UserPosition {
    // RULE: Links back to the Market's global state.
    pub market: Pubkey, 
    // RULE: Identifies the position owner.
    pub owner: Pubkey, 
    // RULE: Balances use u64 (SPL Token standard).
    pub deposited_collateral: u64,
    pub borrowed_asset: u64,
    // RULE: Used for calculating accrued interest.
    pub last_update_slot: u64, 
    // Padding ensures fixed size.
    pub reserved: [u8; 64], 
}

impl UserPosition {
    // RULE: Total size must be deterministic.
    pub const LEN: usize = 160; 
}