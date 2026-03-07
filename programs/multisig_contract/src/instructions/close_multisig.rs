use anchor_lang::{ prelude::*};

use anchor_lang::system_program::{transfer, Transfer};

use crate::{error::MyError, states::*};

#[derive(Accounts)]
#[instruction(multisig_name:String)]


pub struct CloseMultisig<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"multisig",multisig_name.as_bytes().as_ref(),creator.key().as_ref()],
        bump,
        close=creator,
        constraint= multisig.creator ==creator.key() @ MyError::NotMultisigCreator
    )]
    pub multisig: Account<'info, MultisigState>,
   #[account(mut,
        seeds = [b"vault_state",multisig.key().as_ref(),creator.key().as_ref()],
        bump,
        close=creator,
        constraint= multisig.creator ==creator.key() @ MyError::NotMultisigCreator
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,
    seeds=[b"vault",multisig.key().as_ref()],
    bump
)]
    pub vault: SystemAccount<'info>,

 

    pub system_program: Program<'info, System>,
} 


impl <'info> CloseMultisig<'info> {
    pub fn close_multisig(&mut self)->Result<()>{



    let multisig_config_key = self.multisig.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            multisig_config_key.as_ref(),
            &[self.vault_state.vault_bump],
        ]];

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.creator.to_account_info(),
            },
        )
        .with_signer(signer_seeds);
        transfer(cpi_context, self.vault.lamports())?;
        msg!(
    "Transfer {} lamports from vault {} to {}",
    self.vault.lamports(),
    self.vault.key(),
    self.creator.key()
);


        Ok(())
    }


}