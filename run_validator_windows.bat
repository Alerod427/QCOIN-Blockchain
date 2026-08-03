@echo off
TITLE QCOIN Post-Quantum Blockchain - Windows Validator Node Launcher
COLOR 0A
cls

echo ==============================================================================
echo    🌐 QCOIN POST-QUANTUM BLOCKCHAIN - WINDOWS VALIDATOR LAUNCHER
echo ==============================================================================
echo.

echo Checking environment...

:: Check if Docker is installed
docker --version >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] Docker Desktop detected on Windows!
    echo Starting QCOIN Validator Node via Docker...
    docker run -d --name qcoin-validator ^
        -p 30333:30333 -p 9944:9944 ^
        -v qcoin_data:/data ^
        alerod427/qcoin-node:latest ^
        --chain /data/qcoin_mainnet_spec.json ^
        --validator ^
        --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd
    echo.
    echo ✅ QCOIN Validator Node is now running in background!
    echo Logs can be viewed with: docker logs -f qcoin-validator
    goto END
)

:: Check if WSL is installed
wsl --status >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] Windows Subsystem for Linux (WSL) detected!
    echo Launching QCOIN Validator Node inside WSL...
    wsl bash -c "./start_qcoin_all.sh"
    goto END
)

:: Fallback if native executable exists
if exist "solochain-template-node.exe" (
    echo [OK] Native Windows binary detected!
    echo Starting QCOIN Validator Node...
    solochain-template-node.exe --chain qcoin_mainnet_spec.json --validator --bootnodes /ip4/158.179.211.45/tcp/30333/p2p/12D3KooWFgJgGEuBGfGpojUZv2bUavYhC5mgURuuL44T31m8cPFd
    goto END
)

echo ❌ Docker or WSL2 not detected.
echo.
echo Please install one of the following to run a QCOIN Validator on Windows:
echo   1. Docker Desktop for Windows: https://www.docker.com/products/docker-desktop/
echo   2. WSL2 (Windows Subsystem for Linux): Open PowerShell and run "wsl --install"
echo.

:END
pause
