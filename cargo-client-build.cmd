@echo off
cargo build --release -p continuum-client
if errorlevel 1 exit /b 1
echo.
echo Build complete: target\release\continuum-client.exe