// 2. BORROW

//🎯 Goal: User receives $\text{USDC}$ as a loan. Crucially: The system checks if $\text{LTV}$ is safe (Placeholder Safety Rule) before releasing funds. The Market PDA signs the transfer.
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};


fn borrow_asset(ctx: Context<BorrowAsset>, amount:u64)->Result<()>{

// 2. Prepare the PDA signer seeds
// HINT: Use the &[...] array definition to create the list of seeds

let signer_seeds: &[&[u8]] = &[

    // 1. The fixed string seed is missing here
    b"vault_authority",
    // 2. The Market Key
    ctx.accounts.market.key().as_ref(),
    // 3. The Bump
    &[ctx.bumps.vault_authority],
];
// 3. Prepare the CPI context for the token transfer
// Hint: this uses the new_with_signer context
anchor_spl::token::transfer(
    CpiContext::new_with_signer(
        //program: the token program account info
        ctx.accounts.token_program.to_account_info(),
        // 2. CPI Accounts: The transfer accounts (from, to, authority)
        Transfer{
            from:ctx.accounts.asset_vault.to_account_info(), // HINT 1: Protocol's asset vault
            to:ctx.accounts.user_asset_token_account.to_account_info(), // HINT 2: User's token account
            authority:ctx.accounts.vault_authority.to_account_info(), // HINT 3: The PDA authority

        },&[signer_seeds],

    ),
    amount
)?;
ctx.accounts.user_position.borrowed_asset+=amount;

ctx.accounts.market.total_asset_borrowed += amount;


    Ok(())
}
