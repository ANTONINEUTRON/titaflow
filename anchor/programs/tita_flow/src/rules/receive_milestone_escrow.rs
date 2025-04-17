use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveMilestoneEscrow {
    pub current_index: u8,
    pub steps: Vec<EscrowStep>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EscrowStep {
    pub name: String,
    pub release_amount: u64,
    pub is_released: bool,
}
