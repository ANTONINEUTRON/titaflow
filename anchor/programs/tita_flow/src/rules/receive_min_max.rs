use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveMinMax {
    pub min: u64,
    pub max: u64,
}
