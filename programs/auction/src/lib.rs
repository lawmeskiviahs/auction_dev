use anchor_lang::prelude::*;
use solana_program::{
    system_program,
    system_instruction,
    program::invoke,
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
        
        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.buyer = *ctx.accounts.buyer.key;


        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, bid:u64,) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;

        msg!("Welcome to bid function");

        if bid > auction_account.highest_bid {

            msg!("Bid value checked and it currently is {}", auction_account.highest_bid);
            auction_account.highest_bid=bid;
            msg!("auction_account.highest_bid set to {}", auction_account.highest_bid);
            auction_account.highest_bidder= ctx.accounts.bidder.key();
            msg!("auction_account.highest_bidder set");

        } 

        msg!("Preparing to launch invoke");

        invoke(
            &system_instruction::transfer(ctx.accounts.bidder.key, ctx.accounts.vault.key, bid),
            &[ctx.accounts.bidder.clone(), ctx.accounts.vault.clone()],
        )?;

        msg!("The line after invoke, please check funds. fn bid samapt hua");
        
        Ok(())
    }
    
    pub fn end_auction(ctx: Context<EndAuction>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.is_on_sale = false;
        auction_account.cost = 0;
        // auction_account.max_bid = 0;
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
        space = 180,
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
    seeds = [
        "auction".as_bytes(),
        program_id.as_ref(),
        mint.key().as_ref(),
        AUCTION_SIGNER_SEEDS.as_bytes(),
    ],
    bump,
    )]
    auction_account: Account<'info, AuctionManager>,
    #[account(signer)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
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
pub struct Bid<'info> {
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
    #[account(mut,signer)]
    /// CHECK XYZ
    bidder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    vault: AccountInfo<'info>,
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
    seller: Pubkey, // 32
    mint: Pubkey, // 32
    cost: u64, // 8
    buyer: Pubkey, // 32
    is_on_sale: bool, // 1
    royalty_percent:u8, // 1
    royalty_owner: Pubkey, // 32
    primary_sale_happened: bool, //1
    highest_bid: u64, // 8
    highest_bidder: Pubkey, // 32
}
