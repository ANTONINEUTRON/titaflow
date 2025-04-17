use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveBlacklist {
    pub blocked_addresses: Vec<Pubkey>,
}
