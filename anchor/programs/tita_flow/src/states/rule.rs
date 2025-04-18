use anchor_lang::prelude::*;

use crate::utils::{verify_merkle_proof, MerkleProof};

#[account]
#[derive(InitSpace)]
pub struct RuleAccount {
    pub flow: Pubkey,            // Associated FlowAccount
    pub authority: Pubkey,       // Creator of this rule (could be flow creator or DAO)
    pub rule_type: RuleType,     // Enum of the actual rule
    pub is_active: bool,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum RuleType {
    SendTimeBased { 
        unlock_time: i64 
    },
    SendMilestone { 
        current: u8, 
        #[max_len(10)]
        milestones: Vec<Milestone> 
    },

    ReceiveWhitelist { 
        merkle_root: [u8; 32],     // Store just the root hash of the Merkle tree
        total_addresses: u32,       // For informational purposes
    },
    
    ReceiveBlacklist { 
        merkle_root: [u8; 32],     // Store just the root hash of the Merkle tree
        total_addresses: u32,       // For informational purposes
    },
    
    ReceiveMinMax { 
        min: u64, 
        max: u64 
    },
    
    ReceiveTimeWindow { 
        start: i64, 
        end: i64 
    },
    
    ReceiveMilestoneEscrow { 
        current_index: u8, 
        #[max_len(10)]
        steps: Vec<EscrowStep> 
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Milestone {
    pub id: u8,
    pub released: bool,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct EscrowStep {
    pub id: u8,
    pub amount: u64,
    pub released: bool,
    pub votes_approve: u32,     // Donors who voted yes
    pub votes_reject: u32,      // Donors who voted no
    pub withdrawal_threshold: u8, // % needed to approve (e.g. 51)
}


impl RuleAccount {
    pub fn is_satisfied(&self, context: &RuleContext) -> Result<bool> {
        match &self.rule_type {
            RuleType::SendTimeBased { unlock_time } => {
                let current_time = Clock::get()?.unix_timestamp;
                Ok(current_time >= *unlock_time)
            },
            
            RuleType::SendMilestone { current, milestones } => {
                // Assuming all previous milestones are completed
                Ok(*current as usize >= milestones.len())
            },
            
            RuleType::ReceiveWhitelist { merkle_root, .. } => {
                // To verify, the client must provide a valid Merkle proof in the transaction
                if let (Some(sender), Some(merkle_proof)) = (context.sender, &context.merkle_proof) {
                    // Verify that the sender is in the whitelist using the Merkle proof
                    let is_valid = verify_merkle_proof(
                        merkle_proof,
                        *merkle_root,
                        sender.to_bytes(),
                    )?;
                    Ok(is_valid)
                } else {
                    msg!("Missing sender or merkle proof for whitelist verification");
                    Ok(false)
                }
            },
            
            RuleType::ReceiveBlacklist { merkle_root, .. } => {
                if let (Some(sender), Some(merkle_proof)) = (context.sender, &context.merkle_proof) {
                    // For blacklist, we verify the address is NOT in the list
                    // So if the proof is valid, the address is blacklisted and should be rejected
                    let is_blacklisted = verify_merkle_proof(
                        merkle_proof,
                        *merkle_root,
                        sender.to_bytes(),
                    )?;
                    Ok(!is_blacklisted) // Return true if NOT blacklisted
                } else {
                    // If no proof is provided, we default to allowing the transaction
                    // (assuming it's not in the blacklist)
                    Ok(true)
                }
            },
            
            RuleType::ReceiveMinMax { min, max } => {
                // Check if amount is within allowed range
                if let Some(amount) = context.amount {
                    Ok(amount >= *min && amount <= *max)
                } else {
                    msg!("Missing amount for min/max verification");
                    Ok(false)
                }
            },
            
            RuleType::ReceiveTimeWindow { start, end } => {
                let current_time = Clock::get()?.unix_timestamp;
                Ok(current_time >= *start && current_time <= *end)
            },
            
            RuleType::ReceiveMilestoneEscrow { current_index, steps } => {
                // Check if current escrow step is complete
                Ok(*current_index as usize >= steps.len())
            },
        }
    }
}

// Context passed to rule verification, now including Merkle proof
#[derive(Default)]
pub struct RuleContext<'a> {
    pub sender: Option<&'a Pubkey>,
    pub receiver: Option<&'a Pubkey>,
    pub amount: Option<u64>,
    pub timestamp: Option<i64>,
    pub merkle_proof: Option<&'a MerkleProof>,
}