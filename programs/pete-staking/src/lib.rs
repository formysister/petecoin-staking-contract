use solana_program::pubkey::*;
use solana_program::clock::{
    Clock, UnixTimestamp
};
use std::time:: {
    Duration
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::{
    token::{ MintTo, Token, Transfer }
};
declare_id!("CTg35G6Cin3iQZHe8i5pN9rJ5ajSyCN2sjvDmVfCyVpi");

#[program]
pub mod pete_staking {
    use super::*;

    pub static mut packages: Vec<Package> = Vec::new();

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let petes_pebble_pounch = Package {
            name: "Pete's Pebble Pounch",
            deposit_amount: 500000,
            reward_amount: 539583,
            period: Duration::from_secs(60 * 60 * 24 * 30),
            limit: 80
        };

        let golden_wheel_guild = Package {
            name: "Golden Wheel Guild",
            deposit_amount: 1000000,
            reward_amount: 1108333,
            period: Duration::from_secs(60 * 60 * 24 * 30 * 2),
            limit: 70
        };

        let burrowers_bounty = Package {
            name: "Burrower's Bounty",
            deposit_amount: 2000000,
            reward_amount: 2250000,
            period: Duration::from_secs(60 * 60 * 24 * 30 * 3),
            limit: 60
        };

        let cheek_pounch_chest = Package {
            name: "Cheek Pounch Chest",
            deposit_amount: 3000000,
            reward_amount: 3525000,
            period: Duration::from_secs(60 * 60 * 24 * 30 * 6),
            limit: 50
        };

        let hamster_haven_hoard = Package {
            name: "Hamster Haven Hoard",
            deposit_amount: 4000000,
            reward_amount: 4750000,
            period: Duration::from_secs(60 * 60 * 24 * 30 * 9),
            limit: 40
        };

        let oxonis_wizard = Package {
            name: "Oxonis Wizard",
            deposit_amount: 5000000,
            reward_amount: 6000000,
            period: Duration::from_secs(60 * 60 * 24 * 30 * 12),
            limit: 30
        };

        packages.push(petes_pebble_pounch);

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, deposit_amount: u64) -> Result<()> {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.autority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, deposit_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub autority: Signer<'info>,
}

pub struct Package<'a> {
    pub name: &'a str,
    pub deposit_amount: u64,
    pub period: Duration,
    pub reward_amount: u64,
    pub limit: u64
}