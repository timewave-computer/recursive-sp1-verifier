#![no_main]
sp1_zkvm::entrypoint!(main);
use jonas_groth16::verifier::verify_groth16_proof;
use types::ArkworksGroth16ProofBatch;
pub fn main() {
    let proofs: ArkworksGroth16ProofBatch = borsh::from_slice(&sp1_zkvm::io::read_vec()).unwrap();
    for proof in proofs.proofs {
        let public_inputs = proof.deserialize_public_inputs();
        let result = verify_groth16_proof(
            proof.g1_affine_points_serialized,
            proof.g2_affine_points_serialized,
            public_inputs,
            proof.ics_input,
        );
        assert!(result);
    }
}
