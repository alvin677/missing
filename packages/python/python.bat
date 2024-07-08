@echo off

for /f "delims=" %%I in ('python --version 3^>^&1') do set "version=%%I"

if defined version (
    echo [42m[30m%version% is already installed.[0m[0m
    where python
    echo.
) else (
    curl -o C:/Users/%username%/.missing/python-installer.exe https://www.python.org/ftp/python/{VERSION}/python-{VERSION}-amd64.exe
    C:/Users/%username%/.missing/python-installer.exe
)