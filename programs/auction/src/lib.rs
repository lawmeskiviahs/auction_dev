use anchor_lang::prelude::*;
use solana_program::{
    program::invoke,
    system_instruction,
    system_program,
};

declare_id!("61reie38A5ecZQ45ebeeCcQgBQ82NtA7h59jPLCzx6mK");

const AUCTION_SIGNER_SEEDS: &str = "testhuehuehuetest";
const OUR_WALLET: &str = "";

#[program]
pub mod auction {
    use super::*;

    pub fn create_auction(ctx: Context<CreateAuction>, price:u64, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;
        msg!("Pda account created");

        auction_account.seller = *ctx.accounts.seller.key;
        auction_account.cost = price;
        auction_account.mint = *ctx.accounts.mint.key;

        msg!("Pda account initialized");
        msg!("Transfer instruction made.");
        msg!("Transfer done");
        msg!("Pda account.cost {}", auction_account.cost);

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNFT>, _bump:u8) -> Result<()> {

        let auction_account: &mut Account<AuctionManager> = &mut ctx.accounts.auction_account;

        invoke(
            &system_instruction::transfer(ctx.accounts.buyer.key, ctx.accounts.vault.key, auction_account.cost),
            &[ctx.accounts.buyer.clone(), ctx.accounts.vault.clone()],
        )?;

        
        auction_account.buyer = ctx.accounts.buyer.key();

        let amount = 1;

            let ix = spl_token::instruction::transfer(
                ctx.accounts.token_program.key,
                ctx.accounts.vault_token_account.key,
                ctx.accounts.vault_token_account.key,
                ctx.accounts.vault.key,
                &[ctx.accounts.vault.key],
                amount,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.vault_token_account.clone(),
                    ctx.accounts.buyer_token_account.clone(),
                    ctx.accounts.vault.clone(),
                    ctx.accounts.token_program.clone(),
                ],
            )?;

            invoke(
                &system_instruction::transfer(ctx.accounts.vault.key, ctx.accounts.seller.key, ctx.accounts.auction_account.cost),
                &[ctx.accounts.vault.clone(), ctx.accounts.seller.clone()],
            )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct CreateAuction<'info> {
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
    // #[account(mut)]
    // /// CHECK XYZ Signer<'info>,
    // seller_token_account: AccountInfo<'info>,
    // #[account(mut)]
    // /// CHECK XYZ
    // vault_token_account: AccountInfo<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
    // /// CHECK XYZ
    // token_program: AccountInfo<'info>,
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
    #[account(mut, constraint = mint.key() == auction_account.mint.key())]
    /// CHECK checked in program
    mint:AccountInfo<'info>,
    #[account(mut, constraint = seller.key() == auction_account.seller.key())]
    /// CHECK XYZ
    seller: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK XYZ
    buyer: AccountInfo<'info>,
    /// CHECK XYZ
    buyer_token_account: AccountInfo<'info>,
    #[account(mut, constraint = vault.key() == OUR_WALLET.parse::< Pubkey > ().unwrap())]
    /// CHECK XYZ
    vault: AccountInfo<'info>,
    /// CHECK XYZ
    vault_token_account: AccountInfo<'info>,
    #[account(address = system_program::id())]
    /// CHECK XYZ
    system_program: AccountInfo<'info>,
    /// CHECK XYZ
    token_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct AuctionManager {
    seller: Pubkey,
    mint: Pubkey,
    cost: u64,
    buyer: Pubkey,
}
