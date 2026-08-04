@echo off
TITLE QCOIN Post-Quantum Blockchain - Windows Validator Console (LIVE)
COLOR 0A
cls

echo ==============================================================================
echo    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - WINDOWS VALIDATOR CONSOLE (LIVE)
echo ==============================================================================
echo.

echo [1/2] Checking environment...

:: Check Docker Desktop first
docker --version >nul 2>&1
if %errorlevel% equ 0 goto USE_DOCKER

:: Check WSL next
wsl --status >nul 2>&1
if %errorlevel% equ 0 goto USE_WSL

:: Check native exe
if exist "solochain-template-node.exe" goto USE_EXE

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
docker run -d --name qcoin-validator -p 30333:30333 -p 9944:9944 -v qcoin_data:/data -v "%cd%":/qcoin qcoin-node:latest --base-path /data --chain /qcoin/qcoin_mainnet_spec.json --validator --unsafe-rpc-external --rpc-cors all --rpc-methods unsafe --node-key 8dd6190191a6062364d12d7449fa120de8b16bba48f6fc6903a19c04ee289193 --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWGVfgs9dCt98HxkJnEiQXXYqxFnokgHyXzPzFmrCtEUGW

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
solochain-template-node.exe --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWGVfgs9dCt98HxkJnEiQXXYqxFnokgHyXzPzFmrCtEUGW
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
