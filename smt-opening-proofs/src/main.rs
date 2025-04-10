//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]

use types::SmtOpeningBatch;
use valence_smt::MemorySmt;
sp1_zkvm::entrypoint!(main);
pub fn main() {
    let proof_batch: SmtOpeningBatch = borsh::from_slice(&sp1_zkvm::io::read_vec()).unwrap();
    for proof in proof_batch.proofs {
        // assert that the proof is valid for the corresponding root
        assert!(MemorySmt::verify(
            &proof.context,
            &proof.root,
            &borsh::from_slice(&proof.proof_serialized).unwrap()
        ));
    }
}
