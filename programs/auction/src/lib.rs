use anchor_lang::prelude::*;
use solana_program::{
    program::invoke,
    system_program,
    // system_instruction
};

declare_id!("2b1sK3RkQBPPdLhJ67N2M7XiBj8gEX7ysPeGxadBQBRp");

const AUCTION_SIGNER_SEEDS: &str = "yaxche";

#[program]
pub mod auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialze>, price:u64, _bump:u8, royalty:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price;
        auction_account.mint = *ctx.accounts.mint.key;
        auction_account.is_on_sale = true;
        auction_account.royalty_owner = *ctx.accounts.seller.key;
        auction_account.royalty_percent = royalty;
        auction_account.primary_sale_happened = false;

        Ok(())
    }

    pub fn create_auction(ctx: Context<CreateAuction>, price:u64, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price;
        auction_account.mint = *ctx.accounts.mint.key;
        auction_account.is_on_sale = true;

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.from_token_account.key,
            ctx.accounts.to_token_account.key,
            ctx.accounts.seller.key,
            &[ctx.accounts.seller.key],
            1,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.from_token_account.clone(),
                ctx.accounts.to_token_account.clone(),
                ctx.accounts.seller.clone(),
                ctx.accounts.token_program.clone(),
            ],
        )?;
        
        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.buyer = *ctx.accounts.buyer.key;


        Ok(())
    }
    
    pub fn end_auction(ctx: Context<EndAuction>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.is_on_sale = false;
        auction_account.primary_sale_happened = true;
        

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct Initialze<'info> {
    #[account(
        init, 
        payer = seller, 
        space = 300,
        seeds = [
            "auction".as_bytes(),
            program_id.as_ref(),
            mint.key().as_ref(),
            AUCTION_SIGNER_SEEDS.as_bytes(),
            ], 
        bump)]
    auction_account: Account<'info, AuctionManager>,
    #[account(mut, signer)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    mint: UncheckedAccount<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct CreateAuction<'info> {
    #[account(
    mut,
    seeds = [
        "auction".as_bytes(),
        program_id.as_ref(),
        mint.key().as_ref(),
        AUCTION_SIGNER_SEEDS.as_bytes(),
    ],
    bump,
    )]
    auction_account: Account<'info, AuctionManager>,
    #[account(mut, signer)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct BuyNFT<'info> {
    #[account(
    mut,
    seeds = [
        "auction".as_bytes(),
        program_id.as_ref(),
        mint.key().as_ref(),
        AUCTION_SIGNER_SEEDS.as_bytes(),
    ],
    bump,
    )]
    auction_account: Account<'info, AuctionManager>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    buyer: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct EndAuction<'info> {
    #[account(
    mut,
    seeds = [
        "auction".as_bytes(),
        program_id.as_ref(),
        mint.key().as_ref(),
        AUCTION_SIGNER_SEEDS.as_bytes(),
    ],
    bump,
    )]
    auction_account: Account<'info, AuctionManager>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct AuctionManager {
    seller: Pubkey,
    mint: Pubkey,
    cost: u64, // 64
    buyer: Pubkey,
    is_on_sale: bool, // 1
    royalty_percent:u8, // 8
    royalty_owner: Pubkey,
    primary_sale_happened: bool //1
}
