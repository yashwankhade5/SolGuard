use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigState {
    #[max_len(50)]
    pub multisig_name: String,

    #[max_len(15)]
    pub participaints: Vec<Pubkey>,
    #[max_len(15)]
    pub approver: Vec<u8>,
    #[max_len(15)]
    pub proposer: Vec<u8>,
    #[max_len(15)]
    pub executor: Vec<u8>,
    #[max_len(15)]
    pub approver_weight: Vec<u8>,

    pub tx_count: u64,
    pub valut_state_bumps: u8,
    pub creator: Pubkey,
    pub created_at: i64,
    pub config_ver: u8,
    pub approve_threshold: u8,
}


#[account]
#[derive(InitSpace)]
pub struct  VaultState{
pub vault_state_bump:u8,
pub vault_bump:u8
}


