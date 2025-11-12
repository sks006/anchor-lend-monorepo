use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};
use anchor_lang::system_program::System; // Required for System program reference
use anchor_lang::solana_program::sysvar::rent::Rent; // Required for Rent Sysvar

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    // 1. The Signer (The user depositing)
    #[account(mut)]
    pub signer: Signer<'info>,

    // 2. The User's Token Account (Source of funds)
    #[account(mut)]
    pub user_collateral_token_account: Account<'info, TokenAccount>,
    
    // 3. The Protocol's Market Account (The Config)
    pub market: Account<'info, Market>,

    // 4. The UserPosition PDA (The user's database entry)
    // RULE: Initialize this if it's the user's first deposit.
    #[account(
        init_if_needed, // Only create if it doesn't exist
        payer = signer, 
        space = 8 + UserPosition::LEN,
        seeds = [b"position", signer.key().as_ref(), market.key().as_ref()],
        bump,
    )]
    pub user_position: Account<'info, UserPosition>,

    // 5. The PDA Authority (The program's owner key for the vault)
    #[account(
        seeds = [b"market_vault", market.key().as_ref()], 
        bump,
    )]
    pub collateral_vault_authority: AccountInfo<'info>, 

    // 6. The Protocol's Collateral Vault (The storage)
    // RULE: Initialize this if it's the protocol's first deposit.
    #[account(
        init_if_needed, // Only create if it doesn't exist
        payer = signer,
        token::mint = collateral_mint,
        token::authority = collateral_vault_authority,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    // 7. Mints and Programs
    pub collateral_mint: Account<'info, Mint>, 
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}