use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SendMilestone {
    pub current_milestone: u8,
    pub milestones: Vec<Milestone>, // list of milestones with checks
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Milestone {
    pub name: String,
    pub unlocked: bool,
    pub amount: u64, // how much to release
}