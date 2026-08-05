@echo off
TITLE QCOIN Post-Quantum Blockchain - Windows Validator Console (LIVE)
COLOR 0A
cls

echo ==============================================================================
echo    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - WINDOWS VALIDATOR CONSOLE (LIVE)
echo ==============================================================================
echo.

echo [1/2] Checking environment...

:: Check Docker Desktop daemon status
docker info >nul 2>&1
if %errorlevel% equ 0 goto USE_DOCKER

:: Check if Docker CLI is installed but daemon is stopped
docker --version >nul 2>&1
if %errorlevel% equ 0 (
    set DOCKER_STOPPED=1
)

:: Check WSL next
wsl --status >nul 2>&1
if %errorlevel% equ 0 goto USE_WSL

:: Check native exe
if exist "solochain-template-node.exe" goto USE_EXE

if defined DOCKER_STOPPED (
    echo [ERROR] Docker Desktop esta instalado pero NO esta iniciado.
    echo.
    echo Por favor, abre la aplicacion 'Docker Desktop' en Windows, espera a que el icono cambie a verde (Engine running) y vuelve a ejecutar este archivo.
    goto END
)

goto NO_ENV

:USE_DOCKER
echo [OK] Docker Desktop detected!
echo.
echo ==============================================================================
echo    💰 CONFIGURACION DE CARTERA DE RECOMPENSAS QCOIN
echo ==============================================================================
set /p REWARD_WALLET="Escribe o pega tu direccion de SubWallet (Enter para usar por defecto): "
echo.
if not "%REWARD_WALLET%"=="" (
    echo [OK] Cartera asignada correctamente: %REWARD_WALLET%
) else (
    echo [OK] Usando cartera predeterminada del validador.
)
echo.
echo 🚀 Iniciando Validador de QCOIN en Modo Consola en Vivo...
echo.

docker stop qcoin-validator >nul 2>&1
docker rm qcoin-validator >nul 2>&1

:: Build image if missing
docker image inspect qcoin-node:latest >nul 2>&1
if %errorlevel% neq 0 (
    echo [INFO] Building QCOIN container image locally...
    docker build -t qcoin-node:latest .
)

:: Start container with inline node key and external RPC access
:: Each validator operator should replace this key with their own unique 64-char hex string
docker run -d --name qcoin-validator --add-host=host.docker.internal:host-gateway -p 30333:30333 -p 9944:9944 -v qcoin_data_v110:/data -v "%cd%":/qcoin qcoin-node:latest --base-path /data --chain /qcoin/qcoin_mainnet_spec.json --validator --no-telemetry --unsafe-rpc-external --rpc-cors all --rpc-methods unsafe --bootnodes /ip4/10.0.0.90/tcp/30333/p2p/12D3KooWLz3Yj6Bxi5FdQDfKjkn7J1K535jbT2WhFD373EdP5z7P /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWLz3Yj6Bxi5FdQDfKjkn7J1K535jbT2WhFD373EdP5z7P

if not "%REWARD_WALLET%"=="" (
    echo [INFO] Vinculando cartera %REWARD_WALLET% en la blockchain...
    timeout /t 5 >nul
    docker exec qcoin-validator python3 -c "from substrateinterface import SubstrateInterface, Keypair; sub=SubstrateInterface(url='ws://127.0.0.1:9944'); key=Keypair.create_from_seed('0x8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193', ss58_format=42); call=sub.compose_call('Template', 'set_reward_wallet', {'new_wallet': '%REWARD_WALLET%'}); receipt=sub.submit_extrinsic(sub.create_signed_extrinsic(call=call, keypair=key), wait_for_inclusion=True); print('✅ RECOMPENSAS VINCULADAS CON EXITO A:', '%REWARD_WALLET%') if receipt.is_success else print('❌ Error al vincular cartera')" >nul 2>&1
)

echo.
echo ==============================================================================
echo    🟢 LIVE VALIDATOR NODE LOGS (Press Ctrl+C to disconnect view)
echo ==============================================================================
echo.
docker logs -f qcoin-validator
goto END

:USE_WSL
echo [OK] Windows Subsystem for Linux detected!
echo 🚀 Launching QCOIN Validator inside WSL in LIVE CONSOLE MODE...
wsl bash -c "chmod +x start_qcoin_all.sh && ./start_qcoin_all.sh && tail -f logs/node.log"
goto END

:USE_EXE
echo [OK] Native Windows binary detected!
echo 🚀 Starting QCOIN Validator Node in LIVE CONSOLE MODE...
solochain-template-node.exe --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWSNfi1qbFrBrQrauyeDBPEyxSpdxoi1tqW8EdHNuGz3hG
goto END

:NO_ENV
echo ❌ Neither Docker Desktop nor WSL2 were detected on this Windows PC.
echo.
echo To run a QCOIN Validator Node on Windows, please install Docker Desktop:
echo https://www.docker.com/products/docker-desktop/
echo.

:END
echo.
echo Press any key to exit...
pause >nul
