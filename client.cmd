@echo off
set PATH=C:\vcpkg\installed\x64-windows\bin;%PATH%
cd target\release\
if not exist qml mkdir qml
xcopy /E /I /Y ..\..\client\qml qml\ >nul
continuum-client.exe
cd ..\..\