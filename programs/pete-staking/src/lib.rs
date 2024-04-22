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
    use std::time;

    use solana_program::clock;

    use super::*;

    // pub static mut packages: Vec<Package> = Vec::new();

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let petes_pebble_pounch = Package {
            name: String::from("Pete's Pebble Pounch"),
            deposit_amount: 500000,
            reward_amount: 539583,
            period: 60 * 60 * 24 * 30,
            slot_limit: 80,
            slot_count: 0
        };

        let golden_wheel_guild = Package {
            name: String::from("Golden Wheel Guild"),
            deposit_amount: 1000000,
            reward_amount: 1108333,
            period: 60 * 60 * 24 * 30 * 2,
            slot_limit: 70,
            slot_count: 0
        };

        let burrowers_bounty = Package {
            name: String::from("Burrower's Bounty"),
            deposit_amount: 2000000,
            reward_amount: 2250000,
            period: 60 * 60 * 24 * 30 * 3,
            slot_limit: 60,
            slot_count: 0
        };

        let cheek_pounch_chest = Package {
            name: String::from("Cheek Pounch Chest"),
            deposit_amount: 3000000,
            reward_amount: 3525000,
            period: 60 * 60 * 24 * 30 * 6,
            slot_limit: 50,
            slot_count: 0
        };

        let hamster_haven_hoard = Package {
            name: String::from("Hamster Haven Hoard"),
            deposit_amount: 4000000,
            reward_amount: 4750000,
            period: 60 * 60 * 24 * 30 * 9,
            slot_limit: 40,
            slot_count: 0
        };

        let oxonis_wizard = Package {
            name: String::from("Oxonis Wizard"),
            deposit_amount: 5000000,
            reward_amount: 6000000,
            period: 60 * 60 * 24 * 30 * 12,
            slot_limit: 30,
            slot_count: 0
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

    pub fn stake(ctx: Context<Deposit>, package_index: u8) -> Result<()> {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.autority.to_account_info(),
        };

        
        // validate package index
        let packages = & ctx.accounts.staking_storage.packages;

        if package_index >= packages.len() as u8{
            return Err(ErrorCode::InvalidPackageIndex.into());
        }

        // check if user already have stake on same package
        let stake_logs = & ctx.accounts.staking_storage.stake_logs;
        for stake_log in  stake_logs.iter() {
            if stake_log.staker == ctx.accounts.from.to_account_info().key() && package_index == stake_log.package_index && stake_log.terminated == false {
                return Err(ErrorCode::AccountAlreadyStaked.into())
            }
            else {
                continue;
            }
        }


        let deposit_amount = ctx.accounts.staking_storage.packages[package_index as usize].deposit_amount;

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, deposit_amount)?;

        let clock = Clock::get();
        let timestamp = clock.unwrap().unix_timestamp;

        let staking_storage = &mut ctx.accounts.staking_storage;
        let stake_log = StakeLog {
            staker: ctx.accounts.from.to_account_info().key(),
            package_index: package_index,
            stake_timestamp: timestamp,
            terminated: false
        };

        staking_storage.stake_logs.push(stake_log);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space=size_of::<StakingStorage>() + 800,
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

    pub clock: Sysvar<'info, Clock>,

    #[account(mut, seeds = [], bump)]
    pub staking_storage: Account<'info, StakingStorage>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Package {
    pub name: String,
    pub deposit_amount: u64,
    pub period: u64,
    pub reward_amount: u64,
    pub slot_limit: u8,
    pub slot_count: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakeLog {
    pub staker: Pubkey,
    pub package_index: u8,

    // #[account(address = solana_program::sysvar::clock::ID)]
    pub stake_timestamp: i64,
    pub terminated: bool
}

#[account]
pub struct StakingStorage {
    packages: Vec<Package>,
    stake_logs: Vec<StakeLog>
}

#[derive(Accounts)]
pub struct SetStakingStorage<'info> {
    #[account(mut, seeds = [], bump)]
    pub staking_storage: Account<'info, StakingStorage>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid package index. It must be 0 ~ 5")]
    InvalidPackageIndex,
    #[msg("Account already staked on same package")]
    AccountAlreadyStaked
}