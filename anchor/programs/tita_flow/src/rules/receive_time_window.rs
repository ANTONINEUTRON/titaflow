use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveTimeWindow {
    pub start_time: i64,
    pub end_time: i64,
}
