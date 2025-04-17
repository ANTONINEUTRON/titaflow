use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SendDaoBased {
    pub dao_address: Pubkey,
    pub proposal_id: u64,
    pub required_votes: u64,
    pub has_passed: bool,
}
