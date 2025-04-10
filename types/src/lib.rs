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
pub struct ArkworksGroth16ProofBatch {
    pub proofs: Vec<ArkworksGroth16Proof>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ArkworksGroth16Proof {
    pub g1_affine_points_serialized: Vec<[u8; 48]>,
    pub g2_affine_points_serialized: Vec<[u8; 96]>,
    pub public_inputs_serialized: Vec<Vec<u8>>,
    pub ics_input: Vec<Vec<u8>>,
}

impl ArkworksGroth16Proof {
    pub fn deserialize_public_inputs(&self) -> Vec<BigUint> {
        self.public_inputs_serialized
            .iter()
            .map(|x| BigUint::from_bytes_be(x))
            .collect()
    }
}

pub type MockNode = Vec<u8>;
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MockMerkleProofBatch {
    pub proofs: Vec<Vec<MockNode>>,
}
