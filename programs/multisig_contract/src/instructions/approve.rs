use anchor_lang::prelude::*;

use crate::error::MyError;
use crate::states::*;

#[derive(Accounts)]
#[instruction(proposer_id:u64)]
pub struct ApprovalContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
mut,
        seeds = [b"multisig",multisig_config.multisig_name.as_bytes().as_ref(),multisig_config.creator.key().as_ref()],
        bump
    )]
    pub multisig_config: Account<'info, MultisigState>,

    #[account(mut,
    seeds=[b"proposal",multisig_config.key().as_ref(),proposer_id.to_le_bytes().as_ref()],
    bump

)]
    pub proposal: Account<'info, Proposal>,
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

impl<'info> ApprovalContext<'info> {
    pub fn approved(&mut self) -> Result<()> {
        let approver_index = self
            .multisig_config
            .participaints
            .iter()
            .position(|x| *x == self.signer.key())
            .ok_or(MyError::DuplicateOwners)? as u8;

        require!(
            self.multisig_config.approver.contains(&approver_index),
            MyError::NoTApprover
        );

        msg!(&(self.multisig_config.approver.contains(&approver_index)).to_string());
        require!(
            self.proposal.approved_by[approver_index as usize] == true,
            MyError::AlreadyApproved
        );

        let weight = self
            .multisig_config
            .approver_weight
            .get(approver_index as usize)
            .ok_or(MyError::InvalidApproverIndex)?;

        self.proposal.approval_count += *weight;
        self.proposal.approved_by[approver_index as usize] = true;

        Ok(())
    }
}
