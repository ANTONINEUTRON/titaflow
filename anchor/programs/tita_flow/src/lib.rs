use anchor_lang::prelude::*;

declare_id!("2coGXpnEG9qJMzKZCxvgfSznWoDzVFazugp4VMSvbUYk");


pub mod utils;
use crate::utils::*;

pub mod instructions;
use crate::instructions::*;

pub mod states;
use crate::states::*;

pub mod constants;
use crate::constants::*;


#[program]
pub mod tita_flow {
    use super::*;

    // pub fn (ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    pub fn create_flow(
        ctx: Context<CreateFlow>, 
        flow_id: String,
        flow_type: u8,
        goal: u64,
        start_date: i64,
        end_date: Option<i64>,
    ) -> Result<()> {
        let _ = ctx.accounts.create(
            flow_id,
            flow_type,
            goal,
            start_date,
            end_date,
            ctx.accounts.flow_account.bump,
        )?;

        Ok(())
    }

    pub fn add_rule(
        ctx: Context<AddRule>, 
        rule_index: u8,
    ) -> Result<()> {
        let _ = ctx.accounts.add_rule(rule_type, "")?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
