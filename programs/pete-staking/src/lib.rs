use solana_program::pubkey::*;
use solana_program::clock::{
    Clock, UnixTimestamp
};
use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::{
    token::{ MintTo, Token, Transfer }
};
declare_id!("FnGJWC16sMUACBekWbyGBaq3H9jc46oDMQ221AvXy1ew");

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
            to: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        
        let staking_storage = &mut ctx.accounts.staking_storage;
        let packages = & staking_storage.packages;
        let package = & packages[package_index as usize];

        // validate package index
        if package_index >= packages.len() as u8{
            return Err(ErrorCode::InvalidPackageIndex.into());
        }

        // check if user already have stake on same package
        for stake_log in  staking_storage.stake_logs.iter() {
            if stake_log.staker == ctx.accounts.from.to_account_info().key() && package_index == stake_log.package_index && stake_log.terminated == false {
                return Err(ErrorCode::AccountAlreadyStaked.into());
            }
            else {
                continue;
            }
        }

        // check if package slot fulfilled

        if package.slot_count == package.slot_limit {
            return Err(ErrorCode::PackageSlotFulFilled.into());
        }

        //start main staking process - deposit token
        let deposit_amount = package.deposit_amount;

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, deposit_amount)?;

        // update stake log
        let clock = Clock::get();
        let timestamp = clock.unwrap().unix_timestamp;

        let stake_log = StakeLog {
            staker: ctx.accounts.from.to_account_info().key(),
            package_index: package_index,
            stake_timestamp: timestamp,
            terminated: false
        };

        staking_storage.stake_logs.push(stake_log);

        // update package state
        let slot_count = staking_storage.packages[package_index as usize].slot_count;
        staking_storage.packages[package_index as usize].slot_count = slot_count + 1;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, escrow_bump: u8, package_index: u8) -> Result<()> {
        // validate package index
        let packages = & ctx.accounts.staking_storage.packages;

        if package_index >= packages.len() as u8{
            return Err(ErrorCode::InvalidPackageIndex.into());
        }

        // check if user is valid staker and time lock
        let stake_logs = & ctx.accounts.staking_storage.stake_logs;
        let mut is_valid_staker = false;
        let mut active_stake_log: &StakeLog;
        let mut log_index: usize = 0;
        for stake_log in  stake_logs.iter() {
            if stake_log.staker == ctx.accounts.to.to_account_info().key() && package_index == stake_log.package_index {
                is_valid_staker = true;
                active_stake_log = stake_log;

                log_index = stake_logs.iter().position(|x| x.package_index == stake_log.package_index && x.staker == stake_log.staker).unwrap_or(0) as usize;

                // check if active stake
                if stake_log.terminated == true {
                    return Err(ErrorCode::StakeAlreadyTerminated.into());
                }

                // check time lock
                let clock = Clock::get();
                let timestamp = clock.unwrap().unix_timestamp;
                let expected_timestamp = active_stake_log.stake_timestamp + packages[package_index as usize].period;
            
                // if expected_timestamp > timestamp {
                //     return Err(ErrorCode::InvalidLockTime.into());
                // }
            }
            else {
                continue;
            }
        }
        if is_valid_staker == false {
            return Err(ErrorCode::AccountNeverStaked.into());
        }

        let mint_key = &mut ctx.accounts.mint.key();
        let seeds = &["escrow_vault".as_bytes(), mint_key.as_ref(), &[escrow_bump]];
        let signer_seeds = &[&seeds[..]];

        let transfer_instruction = Transfer{
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.escrow_vault.to_account_info(),
        };

        // start main withdraw/reward process - deposit token
        let withdraw_amount = ctx.accounts.staking_storage.packages[package_index as usize].reward_amount;

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_instruction, signer_seeds);

        token::transfer(cpi_ctx, withdraw_amount)?;

        let staking_storage = &mut ctx.accounts.staking_storage;
        staking_storage.stake_logs[log_index].terminated = true;

        Ok(())
    }

    pub fn charge_escrow(ctx: Context<EscrowCharge>, deposit_amount: u64) -> Result<()> {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        token::transfer(cpi_ctx, deposit_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space=size_of::<StakingStorage>() + 8000,
        seeds = [],
        bump)]
    pub staking_storage: Account<'info, StakingStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(init,
        payer = signer,
        owner = token_program.key(),
        seeds = [b"escrow_vault".as_ref(), mint.key().as_ref()],
        rent_exempt = enforce,
        token::mint = mint,
        token::authority = escrow_vault,
        bump)]
    pub escrow_vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    // #[account(mut)]
    // pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [], bump)]
    pub staking_storage: Account<'info, StakingStorage>,

    pub system_program: Program<'info, System>,

    #[account(mut,
        seeds = [b"escrow_vault".as_ref(), mint.key().as_ref()],
        bump)]
    pub escrow_vault: Account<'info, TokenAccount>,

    // #[account(init,
    //     payer = autority,
    //     owner = token_program.key(),
    //     seeds = [b"escrow_vault".as_ref(), mint.key().as_ref()],
    //     rent_exempt = enforce,
    //     token::mint = mint,
    //     token::authority = escrow_vault,
    //     bump)]
    // pub escrow_vault: Account<'info, TokenAccount>,
        
    /// Token mint.
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct EscrowCharge<'info> {
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    // #[account(mut)]
    // pub to: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut,
        seeds = [b"escrow_vault".as_ref(), mint.key().as_ref()],
        bump)]
    pub escrow_vault: Account<'info, TokenAccount>,
        
    /// Token mint.
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub token_program: Program<'info, Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [], bump)]
    pub staking_storage: Account<'info, StakingStorage>,

    #[account(mut,
        seeds = [b"escrow_vault".as_ref(), mint.key().as_ref()],
        bump)]
    pub escrow_vault: Account<'info, TokenAccount>,

    /// Token mint.
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,

    // #[account(mut)]
    // pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Package {
    pub name: String,
    pub deposit_amount: u64,
    pub period: i64,
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
    AccountAlreadyStaked,
    #[msg("Package slot fulfilled")]
    PackageSlotFulFilled,
    #[msg("Account never staked")]
    AccountNeverStaked,
    #[msg("Lock time period is not satisfied")]
    InvalidLockTime,
    #[msg("Stake already terminated")]
    StakeAlreadyTerminated
}