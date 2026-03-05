use anchor_lang::prelude::*;

use crate::error::MyError;
use crate::states::*;

#[derive(Accounts)]
#[instruction(multisig_name:String)]
pub struct ChangeMultisigConfig<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer=creator,
        space= 8+MultisigState::INIT_SPACE,
        seeds = [b"multisig",multisig_name.as_bytes().as_ref(),creator.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,
 

    pub system_program: Program<'info, System>,
}

impl <'info> ChangeMultisigConfig<'info>{
  pub fn change_config(&mut self, multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
       
        approve_threshold: u8,
        )->Result<()>{
            

self.multisig.multisig_name=multisig_name;
self.multisig.participaints=owners;
self.multisig.approver=approver;
self.multisig.proposer=proposer;
self.multisig.executor=executor;
self.multisig.approver_weight=approver_weight;
self.multisig.approve_threshold=approve_threshold;



    Ok(())
  }
}