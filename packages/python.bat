@echo off

for /f "delims=" %%I in ('python --version 3^>^&1') do set "python_version=%%I"

if defined python_version (
    echo [42m[30m%python_version% is already installed.[0m[0m
    where python
    echo.
) else (
    echo Python is not installed.
    exit /b 1
)