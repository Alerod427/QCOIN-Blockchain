# 🪙 QCOIN Tokenomics & Monetary Policy

## Executive Summary
**QCOIN** is the native cryptocurrency of the Post-Quantum Solochain Blockchain (`qcoin-node`). It is designed to be a quantum-resistant digital asset powering safe decentralized value transfer in an era where quantum computing threatens traditional ECDSA/RSA cryptography.

---

## 📊 Token Metrics

| Parameter | Specification |
| :--- | :--- |
| **Token Name** | QCOIN |
| **Ticker Symbol** | **QCOIN** |
| **Base Unit** | `Planck` (smallest indivisible unit) |
| **Decimals** | 12 (`1 QCOIN = 1,000,000,000,000 Plancks` / `10^12`) |
| **Initial Supply (Genesis)** | `1,000,000,000 QCOIN` (1 Billion QCOIN) |
| **Circulating Supply at Launch** | `100,000,000 QCOIN` (10% Dev/Local Genesis Distribution) |
| **Existential Deposit (ED)** | `1 QCOIN` (`1,000,000,000,000 Plancks`) |
| **Consensus Mechanism** | Proof-of-Authority / Aura (Block Production) + Grandpa (Finality) |

---

## 💰 Initial Genesis Distribution

The initial supply of **1,000,000,000 QCOIN** is allocated as follows:

```
+-----------------------------------------------------------+
| QCOIN Genesis Allocation                                  |
+-----------------------------------------------------------+
| [████████████████████████████████████████] 50% Reserve   |
| [████████████████████] 25% Ecosystem & Treasury          |
| [████████████] 15% Core Developers & Foundation          |
| [██████] 10% Initial Endowed Accounts (Alice/Bob/Stash)  |
+-----------------------------------------------------------+
```

1. **50% (500,000,000 QCOIN) - Post-Quantum Security Reserve**: Allocated for future staking rewards, validator incentives, and migration pools.
2. **25% (250,000,000 QCOIN) - Ecosystem & Treasury**: Managed by on-chain governance for grants, dApp incentives, and audit funds.
3. **15% (150,000,000 QCOIN) - Core Development & Foundation**: Vested over 48 months with a 12-month cliff for core protocol engineers.
4. **10% (100,000,000 QCOIN) - Initial Genesis Circulation**: Distributed to initial genesis accounts (Alice, Bob, Charlie, Dave, Eve, Ferdie and Stashes) with `1u128 << 60` units (~1.15 M QCOIN each) for local network testing and deployment.

---

## ⚙️ Monetary Policy & Fee Model

### 1. Dynamic Transaction Fees (`pallet-transaction-payment`)
Transaction fees on the QCOIN network are calculated dynamically based on three factors:
- **Base Fee**: Constant minimum weight required to execute an extrinsic.
- **Length Fee**: Proportional to the size of the extrinsic in bytes (important due to larger ML-DSA-65 signatures).
- **Weight Fee**: Compute time required on the WASM runtime execution environment.

### 2. Fee Burning & Treasury Split
- **80% of Transaction Fees**: Automatically **burned** to create a deflationary pressure proportional to network usage.
- **20% of Transaction Fees**: Sent to the **Treasury Account** to support long-term development.

### 3. Existential Deposit
Accounts must maintain a minimum balance of **1 QCOIN** (`10^12 Plancks`). If an account balance falls below this limit, the account is reaped from storage to keep state size lean.
