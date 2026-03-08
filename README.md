# Post-Quantum Signatures in Substrate – Benchmark Client

This project evaluates the impact of **post-quantum signatures (CRYSTALS-Dilithium / ML-DSA)** on a **Substrate-based blockchain**.  
It benchmarks Dilithium variants against commonly used blockchain signatures:

- **sr25519**
- **ECDSA**
- **ML-DSA-44 (Dilithium2)**
- **ML-DSA-65 (Dilithium3)**
- **ML-DSA-87 (Dilithium5)**

The goal is to measure the performance implications of replacing classical signatures with post-quantum alternatives in a Substrate runtime.

---

Two executables are provided:

### Experiment Runner

Runs benchmarking experiments and exports results to CSV files.
`cargo run --bin experiments`

### Simple Client

Sends transactions using all supported signature algorithms.
`cargo run --bin client`

---

# Metrics Collected

The benchmarking framework measures several aspects of signature performance.

### 1. Ledger Transaction Latency

Time between transaction submission and finalization on-chain.

### 2. Key Generation Latency

Time required to generate a keypair from a seed.

### 3. Transaction Signing Latency

Time required to sign a transaction payload.

### 4. Extrinsic Size and Weight

Encoded size of the transaction containing the signature and On-chain computational cost of verifying the signature.

---

# Requirements

- Rust
- Cargo
- A running **Substrate node** exposing RPC at: `ws://127.0.0.1:9944`

The development node should support:

- sr25519
- ecdsa
- Dilithium / ML-DSA

---

# Changing Dilithium Versions

The project supports three Dilithium variants:

| Variant    | Standard Name | Branch                |
| ---------- | ------------- | --------------------- |
| Dilithium2 | ML-DSA-44     | `dilithium-ml-dsa-44` |
| Dilithium3 | ML-DSA-65     | `dilithium-ml-dsa-65` |
| Dilithium5 | ML-DSA-87     | `dilithium`           |

To switch variants, update the Git dependencies used in the project via Cargo.toml.

---

## Dilithium2 (ML-DSA-44)

Change dependencies to:

https://github.com/bsaviozz/polkadot-sdk.git
branch = "dilithium-ml-dsa-44"

https://github.com/bsaviozz/subxt
branch = "dilithium-ml-dsa-44"

Also update labels in the benchmarking code to: "ml_dsa_44"

---

## Dilithium3 (ML-DSA-65)

Change dependencies to:

https://github.com/bsaviozz/polkadot-sdk.git
branch = "dilithium-ml-dsa-65"

https://github.com/bsaviozz/subxt
branch = "dilithium-ml-dsa-65"

Update labels in the benchmarking code to: "ml_dsa_65"

---

## Dilithium5 (ML-DSA-87)

Change dependencies to:

https://github.com/bsaviozz/polkadot-sdk.git
branch = "dilithium"

https://github.com/bsaviozz/subxt
branch = "master"

Update labels in the benchmarking code to: "ml_dsa_87"

---

# Example Workflow

1. Start the Substrate development node
2. Run the experiment suite

cargo run --bin experiment

3. Results will be written to CSV files.

If you want to process the files for statistical analysis:

1. Move files to csv-files folder
2. Run `python graphs.py`

---

# Purpose

Blockchains rely heavily on digital signatures for transaction validation and consensus security.  
However, widely used schemes such as **ECDSA** and **sr25519** are vulnerable to future **quantum computers**.

This project evaluates the feasibility of integrating **post-quantum signatures** into Substrate by measuring:

- computational overhead
- transaction size impact
- verification throughput
- blockchain performance implications
