#!/usr/bin/env bash
# ==============================================================================
# QCOIN POST-QUANTUM BLOCKCHAIN - LINUX VALIDATOR CONSOLE (LIVE)
# ==============================================================================

# Prevent terminal window from closing automatically on error or exit
finish() {
    echo ""
    read -p "Presiona ENTER para salir..." unused 2>/dev/null || true
}
trap finish EXIT

echo -e "\033[1;36m"
echo "=============================================================================="
echo "   🌐 QCOIN POST-QUANTUM BLOCKCHAIN - LINUX VALIDATOR CONSOLE (LIVE)         "
echo "=============================================================================="
echo -e "\033[0m"

# 1. Determine Docker command & permissions (docker vs sudo docker)
DOCKER_CMD="docker"
if ! docker info >/dev/null 2>&1; then
    if sudo docker info >/dev/null 2>&1; then
        DOCKER_CMD="sudo docker"
    else
        echo -e "❌ \033[1;31mError: El servicio de Docker no está en ejecución o no tienes permisos.\033[0m"
        echo ""
        echo "Por favor ejecuta en tu terminal para iniciar el servicio de Docker:"
        echo "  sudo systemctl start docker"
        echo "  sudo usermod -aG docker \$USER"
        echo ""
        exit 1
    fi
fi

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

# Stop any previous validator container
${DOCKER_CMD} stop qcoin-validator >/dev/null 2>&1 || true
${DOCKER_CMD} rm qcoin-validator >/dev/null 2>&1 || true

# Ensure release binary exists locally or build via Docker
if [ ! -f "target/release/solochain-template-node" ]; then
    if command -v cargo >/dev/null 2>&1; then
        echo "[INFO] Compilando ejecutable nativo de QCOIN (cargo build --release)..."
        cargo build --release
    else
        echo "[INFO] Cargo no está instalado en el sistema anfitrión. Docker compilará el nodo automáticamente dentro del contenedor."
    fi
fi

# Build docker image if missing
if ! ${DOCKER_CMD} image inspect qcoin-node:latest >/dev/null 2>&1; then
    echo "[INFO] Construyendo imagen Docker de QCOIN localmente..."
    ${DOCKER_CMD} build -t qcoin-node:latest .
fi

# Mount host SSL certs if available
SSL_MOUNT=""
if [ -d "/etc/ssl/certs" ]; then
    SSL_MOUNT="-v /etc/ssl/certs:/etc/ssl/certs:ro"
fi

# Run validator container
${DOCKER_CMD} run -d --name qcoin-validator \
  --add-host=host.docker.internal:host-gateway \
  -p 30333:30333 \
  -p 9944:9944 \
  -v qcoin_data_v110:/data \
  -v "$(pwd)":/qcoin \
  ${SSL_MOUNT} \
  qcoin-node:latest \
  --base-path /data \
  --chain /qcoin/qcoin_mainnet_spec.json \
  --validator \
  --no-telemetry \
  --unsafe-rpc-external \
  --rpc-cors all \
  --rpc-methods unsafe \
  --bootnodes /ip4/172.17.0.1/tcp/30333/p2p/12D3KooWLz3Yj6Bxi5FdQDfKjkn7J1K535jbT2WhFD373EdP5z7P /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWLz3Yj6Bxi5FdQDfKjkn7J1K535jbT2WhFD373EdP5z7P

if [ -n "${REWARD_WALLET}" ]; then
    echo "[INFO] Vinculando cartera ${REWARD_WALLET} en la blockchain..."
    sleep 5
    python3 -c "from substrateinterface import SubstrateInterface, Keypair; sub=SubstrateInterface(url='ws://127.0.0.1:9944'); key=Keypair.create_from_seed('0x8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193', ss58_format=42); call=sub.compose_call('Template', 'set_reward_wallet', {'new_wallet': '${REWARD_WALLET}'}); receipt=sub.submit_extrinsic(sub.create_signed_extrinsic(call=call, keypair=key), wait_for_inclusion=True); print('✅ RECOMPENSAS VINCULADAS CON EXITO A:', '${REWARD_WALLET}') if receipt.is_success else print('❌ Error al vincular cartera')" 2>/dev/null || ${DOCKER_CMD} exec qcoin-validator python3 -c "from substrateinterface import SubstrateInterface, Keypair; sub=SubstrateInterface(url='ws://127.0.0.1:9944'); key=Keypair.create_from_seed('0x8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193', ss58_format=42); call=sub.compose_call('Template', 'set_reward_wallet', {'new_wallet': '${REWARD_WALLET}'}); receipt=sub.submit_extrinsic(sub.create_signed_extrinsic(call=call, keypair=key), wait_for_inclusion=True); print('✅ RECOMPENSAS VINCULADAS CON EXITO A:', '${REWARD_WALLET}') if receipt.is_success else print('❌ Error al vincular cartera')" || true
fi

echo ""
echo "=============================================================================="
echo "   🟢 LIVE VALIDATOR NODE LOGS (Presiona Ctrl+C para salir)"
echo "=============================================================================="
echo ""
${DOCKER_CMD} logs -f qcoin-validator
