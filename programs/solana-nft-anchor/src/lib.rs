use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    pda::{find_master_edition_account, find_metadata_account},
    state::DataV2,
};

// The program ID should be unique and not similar to the one provided here.
declare_id!("9TEtkW972r8AVyRmQzgyMz8GpG7WJxJ2ZUVZnjFNJgWM");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    // Initializes an NFT by creating a mint account, metadata account, and master edition account.
    pub fn init_nft(
        ctx: Context<InitNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // Create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        // Mint a single token (NFT) to the associated token account
        mint_to(cpi_context, 1)?;

        // Create metadata account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        // Define the metadata for the NFT
        let data_v2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        // Create the metadata accounts for the NFT using the provided metadata
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        // Create master edition account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        // Create the master edition for the NFT, indicating it's a unique token
        create_master_edition_v3(cpi_context, None)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: This account is explicitly passed in and marked as mutable and a signer.
    /// It is the authority and fee payer for the transactions and must sign the transaction.
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,

    /// The mint account for the NFT, which contains details such as mint authority,
    /// freeze authority, and total supply. It is initialized here with the signer as the authority,
    /// no decimals (since NFTs are not divisible), and the signer is also set as the freeze authority.
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info, Mint>,

    /// The associated token account for the NFT, which will hold the minted token.
    /// It is initialized if needed, with the signer as the payer and authority, and linked to the mint.
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: The metadata account address is derived and checked to ensure correctness.
    /// It is associated with the mint account and will store the NFT's metadata.
    #[account(
        mut,
        address = find_metadata_account(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: The master edition account address is derived and checked to ensure correctness.
    /// It proves the non-fungibility of the token and is used for setting up the master edition NFT.
    #[account(
        mut,
        address = find_master_edition_account(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,

    /// The Token program account, which is responsible for handling operations related to SPL tokens.
    pub token_program: Program<'info, Token>,

    /// The Associated Token program account, which handles the creation of associated token accounts.
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// The Token Metadata program account, which is used for creating and managing metadata for SPL tokens.
    pub token_metadata_program: Program<'info, Metadata>,

    /// The System program account, which is responsible for creating and managing accounts on Solana.
    pub system_program: Program<'info, System>,

    /// The Rent sysvar account, which provides information about the rent exemption costs on Solana.
    /// All accounts must be rent-exempt by depositing a certain amount of SOL.
    pub rent: Sysvar<'info, Rent>,
}

