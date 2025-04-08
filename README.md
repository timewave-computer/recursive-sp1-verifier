# Recursive SP1 Circuit

>[!NOTE]
> Work in progress.
> This MIGHT become an MVP implementation for merkle openings
> in the Valence coprocessor Tree.

## Overview

This project explores the implementation of a recursive (wrapped) Groth16 circuit using SP1's SHA2 precompile and Succinct's Gnark verifier. It's being developed as part of the investigation into different proof systems for the Valence coprocessor, with a focus on developer experience and practical implementation.

## Features

- SP1 SHA2 precompile integration
- Succinct Gnark verifier implementation
- Recursive Groth16 circuit wrapping
- Basic benchmarking capabilities


## Issues and Performance
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

## Install any version of SP1 toolchain and prover utils
```shell
$ sp1up --version 4.1.7
```

### Prerequisites

- Rust toolchain (latest stable version recommended)
- SP1 dependencies
- Succinct Gnark dependencies

### Building and Running

To build and run the prover:

```bash
RUST_LOG=info cargo run --release
```

This will perform the following steps:

1. Generate an SP1 proof of a hash
2. Wrap that proof as a Groth16 proof
3. Verify that Groth16 proof inside an SP1 circuit (recursive circuit) to generate a new SP1 proof
4. Wrap that new SP1 proof as Groth16
5. Verify that new Groth16 proof outside the circuit (this would usually happen on-chain)
