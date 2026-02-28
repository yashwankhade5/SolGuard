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
   
}