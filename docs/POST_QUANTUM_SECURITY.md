# 🛡️ Post-Quantum Cryptography Architecture (NIST FIPS 204 ML-DSA-65)

## 📌 Context & Motivation
Standard blockchain networks (such as Bitcoin and Ethereum) rely on Elliptic Curve Cryptography (ECDSA secp256k1 or Ed25519) for signing transactions. 
However, **Shor's Algorithm** running on a sufficiently powerful Quantum Computer can solve the Elliptic Curve Discrete Logarithm Problem (ECDLP) in polynomial time, completely compromising private keys derived from public keys.

The **QCOIN Solochain Node** integrates **NIST FIPS 204 ML-DSA-65 (Module-Lattice-Based Digital Signature Standard, formerly CRYSTALS-Dilithium)** directly inside its WebAssembly (WASM) runtime environment (`pallet-template` / `no_std`).

---

## 🔬 Cryptographic Specification: ML-DSA-65

| Feature | Specification |
| :--- | :--- |
| **Standard** | NIST FIPS 204 (Final Standardized Specification) |
| **Algorithm Family** | Module-Lattice-Based Digital Signature Algorithm (ML-DSA) |
| **Security Category** | NIST Category 3 (equivalent to AES-192 against quantum attacks) |
| **Public Key Size (`PK_LEN`)** | **1,952 bytes** |
| **Signature Size (`SIG_LEN`)** | **3,309 bytes** |
| **Secret Key Size (`SK_LEN`)** | **4,032 bytes** |
| **Rust Crate** | `fips204 v0.4.6` (Safe Rust, 100% `no_std`, zero dynamic allocation) |

---

## ⚙️ How Post-Quantum Verification Works On-Chain

```
+------------------+         +------------------+         +--------------------------+
|  User (Off-Chain)|         | Post-Quantum Key |         | On-Chain State           |
|  ML-DSA-65 Key   | ------->| Registration     | ------->| PqPublicKeys<T>          |
|  Generation      |         | Extrinsic        |         | (AccountId => 1952 bytes)|
+------------------+         +------------------+         +--------------------------+
          |                                                            |
          | Signs message                                              | Validates against
          v                                                            v
+------------------+                                      +--------------------------+
| Off-Chain        | ------------------------------------>| Extrinsic                |
| Signature (3309B)|                                      | verify_pq_signature()    |
+------------------+                                      +--------------------------+
                                                                       |
                                                                       v
                                                          +--------------------------+
                                                          | fips204::ml_dsa_65       |
                                                          | pk.verify(msg, sig, b"") |
                                                          +--------------------------+
                                                                       |
                                                                       v
                                                          +--------------------------+
                                                          | VerifiedPqCount += 1     |
                                                          | Event::PqSignatureVerified|
                                                          +--------------------------+
```

### 1. `register_pq_public_key` Extrinsic
- Users register their 1,952-byte ML-DSA-65 public key on-chain.
- The runtime verifies the structural integrity of the public key bytes via `ml_dsa_65::PublicKey::try_from_bytes`.
- The key is saved in storage map `PqPublicKeys<T>`.

### 2. `verify_pq_signature` Extrinsic
- The user submits a message and a 3,309-byte post-quantum signature.
- The runtime fetches the user's stored public key from `PqPublicKeys<T>`.
- The `fips204` engine executes lattice-based verification `pk.verify(message, signature, b"")` inside the WASM execution sandbox.
- On success, `VerifiedPqCount` is incremented and `PqSignatureVerified` event is emitted.
