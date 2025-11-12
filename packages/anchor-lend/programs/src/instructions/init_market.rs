// 0. SETUP
// 🎯 Goal: Establish the protocol's global rules (LTV threshold, APR) and create the on-chain token vaults (PDAs) that will hold all the collateral and borrowed assets

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InitMarket<'info> {
    // HINT 1: What type is used here? (We defined it in state.rs)
    
    #[account(init, payer = signer, space = 8 + Market::LEN)]
    pub market: Account<'info, Market /* HINT 1 */>,

    // HINT 2: What is the required constraint to allocate the space based on LEN?
    #[account(mut)]
    pub signer: Signer<'info>,

    // RULE: These reference the existing SPL Mints the protocol will use.
    pub collateral_mint: Account<'info, Mint>, 
    pub asset_mint: Account<'info, Mint>,

    // HINT 3: What is the Anchor struct for the System Program?
    pub system_program: Program<'info,System /* HINT 3 */>,
    
    // HINT 4: What is the Anchor SPL type for an existing Token Mint account?
    // We need two of these: collateral_mint and asset_mint.
}


pub fn init_market(ctx:Context<InitMarket>)-> Result<()> {
    // Get a mutable reference to the new Market account data
    let market=&mut ctx.accounts.market;
    // RULE: Set the authority of the market to the transaction signer.
    // The Pubkey of the Signer is now the sole config controller.
    market.authority= ctx.accounts.signer.key();
    // RULE: Record which tokens this market uses, pulling the Pubkeys from the input accounts.
    market.collateral_mint= ctx.accounts.collateral_mint.key();
    market.asset_mint= ctx.accounts.asset_mint.key();
    // RULE: The total balances start at zero by default (thanks to #[derive(Default)])
    // but you may want to set the configuration parameters (e.g., collateral_factor)
    // using the instruction arguments here, or by creating a separate instruction later.

    Ok(())
}