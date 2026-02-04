use anchor_lang::prelude::*;

declare_id!("Pursh1111111111111111111111111111111111");

#[program]
pub mod pursh {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.pursh_data;
        data.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let data = &mut ctx.accounts.pursh_data;
        data.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub pursh_data: Account<'info, PurshData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub pursh_data: Account<'info, PurshData>,
}

#[account]
pub struct PurshData {
    pub counter: u64,
}
