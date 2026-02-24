@echo off
chcp 65001 >nul
echo ==========================================
echo RMD - Rust Markdown Editor Build Script
echo ==========================================
echo.

:: Check if Rust/Cargo is installed
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未检测到 Rust/Cargo 安装
    echo.
    echo 请先安装 Rust:
    echo   1. 访问 https://rustup.rs/
    echo   2. 下载并运行 rustup-init.exe
    echo   3. 重启终端后重试
    echo.
    pause
    exit /b 1
)

echo [信息] 检测到:
for /f "tokens=*" %%a in ('cargo --version') do echo        %%a
for /f "tokens=*" %%a in ('rustc --version') do echo        %%a
echo.

:: Parse arguments
set BUILD_TYPE=debug
set RUN_AFTER_BUILD=false

:parse_args
if "%1"=="" goto :done_parse
if /i "%1"=="release" set BUILD_TYPE=release
if /i "%1"=="run" set RUN_AFTER_BUILD=true
if /i "%1"=="clean" goto :clean
shift
goto :parse_args
:done_parse

:: Clean build
:clean
if /i "%1"=="clean" (
    echo [信息] 清理构建目录...
    cargo clean
    if exist "target" rmdir /s /q "target"
    echo [成功] 清理完成
    echo.
    pause
    exit /b 0
)

:: Build
echo [信息] 开始构建 (%BUILD_TYPE% 模式)...
echo.

if "%BUILD_TYPE%"=="release" (
    cargo build --release
) else (
    cargo build
)

if errorlevel 1 (
    echo.
    echo [错误] 构建失败！
    echo.
    pause
    exit /b 1
)

echo.
echo [成功] 构建完成！
echo.

:: Copy executable to project root for convenience
if "%BUILD_TYPE%"=="release" (
    if exist "target\release\rmd.exe" (
        copy /y "target\release\rmd.exe" "rmd.exe" >nul
        echo [信息] 已复制可执行文件到项目根目录
    )
) else (
    if exist "target\debug\rmd.exe" (
        copy /y "target\debug\rmd.exe" "rmd-debug.exe" >nul
        echo [信息] 已复制调试可执行文件到项目根目录
    )
)

echo.

:: Run if requested
if "%RUN_AFTER_BUILD%"=="true" (
    echo [信息] 启动 RMD...
    echo ==========================================
    echo.
    if "%BUILD_TYPE%"=="release" (
        .\rmd.exe
    ) else (
        .\rmd-debug.exe
    )
)

echo.
pause
