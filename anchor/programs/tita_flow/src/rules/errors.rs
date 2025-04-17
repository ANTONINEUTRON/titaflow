use anchor_lang::prelude::*;

#[error_code]
pub enum RuleError {
    #[msg("Rule is not satisfied")]
    RuleNotSatisfied,
    
    #[msg("Rule is not applicable")]
    RuleNotApplicable,
    
    #[msg("Invalid rule parameters")]
    InvalidRuleParameters,
    
    #[msg("Rule verification failed")]
    VerificationFailed,
    
    #[msg("Unauthorized access to rule")]
    Unauthorized,
    
    #[msg("Rule has expired")]
    RuleExpired,
}