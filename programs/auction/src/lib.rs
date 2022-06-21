use anchor_lang::prelude::*;
use solana_program::{
    system_program,
    system_instruction,
    program::invoke,
};

declare_id!("G1cr8ubixaosck66PGR9wyyZbTsSzTnxRVhV4FYn5AqX");

const AUCTION_SIGNER_SEEDS: &str = "yaxche";
const BID_SIGNER_SEEDS: &str = "bidtesthuehue";
const LAMPORTS_PER_SOL:u64 = 1000000000;

#[program]
pub mod auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialze>, mut price:i64, _bump:u8, royalty:u8) -> Result<()> {


        if price < 0 { &mut price = |price| } 

        msg!("Welcome to initialize function");

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;

        msg!("auctionAccount PDA loaded successfully");

        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price as u64;
        auction_account.mint = *ctx.accounts.mint.key;
        auction_account.is_on_sale = true;
        auction_account.royalty_owner = *ctx.accounts.seller.key;
        auction_account.royalty_percent = royalty;
        auction_account.primary_sale_happened = false;

        msg!("Cost of NFT is {}", auction_account.cost);

        msg!("Auction settings set, preparing to launch invoke");

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

        msg!("Invoke done, initialize function ab samapt hua");

        Ok(())
    }

    pub fn create_auction(ctx: Context<CreateAuction>, price:u64, _bump:u8) -> Result<()> {

        msg!("Welcome to create auction function");

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;

        msg!("AuctionAccound pda loaded");

        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price;
        auction_account.is_on_sale = true;

        msg!("Auction settings set, preparing to launch invoke");

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
        
        msg!("Invoke done, create_auction function ab samapt hua");

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _bump:u8) -> Result<()> {

        msg!("Buy NFT called");

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        auction_account.buyer = *ctx.accounts.buyer.key;
        let sol_amount = auction_account.cost * LAMPORTS_PER_SOL;

        msg!("Transferring {} units", sol_amount);
        msg!("Cost of NFT is {}", auction_account.cost);

        if ctx.accounts.seller.key() == auction_account.seller {
            invoke(
                &system_instruction::transfer(ctx.accounts.buyer.key, ctx.accounts.vault.key, sol_amount),
                &[ctx.accounts.buyer.clone(), ctx.accounts.vault.clone()],
            )?;
        } 
            
        msg!("Sol recieved in Vault, Transferring NFT to the buyer");
        
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.from_token_account.key,
            ctx.accounts.to_token_account.key,
            ctx.accounts.vault.key,
            &[ctx.accounts.vault.key],
            1,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.from_token_account.clone(),
                ctx.accounts.to_token_account.clone(),
                ctx.accounts.vault.clone(),
                ctx.accounts.token_program.clone(),
            ],
        )?;

        if auction_account.primary_sale_happened == false {
            
            msg!("Primary sale not happened");

            invoke(
                &system_instruction::transfer(ctx.accounts.vault.key, &auction_account.seller, sol_amount),
                &[ctx.accounts.vault.clone(), ctx.accounts.seller.clone()],
            )?;

        } else {

            if auction_account.royalty_percent == 0 {

                msg!("Primary sale happened but royalty percent is 0");

                invoke(
                    &system_instruction::transfer(ctx.accounts.vault.key, &auction_account.seller, sol_amount),
                    &[ctx.accounts.vault.clone(), ctx.accounts.seller.clone()],
                )?;

            } else {

                msg!("Primary same happened and royalty percent non 0");

                let send_sol_to_creator = (auction_account.cost * auction_account.royalty_percent as u64) * LAMPORTS_PER_SOL/100;
                invoke(
                    &system_instruction::transfer(ctx.accounts.vault.key, &auction_account.royalty_owner, send_sol_to_creator),
                    &[ctx.accounts.vault.clone(), ctx.accounts.creator.clone()],
                )?;
                
                msg!("One sol sent, sending two sol");

                let send_sol_to_seller = sol_amount - send_sol_to_creator;
                invoke(
                    &system_instruction::transfer(ctx.accounts.vault.key, &auction_account.seller, send_sol_to_seller),
                    &[ctx.accounts.vault.clone(), ctx.accounts.seller.clone()],
                )?;
            }
        }

        auction_account.is_on_sale = false;
        auction_account.cost = 0;
        auction_account.highest_bid = 0;
        auction_account.primary_sale_happened = true;

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

        let bid_to_lamports = bid * LAMPORTS_PER_SOL;
        invoke(
            &system_instruction::transfer(ctx.accounts.bidder.key, ctx.accounts.vault.key, bid_to_lamports),
            &[ctx.accounts.bidder.clone(), ctx.accounts.vault.clone()],
        )?;

        let bids = &mut ctx.accounts.bid_account.bid_info;
        bids.push(BidderData{bidder:ctx.accounts.bidder.key(), bid:bid, is_cancelled:false});

        msg!("The line after invoke, please check funds. fn bid samapt hua");
        
        Ok(())
    }

    pub fn end_english_auction(ctx: Context<EndEnglishAuction>, _bump:u8) -> Result<()> {

        msg!("Welcome to end english auction function");
        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        let final_bid_to_lamports = auction_account.highest_bid * LAMPORTS_PER_SOL;
        msg!("final bid to lamports done {}", final_bid_to_lamports);

        msg!("Checking condition and preparing to launch invoke to transfer sol");
        if ctx.accounts.seller.key() == auction_account.seller {
            msg!("inside if while transferring sol");
        invoke(
            &system_instruction::transfer(ctx.accounts.vault.key, &ctx.accounts.auction_account.seller, final_bid_to_lamports),
            &[ctx.accounts.vault.clone(), ctx.accounts.seller.clone()],
        )?;
        }

        msg!("Sol sent, maybe check your balance");
        msg!("Checking conditions and preparing to launch invoke to transfer NFT");

        if ctx.accounts.to_account.key() == ctx.accounts.auction_account.highest_bidder {
            msg!("inside if while transferring NFT");
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.from_token_account.key,
            ctx.accounts.to_token_account.key,
            ctx.accounts.vault.key,
            &[ctx.accounts.vault.key],
            1,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.from_token_account.clone(),
                ctx.accounts.to_token_account.clone(),
                ctx.accounts.vault.clone(),
                ctx.accounts.token_program.clone(),
            ],
        )?;
    }

    msg!("NFT transfer done, kindly check you wallet");
    msg!("End English Auction function ab samapt hua");

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
    #[account(
        init, 
        payer = seller, 
        space = 1000,
        seeds = [
            "bid".as_bytes(),
            auction_account.key().as_ref(),
            program_id.as_ref(),
            BID_SIGNER_SEEDS.as_bytes(),
            ], 
        bump)]
    bid_account: Account<'info, BidVecAccount>,
    #[account(mut, signer)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    mint: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
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
    #[account(mut,signer)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
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
    #[account(mut, signer)]
    /// CHECK XYZ
    buyer: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    seller:AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    creator:AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK XYZ
    vault: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
    
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
    #[account(
        mut,
        seeds = [
            "bid".as_bytes(),
            auction_account.key().as_ref(),
            program_id.as_ref(),
            BID_SIGNER_SEEDS.as_bytes(),
        ],
        bump,
        )]
        bid_account: Account<'info, BidVecAccount>,
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut,signer)]
    /// CHECK XYZ
    bidder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    vault: AccountInfo<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct EndEnglishAuction<'info> {
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
    /// CHECK xyy
    pub to_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    #[account(signer,mut)]
    /// CHECK XYZ
    vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZxx 
    seller: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
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
    royalty_owner: Pubkey, // 32 change name to creator
    primary_sale_happened: bool, //1
    highest_bid: u64, // 8
    highest_bidder: Pubkey, // 32
}

#[account]
pub struct BidVecAccount {
    bid_info: Vec<BidderData>,
}

#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BidderData {
    bidder: Pubkey,
    bid: u64,
    is_cancelled: bool,
}
