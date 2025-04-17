use anchor_lang::prelude::*;

use super::UtilsError;

/// A Merkle proof consisting of the complementary hashes needed to reconstruct the root
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MerkleProof {
    #[max_len(32)]  // Maximum depth of the tree, likely will be much smaller
    pub hashes: Vec<[u8; 32]>,
    #[max_len(32)]  // Matches the length of hashes
    pub indices: Vec<u8>,  // Position (left/right) for each hash
}

/// Verifies that a given value is part of a Merkle tree with the provided root
pub fn verify_merkle_proof(
    proof: &MerkleProof,
    root: [u8; 32], 
    value: [u8; 32]
) -> Result<bool> {
    if proof.hashes.len() != proof.indices.len() {
        return err!(UtilsError::InvalidMerkleProof);
    }

    let mut current_hash = value;
    
    // Traverse up the tree using the proof
    for i in 0..proof.hashes.len() {
        let proof_element = proof.hashes[i];
        let index = proof.indices[i];
        
        // Hash in the correct order based on index (0 = left, 1 = right)
        current_hash = match index {
            0 => hash_pair(&current_hash, &proof_element),
            1 => hash_pair(&proof_element, &current_hash),
            _ => return err!(UtilsError::InvalidMerkleProof),
        };
    }
    
    // Check if we've arrived at the root
    Ok(current_hash == root)
}


/// Hash two nodes together to produce a parent node
fn hash_pair(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    use anchor_lang::solana_program::keccak::hashv;
    
    // Use Solana's built-in keccak hash function instead of requiring sha2
    let hash = hashv(&[a, b]);
    let mut output = [0u8; 32];
    output.copy_from_slice(&hash.to_bytes());
    output
}