use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::{accounts::Metadata, types::Creator};
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;

pub mod account;
pub mod constants;
pub mod error;

use account::*;
use constants::*;
use error::*;

declare_id!("5J3fJvN67uWLo2uNaygTJjdRoJs5mxn9XgtXroiQkcwm");

#[program]
pub mod mugs_marketplace {

 
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
        space = 8 + 368,
        payer = admin
    )]
    pub global_authority: Account<'info, GlobalPool>,
    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct SetTreshold<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AddTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct RemoveTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitUserPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + 48,
        payer = owner,
    )]
    pub user_pool: Account<'info, UserData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserData>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserData>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nft: Pubkey, bump: u8)]
pub struct InitSellData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [SELL_DATA_SEED.as_ref(), nft.to_bytes().as_ref()],
        bump,
        space = 8 + 120,
        payer = payer,
    )]
    pub sell_data_info: Account<'info, SellData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct ListPNftForSale<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,

    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key(),
        constraint = user_token_account.owner == *owner.key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    // #[account(
    //     init_if_needed,
    //     associated_token::mint = nft_mint,
    //     associated_token::authority = global_authority,
    //     payer = owner,
    // )]
    // pub dest_nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: legacy pre delegates will be removed
    pub dest_nft_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
    /// the mint metadata
    #[account(
        mut,
        constraint = mint_metadata.owner == &mpl_token_metadata::ID
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Account<'info, AuctionData>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct DelistPNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,

    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key(),
        constraint = user_token_account.owner == *owner.key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    // #[account(
    //     mut,
    //     constraint = dest_nft_token_account.mint == nft_mint.key(),
    //     constraint = dest_nft_token_account.owner == global_authority.key(),
    //     constraint = dest_nft_token_account.amount == 1,
    // )]
    // pub dest_nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: legacy pre delegates will be removed
    pub dest_nft_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
    /// the mint metadata
    #[account(
            mut,
            constraint = mint_metadata.owner == &mpl_token_metadata::ID
        )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Box<Account<'info, AuctionData>>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct SetPrice<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct PurchasePNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Box<Account<'info, SellData>>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), buyer.key().as_ref()],
        bump,
    )]
    pub buyer_user_pool: Account<'info, UserData>,

    // #[account(
    //     mut,
    //     constraint = auction_data_info.creator==creator.key(),
    // )]
    // /// CHECK: This is not dangerous because we don't read or write from this account
    // pub creator: AccountInfo<'info>,
    #[account(
        mut,
        constraint = user_nft_token_account.mint == nft_mint.key(),
        constraint = user_nft_token_account.owner == *buyer.key,
    )]
    pub user_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = dest_nft_token_account.mint == nft_mint.key(),
        constraint = dest_nft_token_account.owner == sell_data_info.seller.key(),
        constraint = dest_nft_token_account.amount == 1,
    )]
    pub dest_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), seller.key().as_ref()],
        bump,
    )]
    pub seller_user_pool: Account<'info, UserData>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
    /// the mint metadata
    #[account(
        mut,
        constraint = mint_metadata.owner == &mpl_token_metadata::ID
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Box<Account<'info, AuctionData>>,
}

#[derive(Accounts)]
#[instruction(nft: Pubkey, bump: u8)]
pub struct InitOfferData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [OFFER_DATA_SEED.as_ref(), nft.to_bytes().as_ref(), payer.key().to_bytes().as_ref()],
        bump,
        space = 8 + 88,
        payer = payer,
    )]
    pub offer_data_info: Account<'info, OfferData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,

    #[account(
        mut,
        seeds = [OFFER_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref(), owner.key().to_bytes().as_ref()],
        bump,
    )]
    pub offer_data_info: Account<'info, OfferData>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_pool: Account<'info, UserData>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [OFFER_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref(), owner.key().to_bytes().as_ref()],
        bump,
    )]
    pub offer_data_info: Account<'info, OfferData>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_pool: Box<Account<'info, UserData>>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CancelBid<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref(), owner.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Account<'info, AuctionData>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub user_pool: Box<Account<'info, UserData>>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AcceptOfferPNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Box<Account<'info, SellData>>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [OFFER_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref(), buyer.key().to_bytes().as_ref()],
        bump,
    )]
    pub offer_data_info: Box<Account<'info, OfferData>>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), seller.key().as_ref()],
        bump,
    )]
    pub seller_user_pool: Box<Account<'info, UserData>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), buyer.key().as_ref()],
        bump,
    )]
    pub buyer_user_pool: Box<Account<'info, UserData>>,

    #[account(
        mut,
        constraint = user_nft_token_account.mint == nft_mint.key(),
        constraint = user_nft_token_account.owner == *buyer.key,
    )]
    pub user_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = dest_nft_token_account.mint == nft_mint.key(),
        constraint = dest_nft_token_account.owner == seller.key(),
        constraint = dest_nft_token_account.amount == 1,
    )]
    pub dest_nft_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,

    /// the mint metadata
    #[account(
        mut,
        constraint = mint_metadata.owner == &mpl_token_metadata::ID
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Box<Account<'info, AuctionData>>,
}

