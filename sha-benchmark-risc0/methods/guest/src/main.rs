use risc0_zkvm::guest::env;
use std::io::Read;
use types::SmtOpeningBatch;
use valence_smt::MemorySmt;
fn main() {
    let mut buffer: Vec<u8> = vec![];
    let _ = env::stdin().read_to_end(&mut buffer);
    let proof_batch: SmtOpeningBatch = borsh::from_slice(&buffer).unwrap();
    for proof in proof_batch.proofs {
        // assert that the proof is valid for the corresponding root
        assert!(MemorySmt::verify(
            &proof.context,
            &proof.root,
            &borsh::from_slice(&proof.proof_serialized).unwrap()
        ));
    }
}
