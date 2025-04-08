use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Sp1Groth16Proof {
    pub proof: Vec<u8>,
    pub public_values: Vec<u8>,
    pub vk_hash: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Sp1Groth16ProofBatch {
    pub proofs: Vec<Sp1Groth16Proof>,
}
