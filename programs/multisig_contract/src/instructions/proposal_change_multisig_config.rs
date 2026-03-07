
use anchor_lang::prelude::*;

use crate::error::MyError;
use crate::states::*;

#[derive(Accounts)]
pub struct ChangeConfigProposalContext<'info> {
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
    space=736,
    seeds=[b"proposal",multisig_config.key().as_ref(),multisig_config.tx_count.to_le_bytes().as_ref()],
    bump

)]
    pub proposal: Account<'info, ChangeConfigProposal>,

    pub system_program: Program<'info, System>,
}

impl<'info> ChangeConfigProposalContext<'info> {
    pub fn change_config_proposal_init(
        &mut self,
        proposal_type: ProposalType,
        multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
        approve_threshold: u8,
        time_lock_period: i64,
    ) -> Result<()> {
        let proposer_index = self
            .multisig_config
            .participaints
            .iter()
            .position(|x| *x == self.signer.key())
            .ok_or(MyError::NotProposer)? as u8;

        require!(
            self.multisig_config.proposer.contains(&proposer_index),
            MyError::NotProposer
        );

        let currenttime = Clock::get()?.unix_timestamp;

        let mut approved_by = vec![true; self.multisig_config.approver_weight.len()];

        for (index, val) in self.multisig_config.approver_weight.iter_mut().enumerate() {
            if *val == 0 {
                approved_by[index] = false;
            }
        }

        self.proposal.set_inner(ChangeConfigProposal {
            proposal_type,
            created_at: currenttime,
            executed: false,
            executed_at: 0,
            time_lock_period,
            multisig: self.multisig_config.key(),
            approval_count: 0,
            approved_by,
            proposer_id: self.multisig_config.tx_count,
            proposal_creator: self.signer.key(),
            proposal_multsig_config_ver: self.multisig_config.config_ver,
            multisig_name,
            participaints: owners,
            approver,
            proposer,
            executor,
            approver_weight,
            approve_threshold,
        });

        Ok(())
    }
}
