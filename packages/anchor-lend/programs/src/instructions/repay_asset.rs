// 3. CLOSE/REPAY

//🎯 Goal: User sends the borrowed $\text{USDC}$ + interest back to the protocol's vault. Their outstanding debt balance in the UserPosition PDA is reduced.use anchor_lang::prelude::*;

use anchor_spl::token::{ self, Transfer, TokenAccount, Token };

pub fn repay_asset(ctx: Context<RepayAsset>, amount: u64) -> Result<()> {
    
    // Define the CPI Accounts for the transfer (User -> Vault)
    // RULE: For a user-signed transfer, the user is the source (from) and the authority (signer).
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_asset_token_account.to_account_info(), // SOURCE: User's Token Account
        to: ctx.accounts.asset_vault.to_account_info(), // DESTINATION: Protocol's Asset Vault
        authority: ctx.accounts.signer.to_account_info(), // SIGNER: User's Wallet
    };

    // 1. EXECUTE CPI TRANSFER (Tokens move from User to Vault)
    // RULE: Execute the transfer and use the '?' operator to safely propagate errors (e.g., if user has insufficient funds).
    anchor_spl::token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        amount,
    )?;

    // 2. LEDGER RULE: Update the state to reflect the debt reduction.
    // RULE: A repayment DECREASES the outstanding debt.
    
    // Decrement the user's personal debt balance.
    ctx.accounts.user_position.borrowed_asset = ctx.accounts.user_position.borrowed_asset
        .checked_sub(amount)
        .unwrap(); // Safety check: use checked_sub to prevent underflow if the user overpays.
    
    // Decrement the global total debt held by the market.
    ctx.accounts.market.total_asset_borrowed = ctx.accounts.market.total_asset_borrowed
        .checked_sub(amount)
        .unwrap(); // Safety check: ensures global debt doesn't underflow.

    // 3. SUCCESS
    Ok(())
}