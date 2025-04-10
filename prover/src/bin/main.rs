//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```
use std::time::Instant;

use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use sp1_verifier::Groth16Verifier;
use types::{Sp1Groth16Proof, Sp1Groth16ProofBatch};
pub const PROVABLE_ELF: &[u8] = include_elf!("provable-program");
pub const RECURSIVE_ELF: &[u8] = include_elf!("recursive-program");
pub const RECURSIVE_ARKWORKS_ELF: &[u8] = include_elf!("recursive-arkworks-program");
#[allow(unused)]
fn prove_provable_program() -> (Vec<u8>, String, Vec<u8>) {
    // generate a groth16 proof
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&"Hello, Prover!");
    let (pk, vk) = client.setup(PROVABLE_ELF);
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("failed to generate provable_program proof");

    let vk_hash = vk.bytes32();
    (proof.bytes(), vk_hash, proof.public_values.to_vec())
}

fn main() {
    let start_time = Instant::now();
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();
    let (proof, vk_hash, public_values) = prove_provable_program();

    // verify a groth16 proof inside the circuit
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    let inputs = Sp1Groth16ProofBatch {
        proofs: vec![Sp1Groth16Proof {
            proof,
            vk_hash,
            public_values,
        }],
    };
    stdin.write_vec(borsh::to_vec(&inputs).unwrap());

    let (pk, vk) = client.setup(RECURSIVE_ELF);
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("failed to generate recursive proof");

    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
    // verify final groth16 proof
    Groth16Verifier::verify(
        &proof.bytes(),
        &proof.public_values.to_vec(),
        &vk.bytes32(),
        groth16_vk,
    )
    .unwrap();
    let end_time = Instant::now() - start_time;
    println!("Time taken: {:?}", end_time);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp1_verifier::Groth16Verifier;
    use std::{fs, path::Path, time::Instant};
    use types::{
        ArkworksGroth16Proof, ArkworksGroth16ProofBatch, Sp1Groth16Proof, Sp1Groth16ProofBatch,
    };

    #[test]
    fn test_wrapper_merkle_proof_single() {
        let start_time = Instant::now();
        sp1_sdk::utils::setup_logger();
        dotenv::dotenv().ok();
        let (proof, vk_hash, public_values) = prove_provable_program();

        // verify a groth16 proof inside the circuit
        let client = ProverClient::new();
        let mut stdin = SP1Stdin::new();
        let inputs = Sp1Groth16ProofBatch {
            proofs: vec![Sp1Groth16Proof {
                proof,
                vk_hash,
                public_values,
            }],
        };
        stdin.write_vec(borsh::to_vec(&inputs).unwrap());

        let (pk, vk) = client.setup(RECURSIVE_ELF);
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate recursive proof");

        let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
        // verify final groth16 proof
        Groth16Verifier::verify(
            &proof.bytes(),
            &proof.public_values.to_vec(),
            &vk.bytes32(),
            groth16_vk,
        )
        .unwrap();
        let end_time = Instant::now() - start_time;
        println!("Time taken: {:?}", end_time);
    }

    #[test]
    fn test_wrapper_merkle_proof_batch() {
        let start_time = Instant::now();
        sp1_sdk::utils::setup_logger();
        let client = ProverClient::new();
        let mut stdin = SP1Stdin::new();
        let mut inputs: Sp1Groth16ProofBatch = Sp1Groth16ProofBatch { proofs: Vec::new() };
        let (proof, vk_hash, public_values) = prove_provable_program();
        for _ in 0..9 {
            // generate the same opening groth16 proof 10 times
            inputs.proofs.push(Sp1Groth16Proof {
                proof: proof.clone(),
                vk_hash: vk_hash.clone(),
                public_values: public_values.clone(),
            });
        }
        let (pk, vk) = client.setup(RECURSIVE_ELF);
        stdin.write_vec(borsh::to_vec(&inputs).unwrap());
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate recursive proof");

        let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
        // verify final groth16 proof
        Groth16Verifier::verify(
            &proof.bytes(),
            &proof.public_values.to_vec(),
            &vk.bytes32(),
            groth16_vk,
        )
        .unwrap();
        let end_time = Instant::now() - start_time;
        println!("Time taken: {:?}", end_time);
    }

    #[test]
    fn test_arkworks_groth16_proof_batch() {
        let start_time = Instant::now();
        let crate_root = env!("CARGO_MANIFEST_DIR");
        let output_path = Path::new(crate_root).join("src/test_data/proof.bin");
        let mut stdin = SP1Stdin::new();
        let proof_serialized = fs::read(output_path).unwrap();

        let mut proof_batch: ArkworksGroth16ProofBatch =
            ArkworksGroth16ProofBatch { proofs: vec![] };
        // recursively verify 10 proofs of 100 hasehs each
        for _ in 0..1 {
            let proof: ArkworksGroth16Proof = borsh::from_slice(&proof_serialized).unwrap();
            proof_batch.proofs.push(proof);
        }
        let circuit_input = borsh::to_vec(&proof_batch).unwrap();
        stdin.write_vec(circuit_input);
        let client = ProverClient::new();
        let (pk, vk) = client.setup(RECURSIVE_ARKWORKS_ELF);
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate recursive proof");
        let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
        Groth16Verifier::verify(
            &proof.bytes(),
            &proof.public_values.to_vec(),
            &vk.bytes32(),
            groth16_vk,
        )
        .unwrap();
        let end_time = Instant::now() - start_time;
        println!("Time taken: {:?}", end_time);
    }
}
