use anchor_lang::prelude::*;

declare_id!("C6SrAByDmBm4u5dcZLYJErYirVn4o6tf6329SpFoVbW7");

#[program]
pub mod cpi_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {          
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, false),
            AccountMeta::new(*ctx.accounts.user_account.key, true),
            AccountMeta::new_readonly(anchor_lang::system_program::ID, false),
        ];

        //construct 
        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts,
            data: vec![0],
        };

        // CPI to the native counter program
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                // to account info gives the metadata of the key
                ctx.accounts.cpi_program.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn double(ctx: Context<Initialize>) -> Result<()> {
        
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.data_account.key, true),
        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts,
            data: vec![1],
        };

        // CPI to the native counter program
        // CPI to the native counter program
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[ctx.accounts.data_account.to_account_info()],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is safe because it’s only passed to the CPI call and not accessed directly.
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: This must be a signer and mutable as it will pay for the transaction.
    #[account(mut, signer)]
    pub user_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is the external CPI program being invoked. We don’t access its data.
    pub cpi_program: AccountInfo<'info>,
}
