use crate::error::MyError;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct ExecutionContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,
        seeds = [b"multisig",multisig_config.multisig_name.as_bytes().as_ref(),signer.key().as_ref()],
        bump)]
    pub multisig_config: Account<'info, MultisigState>,

    #[account(mut,
    seeds=[b"proposal",multisig_config.key().as_ref(),proposal_id.to_le_bytes().as_ref()],
    bump)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub destination: SystemAccount<'info>,

    #[account(
        seeds = [b"vault_state",multisig_config.key().as_ref(),multisig_config.creator.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,
    seeds=[b"vault",multisig_config.key().as_ref()],
    bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecutionContext<'info> {
    pub fn transfer_sol(&mut self) -> Result<()> {
        let executor_index = self
            .multisig_config
            .participaints
            .iter()
            .position(|x| *x == self.signer.key())
            .ok_or(MyError::NoTExecutor)? as u8;

        require!(
            self.multisig_config.config_ver == self.proposal.proposal_multsig_config_ver,
            MyError::MultsigVersionMismatch
        );

        require!(
            self.multisig_config.executor.contains(&executor_index),
            MyError::NoTExecutor
        );

        let current_time = Clock::get()?.unix_timestamp;

        require!(
            self.proposal.time_lock_period < current_time,
            MyError::TimeNotElasped
        );
        require!(
            self.multisig_config.approve_threshold <= self.proposal.approval_count,
            MyError::NotEnoughApproval
        );

        let multisig_config_key = self.multisig_config.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            multisig_config_key.as_ref(),
            &[self.vault_state.vault_bump],
        ]];

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.destination.to_account_info(),
            },
        )
        .with_signer(signer_seeds);
        transfer(cpi_context, self.proposal.transfer_amount)?;

        self.proposal.executed = true;
        self.proposal.executed_at = current_time;
        Ok(())
    }
}
