use anchor_lang::prelude::*;
pub mod states;
use crate::states::*;
pub mod instructions;
use crate::instructions::*;




declare_id!("9Pgyt8XkZZ9YjG9suPSGsbL6As8keiKA8HPmgpS9XvX");

#[program]
pub mod multisig_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
        approve_threshold: u8
    ) -> Result<()> {


        ctx.accounts.init_multisig( multisig_name,
        owners,
        approver,
        proposer,
        executor,
        approver_weight,
        approve_threshold)
               
    }


pub fn proposal_create(ctx:Context<ProposalContext>,proposal_type:ProposalType,transfer_amount:u64,
time_lock_perod:i64,  destination:Pubkey)->Result<()>{

    ctx.accounts.proposer_init(proposal_type,transfer_amount,time_lock_perod,  destination)
    
}




}


