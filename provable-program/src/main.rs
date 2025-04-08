//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};

pub fn main() {
    let input = sp1_zkvm::io::read::<String>();
    let mut output = borsh::to_vec(&input).unwrap();
    for _ in 0..9 {
        output = compute_digest(&borsh::to_vec(&output).unwrap());
    }
    sp1_zkvm::io::commit_slice(&output);
}

fn compute_digest(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}
