@echo off
TITLE QCOIN Post-Quantum Blockchain - Windows Validator Launcher
COLOR 0A
cls

echo ==============================================================================
echo    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - WINDOWS VALIDATOR LAUNCHER
echo ==============================================================================
echo.

echo [1/3] Checking environment...

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
echo 🚀 Starting QCOIN Validator Node via Docker...
docker stop qcoin-validator >nul 2>&1
docker rm qcoin-validator >nul 2>&1

:: Build image if missing
docker image inspect qcoin-node:latest >nul 2>&1
if %errorlevel% neq 0 (
    echo [INFO] Building QCOIN container image locally...
    docker build -t qcoin-node:latest .
)

docker run -d --name qcoin-validator -p 30333:30333 -p 9944:9944 -v qcoin_data:/data qcoin-node:latest --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd

echo.
echo ✅ QCOIN Validator Node is running in background!
echo To view logs, run: docker logs -f qcoin-validator
goto END

:USE_WSL
echo [OK] Windows Subsystem for Linux detected!
echo 🚀 Launching QCOIN Validator inside WSL...
wsl bash -c "chmod +x start_qcoin_all.sh && ./start_qcoin_all.sh"
goto END

:USE_EXE
echo [OK] Native Windows binary detected!
echo 🚀 Starting QCOIN Validator Node...
solochain-template-node.exe --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd
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
