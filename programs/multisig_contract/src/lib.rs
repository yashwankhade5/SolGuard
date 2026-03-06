use anchor_lang::prelude::*;
pub mod states;
use crate::states::*;
pub mod instructions;
use crate::instructions::*;
pub mod error;

declare_id!("4DD8CuS3Nvw42HmoSChupMgr6Q5E5eR7DpX1bYfL52R3");

#[program]
pub mod multisig_contract {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
        approve_threshold: u8,
    ) -> Result<()> {
        ctx.accounts.init_multisig(
            multisig_name,
            owners,
            approver,
            proposer,
            executor,
            approver_weight,
            approve_threshold,
            ctx.bumps.vault,
            ctx.bumps.vault_state,
        )
    }

    pub fn proposal_create(
        ctx: Context<ProposalContext>,
        proposal_type: ProposalType,
        transfer_amount: u64,
        time_lock_perod: i64,
        destination: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .proposer_init(proposal_type, transfer_amount, time_lock_perod, destination)
    }
    pub fn approva_proposal(ctx: Context<ApprovalContext>, _proposer_id: u64) -> Result<()> {
        ctx.accounts.approved()?;

        Ok(())
    }
    pub fn execute_proposal(ctx: Context<ExecutionContext>, _proposal_id: u64) -> Result<()> {
        ctx.accounts.transfer_sol()?;

        Ok(())
    }
    pub fn execute_token_proposal(
        ctx: Context<ExecutionTokenContext>,
        _proposal_id: u64,
    ) -> Result<()> {
        ctx.accounts.transfer_token(ctx.bumps.multisig_config)?;

        Ok(())
    }

    pub fn close_proposal(ctx: Context<Close>, _proposer_id: u64) -> Result<()> {
        ctx.accounts.close_proposal_pda()?;
        Ok(())
    }

    // pub fn change_multis(
    //     ctx: Context<ChangeMultisigConfig>,
    //     multisig_name: String,
    //     owners: Vec<Pubkey>,
    //     approver: Vec<u8>,
    //     proposer: Vec<u8>,
    //     executor: Vec<u8>,
    //     approver_weight: Vec<u8>,
    //     approve_threshold: u8,
    // ) -> Result<()> {

    //     ctx.accounts.change_config(
    //         multisig_name,
    //         owners,
    //         approver,
    //         proposer,
    //         executor,
    //         approver_weight,
    //         approve_threshold,
    //     )
    // }
    pub fn change_multis_proposal(
        ctx: Context<ChangeConfigProposalContext>,
        proposal_type: ProposalType,
        multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
        approve_threshold: u8,
        time_lock_perod: i64
    ) -> Result<()> {
        ctx.accounts.change_config_proposal_init(
            proposal_type,
            multisig_name,
            owners,
            approver,
            proposer,
            executor,
            approver_weight,
            approve_threshold,
            time_lock_perod
        )
    }
}
