# 🛡️ QCOIN Validator Node Operator Guide

Welcome to the **QCOIN Post-Quantum Network**. By running a validator node, you contribute to securing the world's first NIST FIPS 204 ML-DSA-65 Layer-1 blockchain while earning real native **QCOIN rewards**.

---

## 💎 Why Run a QCOIN Validator Node?

1. **100% Transaction Fee Earnings**: Every block you mine awards you 100% of the network transaction fees processed in that block.
2. **Post-Quantum Block Validation**: Your node verifies quantum-resistant lattice signatures in WebAssembly (WASM).
3. **Low Hardware Requirements**: Extremely lightweight Substrate node running smoothly on budget VPS hardware.
4. **Decentralized Network Growth**: Protect your assets and help decentralize the QCOIN Layer-1 ecosystem.

---

## 💻 Hardware Requirements

| Component | Minimum Specification | Recommended Specification |
| :--- | :--- | :--- |
| **CPU** | 2 Cores | 4 Cores |
| **RAM** | 2 GB | 4 GB |
| **Storage** | 30 GB SSD | 80 GB NVMe SSD |
| **Bandwidth** | 100 Mbps | 1 Gbps |
| **OS** | Ubuntu 22.04 LTS / Debian 12 | Ubuntu 22.04 LTS |

---

## 🚀 Quickstart: Run a Validator Node in 3 Steps

### Step 1: Clone the Official Repository

```bash
git clone https://github.com/Alerod427/QCOIN-Blockchain.git
cd QCOIN-Blockchain
```

### Step 2: Make Launcher Executable

```bash
chmod +x start_qcoin_all.sh
```

### Step 3: Launch the Node & Services

```bash
./start_qcoin_all.sh
```

The script will automatically compile the release binary, start the QCOIN validator node, establish the SSL WSS RPC tunnel, and launch the local block explorer dashboard.

---

## 🪟 Running a QCOIN Validator on Windows

Running a QCOIN node on Windows is fast and simple. You can use any of the 2 methods below:

### Method 1: WSL2 (Windows Subsystem for Linux) - Recommended

1. **Enable WSL2**: Open **PowerShell as Administrator** and run:
   ```powershell
   wsl --install
   ```
   *(Restart your computer if prompted)*.

2. **Open Ubuntu Terminal**: Open Ubuntu from your Windows Start Menu and run:
   ```bash
   git clone https://github.com/Alerod427/QCOIN-Blockchain.git
   cd QCOIN-Blockchain
   chmod +x start_qcoin_all.sh
   ./start_qcoin_all.sh
   ```

### Method 2: 1-Click Launcher (Docker for Windows)

1. **Install Docker Desktop**: Download and install [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/).
2. **Double-Click Launcher**: Double-click `run_validator_windows.bat` in the QCOIN repository folder.
3. The script will automatically start your QCOIN Validator Node in background!

---

## 📊 Monitoring Your Node

- **View Live Logs**:
  ```bash
  tail -f logs/node.log
  ```
- **Check Block Heights**:
  Open your live Polkadot-JS console printed in the terminal or visit `http://YOUR_SERVER_IP:8080#explorer`.

- **Stop Services**:
  ```bash
  pkill -f solochain-template-node && pkill -f cloudflared && pkill -f 'http.server 8080'
  ```

---

## 🛡️ Security Best Practices

1. Keep your server OS updated (`sudo apt update && sudo apt upgrade -y`).
2. Use UFW firewall to restrict non-essential ports.
3. Backup your validator key files securely.

---
© 2026 QCOIN Post-Quantum Blockchain Network.
