use crate::states::*;
use anchor_lang::prelude::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(proposer_id:u64)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
        #[account(
mut,
        seeds = [b"multisig",multisig_config.multisig_name.as_bytes().as_ref(),multisig_config.creator.key().as_ref()],
        bump
    )]
    pub multisig_config: Account<'info, MultisigState>,


     #[account(mut,
    seeds=[b"vault",multisig_config.key().as_ref()],
    bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut,
      seeds=[b"proposal",multisig_config.key().as_ref(),proposer_id.to_le_bytes().as_ref()],
    bump,
    close = vault,
    constraint = proposal.multisig == multisig_config.key() @ MyError::InvalidRelationship,
    constraint = proposal.proposal_creator ==signer.key()  @MyError::ProposerandCloserNotMatch
    
    )]
    pub proposal: Account<'info, Proposal>,
 
       pub clock: Sysvar<'info, Clock>,
}

impl <'info> Close<'info>  {
    pub fn close_proposal_pda(&mut self)->Result<()>{

        let current_time = self.clock.unix_timestamp;
 require!(
        self.proposal.executed || current_time > self.proposal.time_lock_period,
        MyError::CannotCloseYet
    );
        Ok(())
    }
}
