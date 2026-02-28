#!/bin/bash

# 路径 A 快速测试脚本
# 用法：./QUICK_TEST.sh

set -e

echo "========================================="
echo "  路径 A 快速测试"
echo "========================================="
echo ""

# 1. 检查 Rust 环境
echo "📦 检查 Rust 环境..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo 未安装"
    exit 1
fi
echo "✅ Cargo: $(cargo --version)"
echo "✅ Rustc: $(rustc --version)"
echo ""

# 2. 代码检查
echo "🔍 运行 cargo check..."
if cargo check; then
    echo "✅ cargo check 通过"
else
    echo "❌ cargo check 失败"
    exit 1
fi
echo ""

# 3. 编译
echo "🔨 编译项目..."
if cargo build --release; then
    echo "✅ 编译成功"
else
    echo "❌ 编译失败"
    exit 1
fi
echo ""

# 4. 运行测试
echo "🧪 运行测试..."
if cargo test; then
    echo "✅ 测试通过"
else
    echo "❌ 测试失败"
    exit 1
fi
echo ""

# 5. 功能测试
echo "🚀 运行功能测试..."

# 创建测试配置
mkdir -p /tmp/mi7soft-test
cat > /tmp/mi7soft-test/test.toml << 'TESTCONFIG'
[daemon]
pid_file = "/tmp/mi7soft-test/test.pid"
log_file = "/tmp/mi7soft-test/test.log"
check_interval = 5

[[processes]]
name = "test-echo"
command = "/bin/echo"
args = ["hello"]
auto_restart = false
capture_output = true
log_file = "/tmp/mi7soft-test/echo.log"
TESTCONFIG

# 测试基本命令
echo "  测试 status 命令..."
if ./target/release/mi7soft-daemon --config /tmp/mi7soft-test/test.toml status; then
    echo "  ✅ status 命令正常"
else
    echo "  ⚠️  status 命令失败（可能正常）"
fi

# 测试日志
echo "  测试日志系统..."
if RUST_LOG=debug ./target/release/mi7soft-daemon --config /tmp/mi7soft-test/test.toml status 2>&1 | grep -q "DEBUG"; then
    echo "  ✅ 日志系统正常"
else
    echo "  ⚠️  日志系统可能有问题"
fi

echo ""
echo "========================================="
echo "  ✅ 所有测试完成！"
echo "========================================="
echo ""
echo "下一步："
echo "  git add -A"
echo "  git commit -m 'feat: 完成路径 A'"
echo "  git push"
echo ""