#[derive(Accounts)]
#[instruction(nft: Pubkey, bump: u8)]
pub struct InitAuctionData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft.to_bytes().as_ref()],
        bump,
        space = 8 + 152,
        payer = payer,
    )]
    pub auction_data_info: Account<'info, AuctionData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateAuctionPNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Account<'info, AuctionData>,

    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key(),
        constraint = user_token_account.owner == *owner.key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    // #[account(
    //     mut,
    //     constraint = dest_nft_token_account.mint == nft_mint.key(),
    //     constraint = dest_nft_token_account.owner == global_authority.key(),
    // )]
    // pub dest_nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: legacy pre delegates will be removed
    pub dest_nft_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
    /// the mint metadata
    #[account(
    mut,
    constraint = mint_metadata.owner == &mpl_token_metadata::ID
)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Account<'info, AuctionData>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub out_bidder: SystemAccount<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Box<Account<'info, SellData>>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct ClaimAuctionPNft<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Account<'info, AuctionData>,

    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key(),
        constraint = user_token_account.owner == *bidder.key,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = dest_nft_token_account.mint == nft_mint.key(),
        constraint = dest_nft_token_account.owner == auction_data_info.creator.key(),
        constraint = dest_nft_token_account.amount == 1,
    )]
    pub dest_nft_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = auction_data_info.creator==creator.key(),
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub creator: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [ESCROW_VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub escrow_vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), bidder.key().as_ref()],
        constraint = bidder_user_pool.address == bidder.key(),
        bump,
    )]
    pub bidder_user_pool: Box<Account<'info, UserData>>,

    #[account(
        mut,
        seeds = [USER_DATA_SEED.as_ref(), creator.key().as_ref()],
        bump,
    )]
    pub creator_user_pool: Box<Account<'info, UserData>>,

    /// the mint metadata
    #[account(
        mut,
        constraint = mint_metadata.owner == &mpl_token_metadata::ID
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct UpdateReserve<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Box<Account<'info, AuctionData>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CancelAuctionPNft<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    #[account(
        mut,
        seeds = [AUCTION_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub auction_data_info: Box<Account<'info, AuctionData>>,

    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key(),
        constraint = user_token_account.owner == *creator.key,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     mut,
    //     constraint = dest_nft_token_account.mint == nft_mint.key(),
    //     constraint = dest_nft_token_account.owner == global_authority.key(),
    //     constraint = dest_nft_token_account.amount == 1,
    // )]
    // pub dest_nft_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: legacy pre delegates will be removed
    pub dest_nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub nft_mint: AccountInfo<'info>,
    /// the mint metadata
    #[account(
            mut,
            constraint = mint_metadata.owner == &mpl_token_metadata::ID
        )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint_metadata: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,

    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong record is supplied
    #[account(mut)]
    pub dest_token_mint_record: AccountInfo<'info>,

    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,

    /// CHECK: this account is safe
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = token_metadata_program.key == &mpl_token_metadata::ID)]
    pub token_metadata_program: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SELL_DATA_SEED.as_ref(), nft_mint.key().to_bytes().as_ref()],
        bump,
    )]
    pub sell_data_info: Account<'info, SellData>,
}
