use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};

// Native program's instruction enum
#[derive(BorshSerialize, BorshDeserialize)]
pub enum CounterInstruction {
    Initialize,
    Double,
    Half,
}

declare_id!("3vpQib3CfZgzDZkZjjJcfW6ikMee8kp8jLDYNP9Kj83w"); // Replace with your CPI wrapper program ID

#[program]
pub mod cpi_wrapper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let ix_data = CounterInstruction::Initialize.try_to_vec()
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;

        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, false),
            AccountMeta::new(*ctx.accounts.user_account.key, true),
            AccountMeta::new_readonly(anchor_lang::system_program::ID, false),
        ];

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: ix_data,
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.data_account.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.native_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn double(ctx: Context<Modify>) -> Result<()> {
        let ix_data = CounterInstruction::Double.try_to_vec()
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;

        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, true),
        ];

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: ix_data,
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.data_account.to_account_info(),
                ctx.accounts.native_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn half(ctx: Context<Modify>) -> Result<()> {
        let ix_data = CounterInstruction::Half.try_to_vec()
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;

        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, true),
        ];

        let ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.native_program.key(),
            accounts,
            data: ix_data,
        };

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.data_account.to_account_info(),
                ctx.accounts.native_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: Safe because only passed to CPI
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: Payer for account creation
    #[account(mut, signer)]
    pub user_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Native program account
    pub native_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    /// CHECK: Safe because only passed to CPI
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: Native program account
    pub native_program: AccountInfo<'info>,
}
