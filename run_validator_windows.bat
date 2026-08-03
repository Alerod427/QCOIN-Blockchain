@echo off
TITLE QCOIN Post-Quantum Blockchain - Windows Validator Node Launcher
COLOR 0A
cls

echo ==============================================================================
echo    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - WINDOWS VALIDATOR LAUNCHER
echo ==============================================================================
echo.

echo Checking environment...

:: Priority 1: Check if WSL2 is installed (Fastest & Most Reliable Native Linux Kernel)
wsl --status >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] Windows Subsystem for Linux (WSL) detected!
    echo Launching QCOIN Validator Node inside WSL...
    wsl bash -c "chmod +x start_qcoin_all.sh && ./start_qcoin_all.sh"
    goto END
)

:: Priority 2: Check if Docker Desktop is installed
docker --version >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] Docker Desktop detected on Windows!
    
    :: Check if local image exists, if not build it locally
    docker image inspect qcoin-node:latest >nul 2>&1
    if %errorlevel% neq 0 (
        echo 🔨 Building QCOIN Docker image locally (this takes a few moments on first run)...
        docker build -t qcoin-node:latest .
    )

    echo 🚀 Starting QCOIN Validator Node via Docker...
    docker stop qcoin-validator >nul 2>&1
    docker rm qcoin-validator >nul 2>&1

    docker run -d --name qcoin-validator ^
        -p 30333:30333 -p 9944:9944 ^
        -v qcoin_data:/data ^
        qcoin-node:latest ^
        --chain qcoin_mainnet_spec.json ^
        --validator ^
        --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd

    echo.
    echo ✅ QCOIN Validator Node is now running in background!
    echo Logs can be viewed with: docker logs -f qcoin-validator
    goto END
)

:: Priority 3: Fallback if native Windows executable exists
if exist "solochain-template-node.exe" (
    echo [OK] Native Windows binary detected!
    echo Starting QCOIN Validator Node...
    solochain-template-node.exe --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd
    goto END
)

echo ❌ Neither WSL2 nor Docker Desktop were detected.
echo.
echo Please install one of the following to run a QCOIN Validator on Windows:
echo   1. WSL2 (Recommended): Open PowerShell as Administrator and run "wsl --install"
echo   2. Docker Desktop for Windows: https://www.docker.com/products/docker-desktop/
echo.

:END
pause
