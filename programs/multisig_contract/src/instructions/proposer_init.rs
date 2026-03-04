use anchor_lang::prelude::*;

use crate::error::MyError;
use crate::states::*;

#[derive(Accounts)]
pub struct ProposalContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
mut,
        seeds = [b"multisig",multisig_config.multisig_name.as_bytes().as_ref(),multisig_config.creator.key().as_ref()],
        bump
    )]
    pub multisig_config: Account<'info, MultisigState>,

    #[account(init,
    payer=signer,
    space=8+1                               // ProposalType enum (1 byte)
        + 8                                 // transfer_amount u64
        + 8                                 // created_at i64
        + 32                                // destination Pubkey
        + 8                                 // time_lock_period i64
        + 8                                 // executed_at i64
        + 32                                // multisig Pubkey
        + 1                                 // executed bool
        + 1 
        +8
        +8
        +32,
    seeds=[b"proposal",multisig_config.key().as_ref(),multisig_config.tx_count.to_le_bytes().as_ref()],
    bump

)]
    pub proposal: Account<'info, Proposal>,

    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProposalContext<'info> {
    pub fn proposer_init(
        &mut self,
        proposal_type: ProposalType,
        transfer_amount: u64,
        time_lock_perod: i64,
        destination: Pubkey,
    ) -> Result<()> {


        let proposer_index =
            self.multisig_config
                .participaints
                .iter()
                .position(|x| *x == self.signer.key())
                .ok_or(MyError::NotProposer)? as u8;

              
            msg!("signer {}", self.signer.key());
msg!("participants {:?}", self.multisig_config.participaints);
msg!("proposer indexes {:?}", self.multisig_config.proposer);
msg!("derived proposer index {}", proposer_index);



        require!(
            self.multisig_config.proposer.contains(&proposer_index),
            MyError::NotProposer
        );


        let timestamp = self.clock.unix_timestamp;

        let mut approved_by = vec![true; self.multisig_config.approver_weight.len()];

        for (index, val) in self.multisig_config.approver_weight.iter_mut().enumerate() {
            if *val == 0 {
                approved_by[index] = false;
            }
        }

        self.proposal.set_inner(Proposal {
            proposal_type,
            transfer_amount,
            created_at: timestamp,
            destination,
            time_lock_period: time_lock_perod,
            executed_at: 0,
            multisig: self.multisig_config.key(),
            approved_by,
            executed: false,
            approval_count: 0,
            proposer_id:self.multisig_config.tx_count,
            proposal_creator:self.signer.key()
        });

        self.multisig_config.tx_count +=1;
        
        Ok(())
    }
}
