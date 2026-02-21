
use anchor_lang::prelude::*;


#[account]
pub struct Proposal {
    pub proposal_type: ProposalType,
    pub transfer_amount: u64,
    pub created_at: i64,//
    pub destination: Pubkey,
    pub time_lock_period: i64,
    pub executed_at: i64,//
    pub multisig: Pubkey,
    pub executed: bool,//
    pub approval_count: u8,//
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
#[repr(u8)]
pub enum ProposalType {
    TransferSol,
    TransferToken,
    ChangeVersion,
}
