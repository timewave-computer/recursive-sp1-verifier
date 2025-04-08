//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```

use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use sp1_verifier::Groth16Verifier;
use types::Sp1ProofGroth16;
pub const PROVABLE_ELF: &[u8] = include_elf!("provable-program");
pub const RECURSIVE_ELF: &[u8] = include_elf!("recursive-program");

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

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

    let proof_serialized = proof.bytes();
    let public_values_serialized = proof.public_values.to_vec();
    let vk_hash = vk.bytes32();
    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;
    println!("Done proving provable_program, vk_hash: {:?}", vk_hash);

    // verify a groth16 proof inside the circuit
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    let inputs = Sp1ProofGroth16 {
        proof_serialized,
        public_values_serialized,
        vk_hash,
        groth16_vk: groth16_vk.to_vec(),
    };
    println!("Trying to write inputs");
    stdin.write_vec(borsh::to_vec(&inputs).unwrap());
    println!("Inputs written");
    let (pk, vk) = client.setup(RECURSIVE_ELF);
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("failed to generate recursive proof");

    // generate final groth16 proof
    Groth16Verifier::verify(
        &proof.bytes(),
        &proof.public_values.to_vec(),
        &vk.bytes32(),
        groth16_vk,
    )
    .unwrap();
}
