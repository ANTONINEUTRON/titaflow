use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::TITA_FLOW_SEED;
use crate::instructions::TitaError;
use crate::states::flow::{FlowAccount, FlowStatus, FlowType};

// create a flow account
// if flow type is send, debit creator wallet to fund the flow token account
// if flow type is receive, just specify flow fields

#[derive(Accounts)]
#[instruction(
    flow_id: String
)]
pub struct CreateFlow<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + FlowAccount::INIT_SPACE,
        seeds = [
            TITA_FLOW_SEED,
            creator.key().as_ref(),
            flow_id.as_bytes()
        ],
        bump
    )]
    pub flow_account: Account<'info, FlowAccount>,

    /// Associated token account for the flow (escrow)
    #[account(
        init,
        payer = creator,
        seeds = [
            TITA_FLOW_SEED,
            flow_account.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = flow_account,
    )]
    pub flow_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateFlow<'info> {
    pub fn create(
        &mut self,
        flow_id: String,
        flow_type: u8,
        goal: u64,
        start_date: i64,
        end_date: Option<i64>,
        bump: u8,
    ) -> Result<()> {
        // Validate input parameters
        if flow_id.is_empty() || flow_id.len() > 32 {
            return err!(TitaError::InvalidFlowId);
        }

        // Validate start date and end date
        let current_time = Clock::get()?.unix_timestamp;

        if start_date < current_time {
            return err!(TitaError::InvalidStartDate);
        }

        // Optional end date must be after start date if specified
        if let Some(end) = end_date {
            if end <= start_date {
                return err!(TitaError::InvalidDateRange);
            }
        }

        // Convert flow_type from u8 to the actual enum
        let flow_type = match flow_type {
            0 => FlowType::Send,
            1 => FlowType::Receive,
            _ => return err!(TitaError::InvalidFlowType),
        };

        // Initialize the flow account
        let flow = &mut self.flow_account;
        flow.flow_id = flow_id;
        flow.flow_type = flow_type;
        flow.creator = self.creator.key();
        flow.goal = goal;
        flow.raised = 0;
        flow.balance_in_ta = 0;
        flow.token_mint = self.token_mint.key();
        flow.start_date = start_date;
        flow.end_date = end_date;
        flow.sender_count = 0;
        flow.receiver_count = 0;
        flow.rule_accounts_counts = 0;
        flow.status = FlowStatus::Draft;
        flow.bump = bump;

        // Emit an event for indexing
        emit!(FlowCreatedEvent {
            flow_id: flow.flow_id.clone(),
            creator: flow.creator,
            goal,
            token_mint: flow.token_mint,
            start_date,
            end_date,
            flow_type,
            timestamp: Clock::get()?.unix_timestamp,
        });

        msg!("Flow created: {}", flow.flow_id);
        Ok(())
    }
}


#[event]
pub struct FlowCreatedEvent {
    pub flow_id: String,
    pub creator: Pubkey,
    pub goal: u64,
    pub token_mint: Pubkey,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub flow_type: FlowType,
    pub timestamp: i64,
}
