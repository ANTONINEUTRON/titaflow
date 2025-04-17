use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SendTimeBased {
    pub unlock_time: i64, // Unix timestamp that funds will be released
}
