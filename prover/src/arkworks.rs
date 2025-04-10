use ark_crypto_primitives::{
    crh::poseidon::{constraints::CRHGadget, CRH},
    crh::CRHSchemeGadget,
    sponge::poseidon::PoseidonConfig,
};

use ark_ed_on_bls12_381::Fq as FqBLS12_381;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct PoseidonDemoCircuitBls12_381 {
    pub input: Option<FqBLS12_381>, // private input
    pub params: PoseidonConfig<FqBLS12_381>,
    pub expected_output: Option<FqBLS12_381>, // public input
}

impl ConstraintSynthesizer<FqBLS12_381> for PoseidonDemoCircuitBls12_381 {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<FqBLS12_381>,
    ) -> Result<(), SynthesisError> {
        // Allocate the input variable (private witness)
        let input_var = FpVar::new_witness(cs.clone(), || {
            self.input.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the Poseidon parameters as constants
        let params_var = <CRHGadget<FqBLS12_381> as CRHSchemeGadget<
            CRH<FqBLS12_381>,
            FqBLS12_381,
        >>::ParametersVar::new_constant(cs.clone(), &self.params)?;

        // Compute the hash in the circuit 100 times
        let mut current_hash = CRHGadget::<FqBLS12_381>::evaluate(&params_var, &[input_var])?;

        // Hash 99 more times (total of 100)
        for _ in 1..100 {
            current_hash = CRHGadget::<FqBLS12_381>::evaluate(&params_var, &[current_hash])?;
        }

        // Allocate the expected output as a public input
        let expected_output_var = FpVar::new_input(cs, || {
            self.expected_output
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce that the computed hash equals the expected output
        current_hash.enforce_equal(&expected_output_var)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;
    use ark_crypto_primitives::crh::{poseidon::CRH, CRHScheme};
    use ark_ec::scalar_mul;
    use ark_ed_on_bls12_381::Fq as FqBLS12_381;
    use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16};
    use ark_serialize::CanonicalSerialize;
    use ark_snark::SNARK;
    use ark_std::{rand::rngs::StdRng, rand::SeedableRng};
    use jonas_groth16::verifier::add_g1_as_coordinates;
    use jonas_groth16::verifier::extract_g1_coordinates;
    use jonas_groth16::verifier::negate_g1_affine;
    use jonas_groth16::verifier::scalar_mul;
    use jonas_groth16::verifier::verify_groth16_proof;

    #[test]
    fn test_arkworks_poseidon_groth16_bls12_381() {
        let rng = &mut StdRng::seed_from_u64(0u64);

        // Create Poseidon parameters with correct dimensions
        let width = 3;
        let full_rounds = 8;
        let partial_rounds = 56;

        // Create MDS matrix of size width x width
        let mds = vec![
            vec![FqBLS12_381::from(1); width],
            vec![FqBLS12_381::from(2); width],
            vec![FqBLS12_381::from(3); width],
        ];

        // Create ARK matrix of size (full_rounds + partial_rounds) x width
        let ark = vec![vec![FqBLS12_381::from(1); width]; full_rounds + partial_rounds];

        let poseidon_params = PoseidonConfig {
            full_rounds,
            partial_rounds,
            alpha: 5,
            rate: 2,
            capacity: 1,
            mds,
            ark,
        };

        // Compute the hash of the input 100 times
        let input_value = FqBLS12_381::from(42u32);
        let mut current_hash = input_value;

        // Hash 100 times
        for _ in 0..100 {
            let input_vec = vec![current_hash];
            current_hash =
                CRH::<FqBLS12_381>::evaluate(&poseidon_params, input_vec.as_slice()).unwrap();
        }

        let final_hash = current_hash;

        // Create a circuit with a dummy witness for setup
        let setup_circuit = PoseidonDemoCircuitBls12_381 {
            input: Some(FqBLS12_381::from(0)), // Use a dummy value for setup
            params: poseidon_params.clone(),
            expected_output: Some(FqBLS12_381::from(0)), // Use a dummy value for setup
        };

        // Generate the proving and verification keys
        let (pk, vk) =
            Groth16::<ark_bls12_381::Bls12_381, LibsnarkReduction>::circuit_specific_setup(
                setup_circuit,
                rng,
            )
            .unwrap();

        // Create a circuit with the actual witness for proving
        let proof_circuit = PoseidonDemoCircuitBls12_381 {
            input: Some(input_value), // Use the actual input value
            params: poseidon_params.clone(),
            expected_output: Some(final_hash), // Use the actual hash value
        };

        // Generate the proof
        let proof =
            Groth16::<ark_bls12_381::Bls12_381, LibsnarkReduction>::prove(&pk, proof_circuit, rng)
                .unwrap();

        // The public input is the hash value
        let public_inputs = vec![final_hash];

        // Verify the proof
        let is_valid = Groth16::<ark_bls12_381::Bls12_381, LibsnarkReduction>::verify(
            &vk,
            &public_inputs,
            &proof,
        )
        .unwrap();
        assert!(is_valid);

        let pi_a = proof.a;
        let pi_b = proof.b;
        let pi_c = proof.c;
        let vk_alpha = vk.alpha_g1;
        let vk_beta = vk.beta_g2;
        let vk_gamma = vk.gamma_g2;
        let vk_delta = vk.delta_g2;
        let ics = vk.gamma_abc_g1;

        let inputs_as_biguint = convert_381_public_inputs_to_biguint(&public_inputs);
        let mut ics_serialized: Vec<Vec<u8>> = vec![];
        for point in ics.clone() {
            let mut buffer = vec![];
            point.serialize_compressed(&mut buffer).unwrap();
            ics_serialized.push(buffer);
        }

        let mut vk_x: G1 = ics[0];
        for (idx, ic) in ics.into_iter().enumerate().skip(1) {
            let ic_coords = extract_g1_coordinates(ic);
            let ic_scalar: G1 =
                scalar_mul(ic_coords.0, ic_coords.1, inputs_as_biguint[idx - 1].clone());
            let ic_scalar_coords = extract_g1_coordinates(ic_scalar);
            let vk_x_as_coordinates = extract_g1_coordinates(vk_x);
            let vk_x_as_coords = vk_x_as_coordinates.clone();
            vk_x = add_g1_as_coordinates(
                vk_x_as_coords.0,
                vk_x_as_coords.1,
                ic_scalar_coords.0,
                ic_scalar_coords.1,
            );
        }

        let terms = vec![
            (negate_g1_affine(pi_a), pi_b),
            (vk_alpha, vk_beta),
            (vk_x, vk_gamma),
            (pi_c, vk_delta),
        ];
        let first_point = terms.first().unwrap().0;
        let mut point_compressed = vec![];
        first_point
            .serialize_compressed(&mut point_compressed)
            .unwrap();

        let mut g1_affine_points_serialized: Vec<Vec<u8>> = vec![];
        let mut g2_affine_points_serialized: Vec<Vec<u8>> = vec![];

        for point in terms {
            let mut point_compressed = vec![];
            point.0.serialize_compressed(&mut point_compressed).unwrap();
            let point_as_ref: [u8; 48] = point_compressed.try_into().unwrap();
            g1_affine_points_serialized.push(point_as_ref.to_vec());

            let mut point_compressed = vec![];
            point.1.serialize_compressed(&mut point_compressed).unwrap();
            let point_as_ref: [u8; 96] = point_compressed.try_into().unwrap();
            g2_affine_points_serialized.push(point_as_ref.to_vec());
        }

        let is_valid = verify_groth16_proof(
            g1_affine_points_serialized.clone(),
            g2_affine_points_serialized.clone(),
            inputs_as_biguint.clone(),
            ics_serialized.clone(),
        );
        assert!(is_valid);

        let example_proof_payload = ArkworksGroth16Proof {
            g1_affine_points_serialized,
            g2_affine_points_serialized,
            public_inputs_serialized: inputs_as_biguint.iter().map(|x| x.to_bytes_be()).collect(),
            ics_input: ics_serialized,
        };

        let crate_root = env!("CARGO_MANIFEST_DIR");
        let output_path = Path::new(crate_root).join("src/test_data/proof.bin");

        fs::write(output_path, borsh::to_vec(&example_proof_payload).unwrap()).unwrap();
    }

    use ark_ff::BigInteger;
    use ark_ff::PrimeField;
    use jonas_groth16::verifier::G1;
    use num_bigint::BigUint;
    use types::ArkworksGroth16Proof;

    fn fq381_to_biguint(f: &FqBLS12_381) -> BigUint {
        let ark_bigint = f.into_bigint();
        let bytes = ark_bigint.to_bytes_be();
        BigUint::from_bytes_be(&bytes)
    }

    fn convert_381_public_inputs_to_biguint(inputs: &[FqBLS12_381]) -> Vec<BigUint> {
        inputs.iter().map(fq381_to_biguint).collect()
    }
}
