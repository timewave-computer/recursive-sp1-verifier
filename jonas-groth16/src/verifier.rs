use crate::BLS12_381_BASE_FIELD_MODULUS as MODULUS;
pub use ark_bls12_381::G1Affine as G1;
use ark_bls12_381::{self, Fq, Fr};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
#[cfg(feature = "normal")]
use normal_bls::{multi_miller_loop, G1Affine, G2Affine, G2Prepared, Gt};
use num_bigint::BigUint;
#[cfg(all(feature = "sp1", not(feature = "normal")))]
use sp1_bls_precompile::{multi_miller_loop, G1Affine, G2Affine, G2Prepared, Gt};
use std::str::FromStr;

#[allow(clippy::too_many_arguments)]
pub fn verify_groth16_proof(
    g1_affine_points_serialized: Vec<[u8; 48]>,
    g2_affine_points_serialized: Vec<[u8; 96]>,
    public_inputs: Vec<BigUint>,
    ics_serialized: Vec<Vec<u8>>,
) -> bool {
    let mut g1_affine_points: Vec<G1Affine> = vec![];
    let mut g2_affine_points: Vec<G2Prepared> = vec![];

    for point in g1_affine_points_serialized {
        g1_affine_points.push(G1Affine::from_compressed_unchecked(&point).unwrap());
    }

    for point in g2_affine_points_serialized {
        g2_affine_points.push(G2Prepared::from(
            G2Affine::from_compressed_unchecked(&point).unwrap(),
        ));
    }

    let mut ics: Vec<G1> = vec![];
    for point in ics_serialized {
        ics.push(G1::deserialize_compressed_unchecked::<&[u8]>(&point).unwrap());
    }
    let mut vk_x: G1 = ics[0];
    for (idx, ic) in ics.into_iter().enumerate().skip(1) {
        let ic_coords = extract_g1_coordinates(ic);
        let ic_scalar: G1 = scalar_mul(ic_coords.0, ic_coords.1, public_inputs[idx - 1].clone());
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
    let mut vk_x_buffer = vec![];
    vk_x.serialize_compressed(&mut vk_x_buffer).unwrap();
    assert_eq!(
        G1Affine::from_compressed_unchecked(&vk_x_buffer.try_into().unwrap()).unwrap(),
        *g1_affine_points.get(2).unwrap()
    );

    let pairing_inputs: Vec<_> = g1_affine_points
        .iter()
        .zip(g2_affine_points.iter())
        .collect();

    let miller_result = multi_miller_loop(&pairing_inputs);
    miller_result.final_exponentiation() == Gt::identity()

    // todo: constrain vk_x => this is important to verify the public inputs
    // might have to fork the bls precompile for that, work in progress.
}

pub fn add_g1_as_coordinates(p_x: BigUint, p_y: BigUint, q_x: BigUint, q_y: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let q = G1::new_unchecked(
        parse_biguint_to_fq(&q_x.to_string()),
        parse_biguint_to_fq(&q_y.to_string()),
    );
    (p + q).into_affine()
}

pub fn extract_g1_coordinates(p: G1) -> (BigUint, BigUint) {
    (
        fq_to_biguint(&p.x().unwrap()),
        fq_to_biguint(&p.y().unwrap()),
    )
}

pub fn scalar_mul(p_x: BigUint, p_y: BigUint, k: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let scalar = Fr::from_be_bytes_mod_order(&k.to_bytes_be());
    (p.into_group() * scalar).into_affine()
}

pub fn parse_biguint_to_fq(value: &str) -> Fq {
    let big_int = BigUint::parse_bytes(value.as_bytes(), 10).unwrap();
    Fq::from(big_int)
}

fn fq_to_biguint(x_fq: &Fq) -> BigUint {
    let bigint_repr = x_fq.into_bigint();
    BigUint::from_bytes_le(&bigint_repr.to_bytes_le())
}

pub fn negate_g1_affine(p: G1) -> G1 {
    let x_fq = p.x().unwrap();
    let y_coord: BigUint = fq_to_biguint(&p.y().unwrap());
    let base_field_modulus_biguint = BigUint::from_str(MODULUS).unwrap();
    if y_coord == BigUint::ZERO && fq_to_biguint(&x_fq) == BigUint::ZERO {
        G1::new_unchecked(Fq::from(0), Fq::from(0))
    } else {
        let neg_y_coord =
            (base_field_modulus_biguint.clone() - y_coord) % base_field_modulus_biguint;
        G1::new_unchecked(x_fq, Fq::from(neg_y_coord))
    }
}
