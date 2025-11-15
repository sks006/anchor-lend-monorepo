use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Transfer, TokenAccount, Token };

fn borrow_asset(ctx: Context<BorrowAsset>, amount: u64) -> Result<()> {

    // 1. ⚠️ RISK MANAGEMENT RULE: Check LTV before executing any change.
    
    // 1a. Calculate the NEW total debt after borrowing (in raw tokens).
    // RULE: Use checked_add for safe arithmetic.
    let new_borrowing_deposit: u64 = ctx.accounts.user_position.borrowed_asset
        .checked_add(amount)
        .unwrap();

    // 1b. Calculate the MAX allowed debt (Max collateral * Factor).
    // RULE: Use u128 for intermediate math to prevent overflow and preserve precision.
    let max_debt_allowed: u128 = (ctx.accounts.user_position.deposited_collateral as u128)
        .checked_mul(ctx.accounts.market.collateral_factor as u128)
        .unwrap()
        // CORRECTION: The scaling factor must match the precision (e.g., 10^8).
        // The original code had 80,000,000 which is likely incorrect for descaling.
        .checked_div(100_000_000) 
        .unwrap();
    
    // 1c. Check the LTV rule.
    // RULE: If new debt exceeds the allowed max, halt the transaction.
    if (new_borrowing_deposit as u128) > max_debt_allowed {
        return Err(ErrorCode::LTVExceeded.into());
    }
    
    // --- LTV CHECK PASSED: Funds can be transferred ---

    // 2. 🖋️ PDA SIGNING RULE: Define the seeds for the PDA (vault_authority) to sign the transfer.
    let signer_seeds: &[&[u8]] = &[
        b"vault_authority", // Fixed string seed (must match PDA derivation)
        ctx.accounts.market.key().as_ref(), // Unique market key seed
        &[ctx.bumps.vault_authority], // The bump byte
    ];
    
    // 3. 🚀 EXECUTION RULE: Transfer assets from the protocol's vault to the user.
    // HINT: CpiContext::new_with_signer is used because the PDA is the authority.
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), // The Token Program is called.
            Transfer {
                from: ctx.accounts.asset_vault.to_account_info(), // SOURCE: Protocol's Asset Vault
                to: ctx.accounts.user_asset_token_account.to_account_info(), // DESTINATION: User's Token Account
                authority: ctx.accounts.vault_authority.to_account_info(), // SIGNER: The PDA
            },
            &[signer_seeds] // Provides the programmatic signature
        ),
        amount // The amount to borrow
    )?;
    
    // 4. 📝 LEDGER RULE: Update the state to reflect the new debt.
    // RULE: Both user's debt and the market's total debt must increase.
    ctx.accounts.user_position.borrowed_asset = new_borrowing_deposit; 
    ctx.accounts.market.total_asset_borrowed += amount;

    // 5. SUCCESS
    Ok(())
}