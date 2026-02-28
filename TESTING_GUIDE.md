# 路径 A 测试指南

**测试目标：** 验证路径 A 的所有代码修改是否正确  
**测试环境：** 需要 Rust 编译环境（rustc 1.70+）  
**预计时间：** 10-20 分钟  

---

## 📋 测试前准备

### 1. 检查环境

```bash
# 检查 Rust 版本
rustc --version
cargo --version

# 确保版本 >= 1.70
```

### 2. 进入项目目录

```bash
cd /path/to/mi7soft-daemon
```

### 3. 检查文件完整性

```bash
# 应该看到以下新文件
ls -la src/process_output.rs
ls -la src/storage.rs
ls -la CHANGELOG.md
ls -la PATH_A_SUMMARY.md

# 检查备份文件
ls -la *.backup
ls -la src/*.backup
```

---

## 🔍 测试步骤

### 第一步：代码检查（2-3 分钟）

```bash
# 1. 检查依赖是否正确
cargo check

# 预期输出：
# Checking mi7soft-daemon v0.1.0
# Finished dev [unoptimized + debuginfo] target(s) in XX.XXs

# 如果有错误，记录错误信息
```

**可能的问题：**

**问题 1：依赖下载超时**
```bash
# 解决方案：使用国内镜像
# 在 ~/.cargo/config 添加：
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

**问题 2：依赖版本冲突**
```bash
# 解决方案：更新依赖
cargo update
cargo check
```

**问题 3：编译错误**
- 记录错误信息
- 检查对应的 .patch 文件
- 手动集成代码

---

### 第二步：编译测试（3-5 分钟）

```bash
# 1. Debug 编译（快速）
cargo build

# 2. Release 编译（优化）
cargo build --release

# 预期输出：
# Compiling mi7soft-daemon v0.1.0
# Finished release [optimized] target(s) in XX.XXs
```

**检查编译产物：**
```bash
# Debug 版本
ls -lh target/debug/mi7soft-daemon

# Release 版本
ls -lh target/release/mi7soft-daemon
```

---

### 第三步：单元测试（1-2 分钟）

```bash
# 运行所有测试
cargo test

# 预期输出：
# running XX tests
# test result: ok. XX passed; 0 failed; 0 ignored
```

**如果测试失败：**
```bash
# 查看详细输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_storage_basic
cargo test test_output_capture
```

---

### 第四步：功能测试（5-10 分钟）

#### 4.1 准备测试配置

```bash
# 创建测试配置
mkdir -p config
cat > config/test.toml << 'TESTCONFIG'
[daemon]
pid_file = "/tmp/mi7soft-test.pid"
log_file = "/tmp/mi7soft-test.log"
check_interval = 5

[[processes]]
name = "test-sleep"
command = "/bin/sleep"
args = ["100"]
auto_restart = false
capture_output = true
log_file = "/tmp/test-sleep.log"
TESTCONFIG

# 创建日志目录
mkdir -p /tmp/mi7soft-logs
```

#### 4.2 测试日志系统

```bash
# 1. 测试默认日志级别
./target/release/mi7soft-daemon --config config/test.toml status

# 2. 测试 debug 级别
RUST_LOG=debug ./target/release/mi7soft-daemon --config config/test.toml status

# 3. 测试 trace 级别
RUST_LOG=trace ./target/release/mi7soft-daemon --config config/test.toml status

# 预期：应该看到不同详细程度的日志输出
```

#### 4.3 测试进程输出捕获

```bash
# 1. 启动测试进程
./target/release/mi7soft-daemon --config config/test.toml start-process test-sleep

# 2. 检查日志文件
ls -la /tmp/test-sleep.log
cat /tmp/test-sleep.log

# 3. 查看进程日志（如果 logs 命令已集成）
./target/release/mi7soft-daemon --config config/test.toml logs test-sleep

# 4. 停止进程
./target/release/mi7soft-daemon --config config/test.toml stop test-sleep
```

#### 4.4 测试持久化存储

```bash
# 1. 检查数据库文件
ls -la ~/.local/share/mi7soft-daemon/daemon.db

