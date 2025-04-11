use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SmtOpeningInput {
    pub proof_serialized: Vec<u8>,
    pub root: [u8; 32],
    pub context: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SmtOpeningBatch {
    pub proofs: Vec<SmtOpeningInput>,
}
