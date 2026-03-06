use anchor_lang::error_code;

#[error_code]
pub enum MyError {
    #[msg("Invalid owner tried to approve")]
    InvalidOwnerError,
    #[msg("not enough approvals")]
    NotEnoughApproval,
    #[msg("reciepient account not recieved")]
    ReciepientAccountNotReceived,
    #[msg("noT valid proposer")]
    NotProposer,
    #[msg("not enough sol in vault")]
    NotEnoughSol,
    #[msg("overflow count")]
    Overflow,
    
    #[msg("Underflow count")]
    Underflow,
    #[msg("duplicate owners")]
    DuplicateOwners,
    #[msg("Not a Approver")]
    NoTApprover,
    #[msg("Not a Executor")]
    NoTExecutor,
   
    #[msg("Not a valid index")]
    InvalidApproverIndex,
    #[msg("Time not reached for execution")]
    TimeNotElasped,
   
    #[msg("Already Approved")]
    AlreadyApproved,
    #[msg("Already Executed")]
    AlreadyExecuted,
    #[msg("Cannot close the account yet")]
   CannotCloseYet,
   
    #[msg("It is not valid relationship")]
   InvalidRelationship,
   
    #[msg("proposal creator and proposer closer are different")]
   ProposerandCloserNotMatch,
    #[msg("signer is not a multisig participant")]
   NotParticipant,
    #[msg("this propsal doesnt belong to this multisig")]
   InvalidProposal,
    #[msg("proposal transfer amount is invalid")]
   InvalidAmount,
    #[msg("proposal type doesnt belong to this execution instruction it is for execute_proposal")]
   InvalidProposalType,
    #[msg("proposal and multsig config version mismatch it is a proposal before multisig config changed")]
   MultsigVersionMismatch,
    #[msg("Signer is Not a multisig creator")]
   NotMultisigCreator,
   
}