# 2. 查看历史（如果 history 命令已集成）
./target/release/mi7soft-daemon --config config/test.toml history test-sleep

# 3. 查看统计（如果 stats 命令已集成）
./target/release/mi7soft-daemon --config config/test.toml stats test-sleep
```

---

## ✅ 测试检查清单

### 编译测试
- [ ] `cargo check` 通过
- [ ] `cargo build` 成功
- [ ] `cargo build --release` 成功
- [ ] `cargo test` 通过

### 功能测试
- [ ] 日志系统正常工作
- [ ] RUST_LOG 环境变量生效
- [ ] 进程可以启动
- [ ] 进程输出被捕获
- [ ] 日志文件正确生成
- [ ] 数据库文件正确生成
- [ ] history 命令正常（如果已集成）
- [ ] stats 命令正常（如果已集成）

### 代码质量
- [ ] 没有编译警告
- [ ] 没有运行时错误
- [ ] 日志输出格式正确
- [ ] 配置文件正确解析

---

## 🐛 问题排查

### 问题 1：找不到模块

**错误：**
```
error[E0588]: cannot find type `OutputCapture` in this scope
```

**原因：** process_output.rs 未被正确引入

**解决方案：**
```rust
// 在 src/lib.rs 或 src/main.rs 添加：
mod process_output;
mod storage;
```

---

### 问题 2：数据库初始化失败

**错误：**
```
Failed to open log file: Permission denied
```

**原因：** 目录权限问题

**解决方案：**
```bash
# 创建数据目录
mkdir -p ~/.local/share/mi7soft-daemon
chmod 755 ~/.local/share/mi7soft-daemon
```

---

### 问题 3：依赖冲突

**错误：**
```
error: multiple versions of package `xxx`
```

**解决方案：**
```bash
# 清理并更新
cargo clean
cargo update
cargo build
```

---

## 📊 测试结果报告模板

```markdown
# 路径 A 测试结果

**测试时间：** YYYY-MM-DD HH:MM
**测试环境：**
- OS: 
- Rust: 
- Cargo: 

### 测试结果

#### 编译测试
- [ ] cargo check: ✅ / ❌
- [ ] cargo build: ✅ / ❌
- [ ] cargo build --release: ✅ / ❌
- [ ] cargo test: ✅ / ❌

#### 功能测试
- [ ] 日志系统: ✅ / ❌
- [ ] 进程输出捕获: ✅ / ❌
- [ ] 持久化存储: ✅ / ❌

### 遇到的问题

1. 问题：xxx
   解决：xxx

2. 问题：xxx
   解决：xxx

### 总体评价

- [ ] 所有测试通过，可以提交
- [ ] 有小问题，需要修复
- [ ] 有严重问题，需要重新设计
```

---

## 🚀 测试通过后的步骤

### 1. 提交代码

```bash
git add -A
git commit -m "feat: 完成路径 A 核心功能增强

- 统一日志系统（tracing）
- 进程输出捕获（logs 命令）
- 持久化存储（SQLite）
- 新增 history/stats 命令

测试：
- cargo check: ✅
- cargo build: ✅
- cargo test: ✅
- 功能测试: ✅

新增代码 ~600 行，修改 ~100 行"

git push origin master
```

### 2. 更新项目记忆

```bash
# 更新 memory/projects/mi7soft-daemon.md
# 添加测试结果
```

### 3. 通知用户

```bash
# 发送测试报告
# 包含测试结果和提交信息
```

---

## 📞 需要帮助？

如果在测试过程中遇到问题：

1. 记录完整的错误信息
2. 记录你的环境信息（OS、Rust 版本）
3. 提供重现步骤
4. 联系星尘（OpenClaw AI Assistant）

---

**测试愉快！** 🚀

*Generated by 星尘 (OpenClaw AI Assistant)*
*Last updated: 2026-02-28 21:18*
