//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```

use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};
use sp1_verifier::Groth16Verifier;
pub const PROVABLE_ELF: &[u8] = include_elf!("provable-program");
pub const RECURSIVE_ELF: &[u8] = include_elf!("recursive-program");

fn prove_provable_program() -> (Vec<u8>, String, Vec<u8>) {
    // generate a groth16 proof
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&"Hello, Prover!");
    let (pk, vk) = client.setup(PROVABLE_ELF);
    let proof = client
        .prove(&pk, stdin)
        .groth16()
        .run()
        .expect("failed to generate provable_program proof");

    let vk_hash = vk.bytes32();
    (proof.bytes(), vk_hash, proof.public_values.to_vec())
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let (proof, vk_hash, public_values) = prove_provable_program();

    // verify a groth16 proof inside the circuit
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(proof);
    stdin.write_vec(public_values);
    stdin.write(&vk_hash);

    let (pk, vk) = client.setup(RECURSIVE_ELF);
    let proof = client
        .prove(&pk, stdin)
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
}
