use crate::error::MyError;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked,TransferChecked};

#[derive(Accounts)]
#[instruction(proposal_id:u64)]
pub struct ExecutionTokenContext<'info> {
    #[account(mut,
     constraint = multisig_config.participaints.contains(&signer.key()) @ MyError::NotParticipant)]
    pub signer: Signer<'info>,

    #[account(mut,
        seeds = [b"multisig",multisig_config.multisig_name.as_bytes().as_ref(),signer.key().as_ref()],
        bump)]
    pub multisig_config: Account<'info, MultisigState>,

    #[account(mut,
            seeds=[b"proposal",multisig_config.key().as_ref(),proposal_id.to_le_bytes().as_ref()],
            bump,
        constraint = proposal.multisig == multisig_config.key() @ MyError::InvalidRelationship
        )]
    pub proposal: Account<'info, Proposal>,

    pub destination: SystemAccount<'info>,

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
    pub system_program: Program<'info, System>,
}

impl<'info> ExecutionTokenContext<'info> {

    pub fn transfer_token(&mut self,multisig_bump:u8) -> Result<()> {
        let executor_index =
            self.multisig_config
                .participaints
                .iter()
                .position(|x| *x == self.signer.key())
                .ok_or(MyError::NotParticipant)? as u8;
              require!(
            self.multisig_config.config_ver==self.proposal.proposal_multsig_config_ver,
            MyError::MultsigVersionMismatch
        );

            require!(
            self.proposal.multisig == self.multisig_config.key(),
            MyError::InvalidProposal
            );

            require!(
                self.proposal.proposal_type == ProposalType::TransferToken,
                MyError::InvalidProposalType
            );

            require!(
                self.proposal.transfer_amount > 0,
                MyError::InvalidAmount
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
        require!(
            self.proposal.executed == false,
            MyError::AlreadyExecuted
        );

        
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"multisig",
            self.multisig_config.multisig_name.as_bytes(),
            self.multisig_config.creator.as_ref(),&[multisig_bump]
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
