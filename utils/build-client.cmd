@echo off
cd ..
cargo build --release -p continuum-client
if errorlevel 1 (
    cd utils
    exit /b 1
)
echo.
echo Build complete: target\release\continuum-client.exe
cd utils