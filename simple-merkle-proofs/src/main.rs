#![no_main]
sp1_zkvm::entrypoint!(main);
use sha2::{Digest, Sha256};
use types::MockMerkleProofBatch;
pub fn main() {
    let proof_batch: MockMerkleProofBatch = borsh::from_slice(&sp1_zkvm::io::read_vec()).unwrap();
    for proof in proof_batch.proofs {
        let mut current_hash = proof[0].clone();

        for sibling in &proof[1..] {
            let mut hasher = Sha256::new();

            let mut combined = current_hash.clone();
            combined.extend_from_slice(sibling);

            hasher.update(&combined);
            current_hash = hasher.finalize().to_vec();
        }
    }
}
