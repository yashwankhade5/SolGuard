use anchor_lang::prelude::*;

declare_id!("9Pgyt8XkZZ9YjG9suPSGsbL6As8keiKA8HPmgpS9XvX");

#[program]
pub mod multisig_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
