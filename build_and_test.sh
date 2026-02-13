#!/bin/bash

set -e

echo "===================================="
echo "mi7soft-daemon 编译和测试脚本"
echo "===================================="
echo

# 进入项目目录
cd /root/mi7soft-daemon

echo "[1/6] 检查 Rust 环境..."
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust 未安装！"
    echo "请运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
    exit 1
fi
echo "Rust 已安装: $(cargo --version)"
echo

echo "[2/6] 清理旧的构建产物..."
cargo clean
echo

echo "[3/6] 编译项目 (Release 模式)..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "[ERROR] 编译失败！"
    exit 1
fi
echo "编译成功！"
echo

echo "[4/6] 测试 CLI 命令..."
echo ""
echo "----------------------------------------"
echo "测试: --help"
echo "----------------------------------------"
/root/mi7soft-daemon/target/release/mi7soft-daemon --help
echo

echo ""
echo "===================================="
echo "编译完成！"
echo "可执行文件: /root/mi7soft-daemon/target/release/mi7soft-daemon"
echo ""
echo "下一步："
echo "  1. 创建配置文件:"
echo "     mkdir -p ~/.config/mi7soft-daemon"
echo "     cp config/daemon.toml ~/.config/mi7soft-daemon/"
echo "  2. 编辑配置: vi ~/.config/mi7soft-daemon/daemon.toml"
echo "  3. 测试守护进程:"
echo "     /root/mi7soft-daemon/target/release/mi7soft-daemon --daemonize start"
echo "===================================="
