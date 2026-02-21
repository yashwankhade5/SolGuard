use anchor_lang::prelude::*;

use crate::states::*;

#[derive(Accounts)]
#[instruction(multisig_name:String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer=creator,
        space= 8+MultisigState::INIT_SPACE,
        seeds = [b"multisig",multisig_name.as_bytes().as_ref(),creator.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, MultisigState>,

    #[account(mut,
    seeds=[b"vault",vault_state.key().as_ref()],
    bump)]
    pub vault: SystemAccount<'info>,

    #[account(init,
    payer=creator,
     space= 8+MultisigState::INIT_SPACE,
        seeds = [b"vault_state",multisig.key().as_ref(),creator.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_multisig(
        &mut self,
        multisig_name: String,
        owners: Vec<Pubkey>,
        approver: Vec<u8>,
        proposer: Vec<u8>,
        executor: Vec<u8>,
        approver_weight: Vec<u8>,
       
        approve_threshold: u8,
    ) -> Result<()> {
        let timestamp = self.clock.unix_timestamp;

        self.multisig.set_inner(MultisigState {
            multisig_name,
            participaints: owners,
            approver,
            proposer,
            executor,
            approver_weight,
            tx_count:0,
            valut_state_bumps: self.vault_state.vault_state_bump,
            creator: self.creator.key(),
            created_at: timestamp,
            config_ver: 0,
            approve_threshold,
        });


        Ok(())
    }
}
