use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("Cdh53b8Xg3GwMLMaNvwQBEcU9Ncu4uCiCh4UUUZchcsP");

#[program]
pub mod solana_nft_anchor {
    use anchor_spl::token::mint_to;

    use super::*;

    pub fn init_nft(ctx: Context<InitNFT>) -> Result<()> {
        // create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );
        mint_to(cpi_context, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
    signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    mint: Account<'info, Mint>, //new
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>, //new

    pub token_program: Program<'info, Token>, //new
    pub associated_token_program: Program<'info, AssociatedToken>, //new
    pub system_program: Program<'info, System>, //new
    pub rent: Sysvar<'info, Rent> //new
}
