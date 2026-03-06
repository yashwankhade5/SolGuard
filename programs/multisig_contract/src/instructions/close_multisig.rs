use anchor_lang::{accounts::signer, prelude::*};

use crate::{error::MyError, states::*};

#[derive(Accounts)]
#[instruction(multisig_name:String)]


pub struct Initialize<'info> {
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