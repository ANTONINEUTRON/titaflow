use anchor_lang::prelude::*;

declare_id!("2coGXpnEG9qJMzKZCxvgfSznWoDzVFazugp4VMSvbUYk");


pub mod utils;
use crate::utils::*;

pub mod instructions;
use crate::instructions::*;

pub mod states;
use crate::states::*;


#[program]
pub mod tita_flow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
