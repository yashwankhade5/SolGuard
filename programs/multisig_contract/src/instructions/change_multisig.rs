use anchor_lang::prelude::*;

use crate::error::MyError;
use crate::states::*;

#[derive(Accounts)]
#[instruction(multisig_name:String,proposal_id:u64)]
pub struct ChangeMultisigConfig<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut,
        seeds = [b"multisig",multisig_name.as_bytes().as_ref(),creator.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(mut,
    seeds=[b"proposal",multisig.key().as_ref(),proposal_id.to_le_bytes().as_ref()],
    bump)]
    pub proposal: Account<'info, ChangeConfigProposal>,

    pub system_program: Program<'info, System>,
}

impl<'info> ChangeMultisigConfig<'info> {
    pub fn change_config(
        &mut self,
    ) -> Result<()> {
        let executor_index = self
            .multisig
            .participaints
            .iter()
            .position(|x| *x == self.creator.key())
            .ok_or(MyError::NotParticipant)? as u8;
        require!(
            self.multisig.config_ver == self.proposal.proposal_multsig_config_ver,
            MyError::MultsigVersionMismatch
        );

        require!(
            self.proposal.multisig == self.multisig.key(),
            MyError::InvalidProposal
        );

        require!(
            self.proposal.proposal_type == ProposalType::ChangeVersion,
            MyError::InvalidProposalType
        );
        require!(
            self.multisig.config_ver == self.proposal.proposal_multsig_config_ver,
            MyError::MultsigVersionMismatch
        );
        require!(
            self.multisig.executor.contains(&executor_index),
            MyError::NoTExecutor
        );
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            self.proposal.time_lock_period < current_time,
            MyError::TimeNotElasped
        );
        require!(
            self.multisig.approve_threshold <= self.proposal.approval_count,
            MyError::NotEnoughApproval
        );
        require!(self.proposal.executed == false, MyError::AlreadyExecuted);

        self.multisig.multisig_name = self.proposal.multisig_name.clone();
        self.multisig.participaints =self.proposal.participaints.clone();
        self.multisig.approver = self.proposal.approver.clone();
        self.multisig.proposer = self.proposal.proposer.clone();
        self.multisig.executor = self.proposal.executor.clone();
        self.multisig.approver_weight = self.proposal.approver_weight.clone();
        self.multisig.approve_threshold = self.proposal.approve_threshold.clone();

        Ok(())
    }
}
