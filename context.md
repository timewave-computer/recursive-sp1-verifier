# Zero Knowledge Coprocessor for Cross-Chain Merkle Proofs

> **Proposal**: This document outlines a detailed and suggested architecture and development strategy for building a zero knowledge coprocessor capable of verifying cross-chain Merkle proofs in a recursive and efficient manner.  
> It is intended to guide the team — not prescribe immutable rules — and may evolve as implementation proceeds.

---

## 📦 System Purpose

We are building a **zero knowledge coprocessor** designed to batch and verify **cross-chain Merkle proofs** for account, storage, and receipt data. This enables generalized proof-based state access across domains.

---

## 🧱 Core System Components

### 1. Merkle Proof Library

A generic interface and implementation for Merkle proofs across supported chains.

- ✅ Focus for MVP:
  - **Account proofs**
  - **Storage proofs**
- 🕓 Possible extension (post-MVP):
  - **Receipt proofs**, specifically:
    - Ethereum `ERC20 Transfer` logs
    - L2 chains like Optimism and Arbitrum

This library emits standardized Merkle proofs to be consumed by the coprocessor.

### 2. The Coprocessor

The coprocessor receives Merkle proofs and performs the following:

- **Batching**:
  - Groups Merkle proofs by target domain and batch interval
- **Recursive ZK Verification** (✳️ see note below):
  - Verifies the entire batch using recursive proof systems
  - Produces a single ZKP (e.g., Groth16) for the batch

✳️ **We require two distinct recursive proofs**:
  1. **Domain State Proofs** – For verifying individual domain-level Merkle proofs (e.g., account/storage proofs from Ethereum, Cosmos)
  2. **SMT Update Proofs** – For verifying updates to the Sparse Merkle Tree that tracks latest trusted roots for each domain

- **Trusted Root Verification**:
  - Maintains an on-chain **Sparse Merkle Tree (SMT)** that stores the *latest finalized* trusted state root for each domain
  - This SMT is updated via light clients or proof-based SMT updates

For the MVP we will want to settle with a prover that is reasonably fast, but not necessarily perfectly optimized.
Recursive Groth16 is a real challenge that we want to solve. As an initial solution, this repository contains
a recursive Arkworks verifier in SP1, enabling fast opening proofs but still demanding an expensive batching
operation. We verify all Groth16 proofs that were generated using the Arkworks opening circuit inside a single 
SP1 program, effectively batching them into a single Groth16 proof that can be verified on-chain.
We are actively looking for optimizations and ways to improve this proving scheme.

The SMT allows:
- Storing and retrieving the latest root for a given chain
- Verifying that each batched proof is valid against the known state

It is crucial that the SMT root must be published on-chain every ERA, so that proofs can be verified
against the current state.

All recursive ZKPs are verified against the SMT root, which itself becomes a **public input** in the recursive circuit.

---

## ✅ Current Progress

We have a variety of standalone components that now need to be connected into an MVP pipeline:

| Component                                     | Status              |
|----------------------------------------------|---------------------|
| Sparse Merkle Tree (SMT)                     | ✅ Implemented      |
| Merkle Proof Library (Cosmos ICS23 + ETH)    | ✅ Implemented      |
| Recursive ZK Circuit                         | ✅ Working prototype|
| ZK Light Clients (ETH + Cosmos, via Succinct)| ✅ Available        |

---

## 🔍 Unresolved: Circuit Performance

An open question is whether the current recursive prover backend is ideal.

- **Current**: SP1 prover
- **Alternatives**: Consider **Plonky2** or other recursive systems

We need to benchmark:
- Circuit depth
- Batch size scaling
- Proof generation time

---

## 🔄 Proposed Development Strategy

To enable rapid iteration and testing while light clients and circuits evolve, we propose the following MVP-first plan:

### **1. Mock Roots for Development**
Instead of integrating real ZK light clients immediately:

- Manually post **mock trusted roots** for each domain (e.g., Ethereum, Neutron)
- This lets us simulate state finality and test batching logic

### **2. Deploy Coprocessor (MVP)**
- Deploy SMT and recursive circuit logic
- Accept proof inputs from the Merkle proof library
- Allow manually posting new trusted roots

### **3. Querying Mechanism**
- Expose a querying interface for `(KEY, DOMAIN)` pairs
- Internally:
  - Find the latest trusted root for that domain (from the SMT)
  - Batch these requests together
  - Generate a compressed state proof using recursive ZK verification
  - Return a verifiable Groth16 proof

### **4. ERA Update Semantics**

We introduce the concept of **ERAs**, unified update windows for roots across all supported chains.

Important notes:
- Each chain may finalize blocks at different rates
- For example:
  - Ethereum every 12s
  - Cosmos every 6s
- Despite this, the coprocessor maintains only the **latest finalized root** for each domain

All verification occurs **against the last known finalized root**, which may have different ages across chains. This inconsistency must be made explicit in all proof semantics.

---

## 🧪 Additional Considerations

### Finality Consistency

- ERA roots must not assume synchronous freshness
- Clients should always be aware that the "latest trusted root" for Chain A might be significantly older than for Chain B

---

## 🗺 Suggested Development Phases

### ✅ Phase 1: MVP Simulation

- Use **mocked trusted roots**
- Deploy SMT, recursive circuit, and proof library as a closed loop
- Accept queries and produce batched compressed proofs

### 🔗 Phase 2: Light Client Integration

- Connect to Succinct’s light clients:
  - Ethereum
  - Cosmos-based (e.g. Neutron)
- Automate root posting to the SMT

### ⚙️ Phase 3: Optimization & Scaling

- Optimize recursion (Plonky2, etc.)
- Evaluate trade-offs in proof compression (Groth16 vs STARKs)
- Possibly support receipt proofs (ERC20 events)

---

## ✅ Summary

This architecture proposes a zero knowledge coprocessor that verifies cross-chain Merkle proofs using recursion, batching, and SMT-based trusted root storage.

By focusing on an MVP that uses mock roots, we can:

- Validate architecture quickly
- Defer light client complexity
- Enable early proof-of-concept deployments

> ⚠️ This is a **proposed plan**, not a final specification. Feedback and iteration are encouraged as we move forward.
