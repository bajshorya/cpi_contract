use anchor_lang::prelude::*;

declare_id!("6iR6Mf3h5oXYwKUznwNrqijM93uuTeRt8nL4TtGjES7H");

#[program]
pub mod cpi_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
