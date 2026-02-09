@echo off
cd /d "%~dp0"
set WGPU_BACKEND=dx12
"C:\Users\Wili\.cargo\bin\cargo.exe" run --release --bin korangar --features debug
pause
