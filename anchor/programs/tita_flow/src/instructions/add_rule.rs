use crate::constants::MAX_RULES_PER_FLOW;
use crate::instructions::TitaError;
use crate::states::flow::{FlowAccount, FlowStatus, FlowType};
use crate::states::rule::{EscrowStep, Milestone, RuleAccount, RuleType};
use crate::utils::MerkleProof;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(rule_index: u8)]
pub struct AddRule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = flow_account.creator == authority.key() @ TitaError::UnauthorizedAccess,
        constraint = flow_account.status == FlowStatus::Draft @ TitaError::FlowNotDraft,
        constraint = flow_account.rule_accounts_counts < MAX_RULES_PER_FLOW @ TitaError::MaxRuleAccountsReached,
    )]
    pub flow_account: Account<'info, FlowAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + RuleAccount::INIT_SPACE,
        seeds = [
            b"rule",
            flow_account.key().as_ref(),
            &[rule_index],
        ],
        bump
    )]
    pub rule_account: Account<'info, RuleAccount>,

    pub system_program: Program<'info, System>,
}

// Define a public enum that the user will pass to indicate rule type
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RuleInput {
    TimeBased { unlock_time: i64 },
    MilestoneRule { milestones: Vec<(u8, u64)> },
    Whitelist { merkle_root: [u8; 32], total_addresses: u32 },
    Blacklist { merkle_root: [u8; 32], total_addresses: u32 },
    MinMax { min: u64, max: u64 },
    TimeWindow { start: i64, end: i64 },
    MilestoneEscrow { steps: Vec<(u8, u64, u8)> },
}

impl<'info> AddRule<'info> {
    pub fn add_rule(
        &mut self,
        rule_index: u8,
        rule_input: RuleInput,
    ) -> Result<()> {
        match rule_input {
            RuleInput::TimeBased { unlock_time } => {
                self.add_time_based_rule(unlock_time)
            },
            RuleInput::MilestoneRule { milestones } => {
                self.add_milestone_rule(milestones)
            },
            RuleInput::Whitelist { merkle_root, total_addresses } => {
                self.add_whitelist_rule(merkle_root, total_addresses)
            },
            RuleInput::Blacklist { merkle_root, total_addresses } => {
                self.add_blacklist_rule(merkle_root, total_addresses)
            },
            RuleInput::MinMax { min, max } => {
                self.add_minmax_rule(min, max)
            },
            RuleInput::TimeWindow { start, end } => {
                self.add_time_window_rule(start, end)
            },
            RuleInput::MilestoneEscrow { steps } => {
                self.add_milestone_escrow_rule(steps)
            },
        }
    }

    pub fn add_time_based_rule(
        &mut self,
        unlock_time: i64,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        // Validation
        if unlock_time <= current_time {
            return err!(TitaError::InvalidUnlockTime);
        }

        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Send {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::SendTimeBased { unlock_time };
        rule.is_active = true;
        rule.created_at = current_time;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "SendTimeBased".to_string(),
            created_at: current_time,
        });

        Ok(())
    }

    pub fn add_milestone_rule(
        &mut self,
        milestones: Vec<(u8, u64)>, // Vec of (id, amount) tuples
    ) -> Result<()> {
        // Validation
        if milestones.len() > 10 || milestones.is_empty() {
            return err!(TitaError::InvalidMilestoneCount);
        }

        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Send {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Convert milestone tuples to Milestone structs
        let milestone_structs: Vec<Milestone> = milestones
            .iter()
            .map(|(id, amount)| Milestone {
                id: *id,
                released: false,
                amount: *amount,
            })
            .collect();

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::SendMilestone {
            current: 0,
            milestones: milestone_structs,
        };
        rule.is_active = true;
        rule.created_at = Clock::get()?.unix_timestamp;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "SendMilestone".to_string(),
            created_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn add_whitelist_rule(
        &mut self,
        merkle_root: [u8; 32],
        total_addresses: u32,
    ) -> Result<()> {
        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Receive {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::ReceiveWhitelist {
            merkle_root,
            total_addresses,
        };
        rule.is_active = true;
        rule.created_at = Clock::get()?.unix_timestamp;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "ReceiveWhitelist".to_string(),
            created_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn add_blacklist_rule(
        &mut self,
        merkle_root: [u8; 32],
        total_addresses: u32,
    ) -> Result<()> {
        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Receive {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::ReceiveBlacklist {
            merkle_root,
            total_addresses,
        };
        rule.is_active = true;
        rule.created_at = Clock::get()?.unix_timestamp;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "ReceiveBlacklist".to_string(),
            created_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn add_minmax_rule(
        &mut self,
        min: u64,
        max: u64,
    ) -> Result<()> {
        // Validation
        if min >= max {
            return err!(TitaError::InvalidMinMaxValues);
        }

        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Receive {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::ReceiveMinMax { min, max };
        rule.is_active = true;
        rule.created_at = Clock::get()?.unix_timestamp;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "ReceiveMinMax".to_string(),
            created_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn add_time_window_rule(
        &mut self,
        start: i64,
        end: i64,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        // Validation
        if start >= end {
            return err!(TitaError::InvalidDateRange);
        }

        if start < current_time {
            return err!(TitaError::InvalidStartDate);
        }

        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Receive {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::ReceiveTimeWindow { start, end };
        rule.is_active = true;
        rule.created_at = current_time;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "ReceiveTimeWindow".to_string(),
            created_at: current_time,
        });

        Ok(())
    }

    pub fn add_milestone_escrow_rule(
        &mut self,
        steps: Vec<(u8, u64, u8)>, // Vec of (id, amount, threshold) tuples
    ) -> Result<()> {
        // Validation
        if steps.len() > 10 || steps.is_empty() {
            return err!(TitaError::InvalidMilestoneCount);
        }

        // Check flow type compatibility
        if self.flow_account.flow_type != FlowType::Receive {
            return err!(TitaError::IncompatibleRuleType);
        }

        // Convert steps tuples to EscrowStep structs
        let escrow_steps: Vec<EscrowStep> = steps
            .iter()
            .map(|(id, amount, threshold)| EscrowStep {
                id: *id,
                amount: *amount,
                released: false,
                votes_approve: 0,
                votes_reject: 0,
                withdrawal_threshold: *threshold,
            })
            .collect();

        // Initialize the rule account
        let rule = &mut self.rule_account;
        rule.flow = self.flow_account.key();
        rule.authority = self.authority.key();
        rule.rule_type = RuleType::ReceiveMilestoneEscrow {
            current_index: 0,
            steps: escrow_steps,
        };
        rule.is_active = true;
        rule.created_at = Clock::get()?.unix_timestamp;

        // Update the flow account's rule count
        let flow = &mut self.flow_account;
        flow.rule_accounts_counts += 1;

        emit!(RuleAddedEvent {
            flow: self.flow_account.key(),
            rule: self.rule_account.key(),
            rule_type: "ReceiveMilestoneEscrow".to_string(),
            created_at: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct RuleAddedEvent {
    pub flow: Pubkey,
    pub rule: Pubkey,
    pub rule_type: String,
    pub created_at: i64,
}
