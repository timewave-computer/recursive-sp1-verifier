//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use sp1_verifier::Groth16Verifier;

pub fn main() {
    let proof = sp1_zkvm::io::read_vec();
    let sp1_public_values = sp1_zkvm::io::read_vec();
    let sp1_vkey_hash: String = sp1_zkvm::io::read();
    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;

    Groth16Verifier::verify(&proof, &sp1_public_values, &sp1_vkey_hash, groth16_vk)
        .expect("Proof verification failed");
    println!("Proof verified");

    sp1_zkvm::io::commit_slice(&[1]);
}
