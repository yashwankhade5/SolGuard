use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub proposal_type: ProposalType,
    pub transfer_amount: u64,
    pub created_at: i64, //
    pub destination: Pubkey,
    pub time_lock_period: i64,
    pub executed_at: i64, //
    pub multisig: Pubkey,
    pub executed: bool,     //
    pub approval_count: u8, //
    pub approved_by: Vec<bool>,
    pub proposer_id: u64,
    pub proposal_creator: Pubkey,
    pub proposal_multsig_config_ver: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProposalType {
    TransferSol,
    TransferToken,
    ChangeVersion,
}

#[account]
pub struct ChangeConfigProposal {
    pub proposal_type: ProposalType,
    pub created_at: i64,  //
    pub executed: bool,   //
    pub executed_at: i64, //
    pub time_lock_period: i64,
    pub multisig: Pubkey,
    pub approval_count: u8,
    pub approved_by: Vec<bool>,
    pub proposer_id: u64,
    pub proposal_creator: Pubkey,
    pub proposal_multsig_config_ver: u8,
    pub multisig_name: String,
    pub participaints: Vec<Pubkey>,
    pub approver: Vec<u8>,
    pub proposer: Vec<u8>,
    pub executor: Vec<u8>,
    pub approver_weight: Vec<u8>,
    pub approve_threshold: u8,
}
