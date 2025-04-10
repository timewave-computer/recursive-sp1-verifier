# Recursive ZK Circuits

> This project is part of a larger investigation into proof systems for the Valence coprocessor. For detailed background information and motivation, please see our [Context Document](context.md) 📚
>
> Work in progress.
> This MIGHT become an MVP implementation for merkle openings
> in the Valence coprocessor Tree.

[![Learn More About Valence ZK](https://img.shields.io/badge/_Learn_More_About_Valence_ZK-2EA44F?style=for-the-badge&logo=github&logoColor=white)](context.md)

# 1. Recursive SP1 Circuit

## 1.1 Overview
[Click to see Context](context.md)

This project explores the implementation of a recursive (wrapped) Groth16 circuit using SP1's SHA2 precompile and Succinct's Gnark verifier. It's being developed as part of the investigation into different proof systems for the Valence coprocessor, with a focus on developer experience and practical implementation.

## 1.2 Features

- SP1 SHA2 precompile integration
- Succinct Gnark verifier implementation
- Recursive Groth16 circuit wrapping
- Basic benchmarking capabilities


## 1.3 Issues and Performance

We had to temporarily downgrade the SP1 prover to 3.x (from 4.1.7).
The reason for this is that the recursive verification failed with an ambiguous 
error related to precompiles. I assume the reason for this is that the recursive 
circuit hasn't been tested / updated since 4.x.

A fix is on the way, in the meantime this command will lock the dependency:

```shell
$ cargo update substrate-bn-succinct --precise 0.6.0-v4.1.4
```

>[!NOTE]
> We want to migrate to 4.x asap because it offers major performance benefits

## 1.4 Prerequisites

- Rust toolchain (latest stable version recommended)
- SP1 dependencies
- Succinct Gnark dependencies

```shell
$ sp1up --version 4.1.7
```


## 1.5 Basic Benchmark Results
### Macbook Pro M3 Max, 64 GB Ram

1. Recursive circuit as described in `5. Building and Running`

| Recursive Proofs | SHA2 Hashes | Time taken | Test Name | 
|---|---|---|---|
| 1 | 1 | 799.48s | None |
| 1 | 10 | 823.10s | test_wrapper_merkle_proof |
| 10 | 10 |  | test_wrapper_merkle_proof_batch |

## 5. Run the Basic Benchmarks Yourself

Single merkle proof with 10 hashes:

```shell
$ RUST_LOG=info cargo test test_wrapper_merkle_proof_single --release -- --nocapture
```

Batch of 10 merkle proofs with 10 hashes each:

```shell
$ RUST_LOG=info cargo test test_wrapper_merkle_proof_batch --release -- --nocapture
```

This will perform the following steps:

1. Generate an SP1 proof of a hash
2. Wrap that proof as a Groth16 proof
3. Verify that Groth16 proof inside an SP1 circuit (recursive circuit) to generate a new SP1 proof
4. Wrap that new SP1 proof as Groth16
5. Verify that new Groth16 proof outside the circuit (this would usually happen on-chain)


# 2. Arkworks 
In order to accelerate the co-processor opening proof speed, we're working on a custom Groth16 verifier
that can leverage the `bls12_381` precompile to verify Arkworks proofs. The goal is to generate a lot of
merkle opening ZKPs quickly and then wrap them in a single SP1 circuit. We still have to do the heavy lifting 
in the end, but only once for a batch of arbitrary size.


## Run the benchmark
```shell
cargo test test_arkworks_groth16_proof_batch --release --features sp1 -- --nocapture
```
This test will benchmark a batch of recursive arkworks proofs.

The main advantage over using SP1 is that we can quickly generate the groth16 proofs for the openings,
without having to wrap every single proof with SP1. 

We only wrap the final recursive verification in SP1 => this is the most expensive step that is applied
to a batch of opening ZKPs.
