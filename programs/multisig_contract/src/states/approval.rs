// use anchor_lang::prelude::*;
// use crate::states::*;


// #[derive(Accounts)]
// pub struct ApprovalContext<'info>{
//     #[account(mut)]
//     pub signer:Signer<'info>,
//     #[account(
//          seeds = [b"multisig",multisig.multisig_name.as_bytes().as_ref(),signer.key().as_ref()],
//         bump
//     )]
//     pub multisig:Account<'info,MultisigState>,

//     #[account( seeds=[b"proposal",multisig.key().as_ref(),multisig.tx_count.to_le_bytes().as_ref()],
//     bump)]
//     pub proposal:Account<'info,Proposal>
// }