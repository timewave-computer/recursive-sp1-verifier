use std::str::FromStr;

// todo: implement bls12_381
#[cfg(all(feature = "bls12_381", not(feature = "bn254")))]
use ark_bls12_381::{self, Config, Fq, Fr, G1Affine, G2Affine};
#[cfg(all(feature = "bls12_381", not(feature = "bn254")))]
pub type G1 = ark_bls12_381::g1::G1Affine;
#[cfg(all(feature = "bls12_381", not(feature = "bn254")))]
use crate::BLS12_381_BASE_FIELD_MODULUS as MODULUS;
#[cfg(feature = "bn254")]
use ark_bn254::{self, Config, Fq, Fr, G1Affine, G2Affine};
#[cfg(all(feature = "bls12_381", not(feature = "bn254")))]
use ark_ec::{bls12::Bls12 as Model, pairing::Pairing, AffineRepr, CurveGroup};
#[cfg(feature = "bn254")]
use ark_ec::{models::bn::Bn as Model, pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField, Zero};
use num_bigint::BigUint;

#[cfg(feature = "bn254")]
use crate::BN254_BASE_FIELD_MODULUS as MODULUS;

#[cfg(feature = "bn254")]
pub type G1 = ark_bn254::g1::G1Affine;

pub fn parse_biguint_to_fq(value: &str) -> Fq {
    let big_int = BigUint::parse_bytes(value.as_bytes(), 10).unwrap();
    Fq::from(big_int)
}

fn fq_to_biguint(x_fq: &Fq) -> BigUint {
    let bigint_repr = x_fq.into_bigint();
    BigUint::from_bytes_le(&bigint_repr.to_bytes_le())
}

pub fn negate_g1_affine(p: G1Affine) -> G1Affine {
    let x_fq = p.x().unwrap();
    let y_coord: BigUint = fq_to_biguint(&p.y().unwrap());
    let base_field_modulus_biguint = BigUint::from_str(MODULUS).unwrap();
    if y_coord == BigUint::ZERO && fq_to_biguint(&x_fq) == BigUint::ZERO {
        G1Affine::new_unchecked(parse_biguint_to_fq("0"), parse_biguint_to_fq("0"))
    } else {
        let neg_y_coord =
            (base_field_modulus_biguint.clone() - y_coord) % base_field_modulus_biguint;
        G1Affine::new_unchecked(x_fq, Fq::from(neg_y_coord))
    }
}

pub fn verify_groth16_proof(
    pi_a: G1Affine,
    pi_b: G2Affine,
    pi_c: G1Affine,
    vk_alpha1: G1Affine,
    vk_beta2: G2Affine,
    vk_gamma2: G2Affine,
    vk_delta2: G2Affine,
    ics: Vec<G1Affine>,
    inputs: Vec<BigUint>,
) -> bool {
    let mut vk_x: G1 = ics[0];
    for (idx, ic) in ics.into_iter().enumerate().skip(1) {
        let ic_coords = extract_g1_coordinates(ic);
        let ic_scalar: G1 = scalar_mul(ic_coords.0, ic_coords.1, inputs[idx - 1].clone());
        println!("processed input: {}, with ic: {}", inputs[idx - 1], idx);
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
        (vk_alpha1, vk_beta2),
        (vk_x, vk_gamma2),
        (pi_c, vk_delta2),
    ];
    // compute pairing result and return is_zero?
    <Model<Config> as Pairing>::multi_pairing(
        vec![negate_g1_affine(pi_a), vk_alpha1, vk_x, pi_c],
        vec![pi_b, vk_beta2, vk_gamma2, vk_delta2],
    )
    .is_zero()
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

fn scalar_mul(p_x: BigUint, p_y: BigUint, k: BigUint) -> G1 {
    let p = G1::new_unchecked(
        parse_biguint_to_fq(&p_x.to_string()),
        parse_biguint_to_fq(&p_y.to_string()),
    );
    let scalar = Fr::from_be_bytes_mod_order(&k.to_bytes_be());
    (p.into_group() * scalar).into_affine()
}
