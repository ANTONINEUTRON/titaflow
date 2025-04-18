use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FlowAccount {
    #[max_len(32)]
    pub flow_id: String,
    pub flow_type: FlowType,
    pub creator: Pubkey,
    pub goal: u64,
    pub raised: u64,
    pub balance_in_ta: u64,
    pub token_mint: Pubkey,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub sender_count: u32,
    pub receiver_count: u32,
    pub rule_accounts_counts: u8, // A flow with 0 rule just have a token account created for the flow
    pub status: FlowStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum FlowStatus {
    /// Flow has been created but not yet started (waiting for start_date or activation)
    Draft,
    
    /// Flow is active and accepting funds/ready for disbursement
    Active,
    
    /// Flow is temporarily paused (e.g., rule violation or manual pause)
    Paused,
    
    /// Flow has reached its goal or completed its purpose
    Completed,
    
    /// Flow has been canceled by the creator
    Canceled,
    
    /// Flow has reached its end_date without reaching the goal
    Expired,
    
    /// Flow is in dispute or under review
    UnderReview
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum FlowType {
    Send,
    Receive
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum RuleType {
    // Send Rules
    SendTimeBased,
    SendMilestone,
    SendDaoBased,

    // Receive Rules
    ReceiveWhitelist,
    ReceiveBlacklist,
    ReceiveMinMax,
    ReceiveTimeWindow,
    ReceiveMilestoneEscrow,
}

