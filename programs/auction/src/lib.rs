use anchor_lang::prelude::*;
use solana_program::{
    system_program,
    program::invoke,
    system_instruction,
};

declare_id!("58h5LsVGRck3x8ksGK9BB9ke1zKrGMFgcqg12fRMGTe6");

const AUCTION_SIGNER_SEEDS: &str = "testhuehuehuetest";

#[program]
pub mod auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialze>, price:u64, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        msg!("Pda account created");

       // auction_account.seller = ctx.accounts.seller.key.to_string();
        auction_account.seller = *ctx.accounts.seller.key;

        auction_account.cost = price;
        auction_account.mint = *ctx.accounts.mint.key;
        auction_account.is_on_sale = true;

        msg!("Pda account initialized");
        msg!("Transfer instruction made.");
        msg!("Transfer done");
        // msg!("Pda account.cost {}", auction_account.cost);

        Ok(())
    }

    pub fn create_auction(ctx: Context<CreateAuction>, price:u64, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        
        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price;
        auction_account.mint = *ctx.accounts.mint.key;
        auction_account.is_on_sale = true;
        
        msg!("Pda account fetch on create auction");
        msg!("PDA {:?}", auction_account.seller);

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        
        auction_account.buyer = *ctx.accounts.buyer.key;
        
        msg!("Pda account fetch");
        // msg!("PDA {}", auction_account.cost);

        Ok(())
    }

    pub fn transfer_lamports(ctx: Context<TransferSOL>) -> Result<()> {
        
        let amount = 10000000;

        invoke(
            &system_instruction::transfer(ctx.accounts.from_account.key, ctx.accounts.to_account.key, amount),
            &[ctx.accounts.from_account.clone(), ctx.accounts.to_account.clone()],
        )?;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct Initialze<'info> {
    #[account(
        init, 
        payer = seller, 
        space = 200,
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
    // #[account(mut, constraint = mint.key() == auction_account.mint.key())]
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    buyer: AccountInfo<'info>,
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
    // #[account(mut, constraint = mint.key() == auction_account.mint.key())]
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    buyer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferSOL<'info> {
    #[account(mut)]
    /// CHECK XYZ
    pub from_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    pub to_account: AccountInfo<'info>,
    /// CHECK XYZ
    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct AuctionManager {
    seller: Pubkey,
    mint: Pubkey,
    cost: u64,
    buyer: Pubkey,
    is_on_sale: bool,
}
