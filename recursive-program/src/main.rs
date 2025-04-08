//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use sp1_verifier::Groth16Verifier;
use types::Sp1ProofGroth16;

pub fn main() {
    let proof: Sp1ProofGroth16 = borsh::from_slice(&sp1_zkvm::io::read_vec()).unwrap();
    Groth16Verifier::verify(
        &proof.proof_serialized,
        &proof.public_values_serialized,
        &proof.vk_hash,
        &proof.groth16_vk,
    )
    .unwrap();
    // commit "true"
    sp1_zkvm::io::commit_slice(&[1]);
}
