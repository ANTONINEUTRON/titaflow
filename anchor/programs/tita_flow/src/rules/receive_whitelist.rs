use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveWhitelist {
    pub allowed_addresses: Vec<Pubkey>,
}
