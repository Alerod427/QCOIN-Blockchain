# 📖 QCOIN User & Developer Guide

This guide provides instructions on how to build, run, test, and interact with the **QCOIN Post-Quantum Solochain Blockchain**.

---

## 🛠️ Prerequisites & Installation

### 1. Install Rust Toolchain & Target
Ensure Rust is installed with the `wasm32-unknown-unknown` target:
```bash
rustup default stable
rustup target add wasm32-unknown-unknown
```

---

## 🚀 Building & Running the Node

### 1. Build in Release Mode
```bash
cargo build --package solochain-template-node --release
```

### 2. Run the Node in Development Mode
To start a single local node with a clean genesis state:
```bash
./target/release/solochain-template-node --dev
```

You will see log output indicating block production:
```text
2026-08-03 13:30:00 Substrate Node
2026-08-03 13:30:00 ✌️ version 0.1.0-cd9d00e0
2026-08-03 13:30:00 💻 Operating system: linux
2026-08-03 13:30:00 📦 Chain specification: Development
2026-08-03 13:30:06 🙌 Installed 1 genesis block(s)
2026-08-03 13:30:12 📑 Prepared block for proposing at 1 [hash: 0x...; parent_hash: 0x...]
2026-08-03 13:30:12 ⚓ Pre-sealed block for proposal at 1. Submitting slot at 1...
2026-08-03 13:30:12 ✨ Imported #1 (0x...)
```

---

## 🧪 Running Unit & Integration Tests

Run the unit test suite for the post-quantum pallet (`pallet-template`):
```bash
cargo test --package pallet-template
```
Output:
```text
running 5 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test mock::test_genesis_config_builds ... ok
test tests::correct_error_for_none_value ... ok
test tests::it_works_for_default_value ... ok
test tests::pq_signature_verification_works ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

---

## 🌐 Connecting via Front-End Interfaces

Once your node is running (`ws://127.0.0.1:9944`), you can connect using:

1. **Polkadot-JS Apps**:
   Open [https://polkadot.js.org/apps](https://polkadot.js.org/apps) and switch network settings to `Local Node` (`ws://127.0.0.1:9944`).
2. **Subxt / Polkadot API (Javascript / Rust)**:
   Query storage (`TemplateModule::PqPublicKeys`, `TemplateModule::VerifiedPqCount`) or submit extrinsics (`registerPqPublicKey`, `verifyPqSignature`).

---

## 🔐 Interacting with Post-Quantum Extrinsics

### 1. Registering an ML-DSA-65 Public Key
- **Module**: `TemplateModule`
- **Call**: `registerPqPublicKey(public_key)`
- **Parameters**: `public_key` (Hex-encoded 1,952 bytes)

### 2. Verifying a Post-Quantum Signature
- **Module**: `TemplateModule`
- **Call**: `verifyPqSignature(message, signature)`
- **Parameters**:
  - `message`: Hex-encoded raw message bytes (e.g. `0x48656c6c6f`).
  - `signature`: Hex-encoded 3,309 bytes ML-DSA-65 signature.
