#!/usr/bin/env bash
# ==============================================================================
# QCOIN POST-QUANTUM BLOCKCHAIN - LINUX VALIDATOR CONSOLE (LIVE)
# ==============================================================================

set -e

echo -e "\033[1;36m"
echo "=============================================================================="
echo "   🌐 QCOIN POST-QUANTUM BLOCKCHAIN - LINUX VALIDATOR CONSOLE (LIVE)         "
echo "=============================================================================="
echo -e "\033[0m"

echo "=============================================================================="
echo "   💰 CONFIGURACION DE CARTERA DE RECOMPENSAS QCOIN                          "
echo "=============================================================================="
read -p "Escribe o pega tu direccion de SubWallet (Enter para usar por defecto): " REWARD_WALLET

if [ -n "${REWARD_WALLET}" ]; then
    echo -e "✅ \033[1;32mCartera asignada correctamente:\033[0m ${REWARD_WALLET}"
else
    echo -e "ℹ️  \033[1;33mUsando cartera predeterminada del validador.\033[0m"
fi
echo ""

echo "🚀 Iniciando Validador de QCOIN en Modo Docker..."

# Stop any running validator container
docker stop qcoin-validator >/dev/null 2>&1 || true
docker rm qcoin-validator >/dev/null 2>&1 || true

# Build docker image if missing
if ! docker image inspect qcoin-node:latest >/dev/null 2>&1; then
    echo "[INFO] Construyendo imagen Docker de QCOIN localmente..."
    docker build -t qcoin-node:latest .
fi

# Run validator container
docker run -d --name qcoin-validator \
  -p 30333:30333 \
  -p 9944:9944 \
  -v qcoin_data_v109:/data \
  -v "$(pwd)":/qcoin \
  qcoin-node:latest \
  --base-path /data \
  --chain /qcoin/qcoin_mainnet_spec.json \
  --validator \
  --unsafe-rpc-external \
  --rpc-cors all \
  --rpc-methods unsafe \
  --node-key 8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193 \
  --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWSNfi1qbFrBrQrauyeDBPEyxSpdxoi1tqW8EdHNuGz3hG

if [ -n "${REWARD_WALLET}" ]; then
    echo "[INFO] Vinculando cartera ${REWARD_WALLET} en la blockchain..."
    sleep 5
    docker exec qcoin-validator python3 -c "from substrateinterface import SubstrateInterface, Keypair; sub=SubstrateInterface(url='ws://127.0.0.1:9944'); key=Keypair.create_from_seed('0x8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193', ss58_format=42); call=sub.compose_call('Template', 'set_reward_wallet', {'new_wallet': '${REWARD_WALLET}'}); receipt=sub.submit_extrinsic(sub.create_signed_extrinsic(call=call, keypair=key), wait_for_inclusion=True); print('✅ RECOMPENSAS VINCULADAS CON EXITO A:', '${REWARD_WALLET}') if receipt.is_success else print('❌ Error al vincular cartera')" || true
fi

echo ""
echo "=============================================================================="
echo "   🟢 LIVE VALIDATOR NODE LOGS (Presiona Ctrl+C para salir)"
echo "=============================================================================="
echo ""
docker logs -f qcoin-validator
