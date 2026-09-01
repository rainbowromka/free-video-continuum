@echo off
cargo build --release -p continuum-cli
if errorlevel 1 exit /b 1
echo.
echo Build complete: target\release\continuum-cli.exe