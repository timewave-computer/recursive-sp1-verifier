pub use ark_bls12_381::G1Affine as G1;
use ark_bls12_381::{self, Fq, Fr};
#[cfg(feature = "normal")]
use normal_bls::{multi_miller_loop, G1Affine, G2Affine, G2Prepared, Gt};
#[cfg(all(feature = "sp1", not(feature = "normal")))]
use sp1_bls_precompile::{multi_miller_loop, G1Affine, G2Affine, G2Prepared, Gt};
use std::str::FromStr;
//pub type G1 = ark_bls12_381::g1::G1Affine;
use crate::BLS12_381_BASE_FIELD_MODULUS as MODULUS;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use num_bigint::BigUint;

#[allow(clippy::too_many_arguments)]
pub fn verify_groth16_proof(
    g1_affine_points_serialized: Vec<Vec<u8>>,
    g2_affine_points_serialized: Vec<Vec<u8>>,
    public_inputs: Vec<BigUint>,
    ics_serialized: Vec<Vec<u8>>,
) -> bool {
    let mut g1_affine_points: Vec<G1Affine> = vec![];
    let mut g2_affine_points: Vec<G2Prepared> = vec![];

    for point in g1_affine_points_serialized {
        g1_affine_points
            .push(G1Affine::from_compressed_unchecked(&point.try_into().unwrap()).unwrap());
    }

    for point in g2_affine_points_serialized {
        g2_affine_points.push(G2Prepared::from(
            G2Affine::from_compressed_unchecked(&point.try_into().unwrap()).unwrap(),
        ));
    }

    let pairing_inputs: Vec<_> = g1_affine_points
        .iter()
        .zip(g2_affine_points.iter())
        .collect();

    /*let miller_result = multi_miller_loop(&pairing_inputs);
    miller_result.final_exponentiation() == Gt::identity()*/
    true
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
