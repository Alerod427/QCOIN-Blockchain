# 🌐 QCOIN Post-Quantum Blockchain Documentation Hub

Welcome to the official documentation directory for **QCOIN**, a Post-Quantum Layer-1 Solochain built with Substrate / Polkadot SDK and NIST FIPS 204 ML-DSA-65 (CRYSTALS-Dilithium) post-quantum cryptography.

---

## 📚 Documentation Index

1. [🪙 QCOIN Tokenomics & Monetary Policy](QCOIN_TOKENOMICS.md)
   - Initial Genesis Supply (1,000,000,000 QCOIN)
   - Token Decimals (12 Decimals / Plancks)
   - On-Chain Block Mining Rewards & Halving Schedule (10 -> 5 -> 2.5 QCOIN)
   - Dynamic Transaction Fee Model & Deflationary Burning Mechanism

2. [🛡️ Post-Quantum Security Architecture](POST_QUANTUM_SECURITY.md)
   - NIST FIPS 204 ML-DSA-65 Cryptography Specification
   - Public Key (1,952 bytes) & Signature (3,309 bytes) Formats
   - Quantum Threat Model (Shor's Algorithm Defense)
   - WASM Execution Sandbox & On-Chain Verification

3. [📖 User & Developer Guide](USER_GUIDE.md)
   - Installation & Build Instructions
   - Running the Local Node (`./target/release/solochain-template-node --dev`)
   - Unit Testing Guide (`cargo test --package pallet-template`)
   - Interacting with Post-Quantum Extrinsics & RPCs

---

## 🛠️ Project Structure Overview

- **`node/`**: Solochain Node implementation (CLI, RPCs, Consensus, Network Service).
- **`runtime/`**: WASM Runtime execution environment (`solochain-template-runtime`).
- **`pallets/template/`**: FRAME Pallet implementing `fips204` ML-DSA-65 post-quantum signature verification on-chain.
- **`docs/`**: Official project documentation.
- **`.cargo/config.toml`**: WASM target linking configuration (`-C link-arg=--import-undefined`).
