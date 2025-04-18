use anchor_lang::prelude::*;

#[error_code]
pub enum TitaError {
    #[msg("Unauthorized access to rule")]
    Unauthorized,
    
    #[msg("Flow ID cannot be empty or longer than 32 characters")]
    InvalidFlowId,
    
    #[msg("Start date must be in the future")]
    InvalidStartDate,
    
    #[msg("End date must be after start date")]
    InvalidDateRange,
    
    #[msg("Invalid flow type provided. Must be 0 (Send) or 1 (Receive)")]
    InvalidFlowType,

    #[msg("Unauthorized access: only the flow creator can add rules")]
    UnauthorizedAccess,
    
    #[msg("Flow must be in draft status to add rules")]
    FlowNotDraft,
    
    #[msg("Maximum number of rules (10) reached for this flow")]
    MaxRuleAccountsReached,
    
    #[msg("Invalid unlock time: must be set in the future")]
    InvalidUnlockTime,
    
    #[msg("This rule type is not compatible with the flow type")]
    IncompatibleRuleType,
    
    #[msg("Invalid milestone count: must be between 1 and 10")]
    InvalidMilestoneCount,
    
    #[msg("Minimum value must be less than maximum value")]
    InvalidMinMaxValues
}