use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Sp1ProofGroth16 {
    pub proof_serialized: Vec<u8>,
    pub public_values_serialized: Vec<u8>,
    pub vk_hash: String,
    pub groth16_vk: Vec<u8>,
}
