#![no_main]
sp1_zkvm::entrypoint!(main);
use sp1_verifier::Groth16Verifier;
use types::Sp1Groth16ProofBatch;

pub fn main() {
    let proofs: Sp1Groth16ProofBatch = borsh::from_slice(&sp1_zkvm::io::read_vec()).unwrap();
    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
    for proof in proofs.proofs {
        Groth16Verifier::verify(
            &proof.proof,
            &proof.public_values,
            &proof.vk_hash,
            groth16_vk,
        )
        .expect("Failed to verify proof!");
    }
}
