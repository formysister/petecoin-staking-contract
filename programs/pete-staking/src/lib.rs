use solana_program::pubkey::*;
use solana_program::clock::{
    Clock, UnixTimestamp
};
use borsh:: {
    BorshDeserialize,
    BorshSerialize
};
use std::time:: {
    Duration
};
use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::token;
use anchor_spl::{
    token::{ MintTo, Token, Transfer }
};
declare_id!("CTg35G6Cin3iQZHe8i5pN9rJ5ajSyCN2sjvDmVfCyVpi");

#[program]
pub mod pete_staking {
    use super::*;

    // pub static mut packages: Vec<Package> = Vec::new();

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let petes_pebble_pounch = Package {
            name: String::from("Pete's Pebble Pounch"),
            deposit_amount: 500000,
            reward_amount: 539583,
            period: 60 * 60 * 24 * 30,
            limit: 80
        };

        let golden_wheel_guild = Package {
            name: String::from("Golden Wheel Guild"),
            deposit_amount: 1000000,
            reward_amount: 1108333,
            period: 60 * 60 * 24 * 30 * 2,
            limit: 70
        };

        let burrowers_bounty = Package {
            name: String::from("Burrower's Bounty"),
            deposit_amount: 2000000,
            reward_amount: 2250000,
            period: 60 * 60 * 24 * 30 * 3,
            limit: 60
        };

        let cheek_pounch_chest = Package {
            name: String::from("Cheek Pounch Chest"),
            deposit_amount: 3000000,
            reward_amount: 3525000,
            period: 60 * 60 * 24 * 30 * 6,
            limit: 50
        };

        let hamster_haven_hoard = Package {
            name: String::from("Hamster Haven Hoard"),
            deposit_amount: 4000000,
            reward_amount: 4750000,
            period: 60 * 60 * 24 * 30 * 9,
            limit: 40
        };

        let oxonis_wizard = Package {
            name: String::from("Oxonis Wizard"),
            deposit_amount: 5000000,
            reward_amount: 6000000,
            period: 60 * 60 * 24 * 30 * 12,
            limit: 30
        };
        
        let staking_storage = &mut ctx.accounts.staking_storage;
        staking_storage.packages.push(petes_pebble_pounch);
        staking_storage.packages.push(golden_wheel_guild);
        staking_storage.packages.push(burrowers_bounty);
        staking_storage.packages.push(cheek_pounch_chest);
        staking_storage.packages.push(hamster_haven_hoard);
        staking_storage.packages.push(oxonis_wizard);

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

    // pub fn set(ctx: Context<Set>, new_x: u64) -> Result<()> {
    //     let my_storage = &mut ctx.accounts.my_storage;
	//     my_storage.x = new_x;

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space=size_of::<StakingStorage>() + 8,
        seeds = [],
        bump)]
    pub staking_storage: Account<'info, StakingStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Package {
    pub name: String,
    pub deposit_amount: u64,
    pub period: u64,
    pub reward_amount: u64,
    pub limit: u64
}

#[account]
pub struct StakingStorage{
    packages: Vec<Package>
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, StakingStorage>,
}