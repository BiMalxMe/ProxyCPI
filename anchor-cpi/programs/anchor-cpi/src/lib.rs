use anchor_lang::prelude::*;

declare_id!("3vpQib3CfZgzDZkZjjJcfW6ikMee8kp8jLDYNP9Kj83w");

#[program]
pub mod anchor_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
