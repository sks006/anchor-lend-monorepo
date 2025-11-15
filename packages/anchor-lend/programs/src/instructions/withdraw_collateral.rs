// 4. WITHDRAW

//🎯 Goal: User retrieves their collateral. Crucially: The system checks if the debt is zero (or safe). If safe, the Market PDA signs the collateral back to the user.
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    // 1. STATE & SIGNER
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub market: Account<'info, Market>,

    // 2. TOKEN FLOW (Source and Destination)
    #[account(mut)]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_collateral_token_account: Account<'info, TokenAccount>,

    // 3. PDA AUTHORITY (The signer)
    // HINT 2: Needs seeds and bump
    #[account(seeds = [b"market_vault", market.key().as_ref()], bump)]
    pub collateral_vault_authority: AccountInfo<'info>,

    // 4. PROGRAMS
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
    // Check 1: If user has no debt (borrowed_asset == 0), proceed directly to withdrawal.
 let signer_seeds: &[&[u8]] = &[
    b"market_vault", 
    ctx.accounts.market.key().as_ref(),
    &[ctx.bumps.collateral_vault_authority], 
];

// Start the conditional check
if ctx.accounts.user_position.borrowed_asset == 0 {
    
    // 1. EXECUTE CPI TRANSFER (PDA Signs)
    anchor_spl::token::transfer(
        CpiContext::new_with_signer( // <--- Corrected syntax
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.collateral_vault.to_account_info(),
                to: ctx.accounts.user_collateral_token_account.to_account_info(),
                authority: ctx.accounts.collateral_vault_authority.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?; // <-- Correctly end with '?'
    
    // 2. LEDGER RULE: Update the state to reflect the withdrawal.
    // RULE: Withdrawing collateral DECREASES both balances.
    ctx.accounts.user_position.deposited_collateral -= amount;
    ctx.accounts.market.total_collateral_deposited -= amount;
    // 2. LEDGER RULE: Update the state to reflect the withdrawal.
    // RULE: Withdrawing collateral DECREASES both balances.
    ctx.accounts.user_position.deposited_collateral -= amount;
    ctx.accounts.market.total_collateral_deposited -= amount;
    
    // We are done with the successful withdrawal
    return Ok(()); // <-- Exit successfully
}

Ok(())
}
