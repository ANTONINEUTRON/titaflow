use anchor_lang::prelude::*;

#[error_code]
pub enum UtilsError {
    #[msg("Rule is not satisfied")]
    InvalidMerkleProof
}