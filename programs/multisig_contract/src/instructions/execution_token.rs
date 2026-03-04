use crate::error::MyError;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked,TransferChecked};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct ExecutionTokenContext<'info> {
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

    pub destination: SystemAccount<'info>,

    #[account(
        seeds = [b"vault_state",multisig_config.key().as_ref(),signer.key().as_ref()],
        bump= vault_state.vault_state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,
    seeds=[b"vault",multisig_config.key().as_ref()],
    bump= vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    
#[account(
        init_if_needed,
        payer=signer,
        associated_token::mint= mint,
        associated_token::authority = multisig_config,
        associated_token::token_program=token_program
        
    )]
    pub multisig_ata: InterfaceAccount<'info, TokenAccount>,
#[account(
        init_if_needed,
        payer=signer,
        associated_token::mint= mint,
        associated_token::authority = destination,
        associated_token::token_program=token_program
        
    )]
    pub destination_ata: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecutionTokenContext<'info> {
    pub fn transfer_token(&mut self,multisig_bump:u8) -> Result<()> {
        let executor_index =
            self.multisig_config
                .participaints
                .iter()
                .position(|x| *x == self.signer.key())
                .unwrap_or(self.multisig_config.participaints.len() + 100) as u8;

        require!(
            self.multisig_config.executor.contains(&executor_index),
            MyError::NoTExecutor
        );
        let current_time = self.clock.unix_timestamp;

        require!(
            self.proposal.time_lock_period < current_time,
            MyError::TimeNotElasped
        );
        require!(
            self.multisig_config.approve_threshold <= self.proposal.approval_count,
            MyError::NotEnoughApproval
        );
        require!(
            self.proposal.executed == false,
            MyError::AlreadyExecuted
        );

        let creator = self.multisig_config.creator.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"multisig",
            self.multisig_config.multisig_name.as_bytes().as_ref(),
            creator.as_ref(),&[multisig_bump]
        ]];
        let cpi_context = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.multisig_ata.to_account_info(),
                to: self.destination_ata.to_account_info(),
                mint:self.mint.to_account_info(),
                authority:self.multisig_config.to_account_info()
            },
        )
        .with_signer(signer_seeds);
        transfer_checked(cpi_context, self.proposal.transfer_amount,self.mint.decimals)?;

        self.proposal.executed=true;
        self.proposal.executed_at =current_time;

        Ok(())
    }
}
