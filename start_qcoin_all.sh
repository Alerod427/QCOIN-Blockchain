#!/usr/bin/env bash
# ==============================================================================
# QCOIN POST-QUANTUM BLOCKCHAIN SYSTEM - MASTER UNIFIED LAUNCHER
# ==============================================================================
# This script starts all 3 core services in 1 single command:
# 1. QCOIN Mainnet Layer-1 Blockchain Node (NIST FIPS 204 ML-DSA-65)
# 2. Cloudflare SSL WSS Tunnel for SubWallet / Polkadot-JS
# 3. Web dApp & Integrated Block Explorer HTTP Server (Port 8080)
# ==============================================================================

set -e

PROJECT_DIR="/home/alerod/Escritorio/qcoin-node"
LOGS_DIR="${PROJECT_DIR}/logs"

mkdir -p "${LOGS_DIR}"

echo -e "\033[1;36m"
echo "  =========================================================="
echo "    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - UNIFIED LAUNCHER     "
echo "  =========================================================="
echo -e "\033[0m"

echo "⏳ [1/4] Stopping any previous services..."
pkill -9 -f "solochain-template-node" 2>/dev/null || true
pkill -9 -f "cloudflared" 2>/dev/null || true
pkill -9 -f "http.server 8080" 2>/dev/null || true
sleep 1

echo "🚀 [2/4] Starting QCOIN Mainnet Blockchain Node..."
cd "${PROJECT_DIR}"
./target/release/solochain-template-node \
  --chain "${PROJECT_DIR}/qcoin_mainnet_spec.json" \
  --alice \
  --validator \
  --force-authoring \
  --unsafe-rpc-external \
  --rpc-cors all \
  --rpc-port 9944 \
  > "${LOGS_DIR}/node.log" 2>&1 &
NODE_PID=$!
sleep 2

echo "🔒 [3/4] Starting Cloudflare SSL WSS Tunnel..."
/tmp/cloudflared tunnel --url http://127.0.0.1:9944 > "${LOGS_DIR}/cloudflared.log" 2>&1 &
TUNNEL_PID=$!
sleep 5

echo "🌐 [4/4] Starting Web dApp & Integrated Block Explorer (Port 8080)..."
python3 -m http.server 8080 --directory "${PROJECT_DIR}/dapp" > "${LOGS_DIR}/web.log" 2>&1 &
WEB_PID=$!
sleep 1

# Extract Tunnel URL from log with retry loop
TUNNEL_URL=""
for i in {1..10}; do
  TUNNEL_URL=$(grep -o 'https://[^ ]*\.trycloudflare\.com' "${LOGS_DIR}/cloudflared.log" | tail -n 1 || echo "")
  if [ -n "${TUNNEL_URL}" ]; then
    break
  fi
  sleep 1
done

WSS_URL=$(echo "${TUNNEL_URL}" | sed 's/https:/wss:/')

# Save active WSS URL for dApp Web Portal
echo "${WSS_URL}" > "${PROJECT_DIR}/dapp/active_wss_url.txt"

POLKADOT_JS_LINK="https://polkadot.js.org/apps/?rpc=$(echo "${WSS_URL}" | sed 's/:/%3A/g' | sed 's/\//%2F/g')"

echo -e "\033[1;32m"
echo "  =========================================================="
echo "   ✅ ALL QCOIN SERVICES ARE NOW LIVE & RUNNING 24/7!      "
echo "  =========================================================="
echo -e "\033[0m"

echo -e "🟢 \033[1mQCOIN Blockchain Node:\033[0m PID ${NODE_PID} | Log: ${LOGS_DIR}/node.log"
echo -e "🔒 \033[1mCloudflare WSS RPC Tunnel:\033[0m PID ${TUNNEL_PID} | Log: ${LOGS_DIR}/cloudflared.log"
echo -e "🌐 \033[1mWeb dApp & Block Explorer:\033[0m PID ${WEB_PID} | Log: ${LOGS_DIR}/web.log"
echo ""
echo -e "\033[1;33m📌 SubWallet / Wallet RPC Endpoint:\033[0m ${WSS_URL}"
echo -e "\033[1;33m🔗 Polkadot-JS Apps Explorer:\033[0m ${POLKADOT_JS_LINK}"
echo -e "\033[1;33m🌐 Public Web Portal & Explorer:\033[0m http://158.179.211.45:8080"
echo ""
echo "ℹ️  To monitor logs in real-time: tail -f logs/node.log"
echo "ℹ️  To stop all services: pkill -f solochain-template-node && pkill -f cloudflared && pkill -f 'http.server 8080'"
echo ""
