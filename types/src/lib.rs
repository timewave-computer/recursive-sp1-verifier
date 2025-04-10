use borsh::{BorshDeserialize, BorshSerialize};
use num_bigint::BigUint;

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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ArkworksGroth16Proof {}
