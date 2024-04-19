use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::{
    token::{ MintTo, Token, Transfer }
};

declare_id!("CmxrCG1EhjZcedGZxKcKQLamjTZprQvFbvugN9Si1yhn");

#[program]
pub mod pete_token {

    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        let cpi_accounts = MintTo{
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_context, 10000)?;

        Ok(())
    }

    pub fn transfer(ctx: Context<TransferToken>) -> Result<()> {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, 5)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub signer: Signer<'info>
}