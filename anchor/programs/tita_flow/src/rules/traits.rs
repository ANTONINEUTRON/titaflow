use anchor_lang::prelude::*;

use crate::states::RuleType;

/// Core trait that all rules must implement
pub trait Rule<'info> {
    /// Checks if the rule is satisfied under current conditions
    fn is_satisfied(&self) -> Result<bool>;
    
    /// Returns the rule type
    fn rule_type(&self) -> RuleType;
    
    /// Updates rule state if needed
    fn update(&mut self) -> Result<()>;

    /// Validates the rule parameters
    fn validate_parameters(&self) -> Result<()>;
}
