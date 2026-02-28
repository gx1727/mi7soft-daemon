# mi7soft-daemon

进程守护工具 - 让你的服务持续运行。

## 功能特性

### 核心功能
- **进程管理**：启动、停止、重启、监控进程
- **自动拉起**：进程异常退出后自动重启
- **状态持久化**：守护进程退出不影响业务进程，重启后可继续监控
- **PID 文件管理**：通过 PID 文件防止重复启动
- **配置热重载**：支持 SIGHUP 信号重载配置
- **Web UI**：基于 React 的 Web 界面（开发中）

### 🆕 路径 A 新增功能（v0.1.2+）

#### 1. 统一日志系统
- **结构化日志**：使用 tracing 框架，支持 JSON 格式
- **日志级别控制**：通过环境变量动态调整（RUST_LOG）
- **模块级别过滤**：可针对不同模块设置不同日志级别
- **日志轮转**：支持按日期自动轮转日志文件

#### 2. 进程输出捕获
- **stdout/stderr 捕获**：自动捕获进程的标准输出和错误输出
- **日志存储**：将进程输出保存到指定文件
- **实时查看**：支持实时跟踪进程日志（类似 tail -f）
- **历史查询**：可查看最近 N 行或指定时间范围内的日志
- **文件大小限制**：支持设置最大日志文件大小

#### 3. 持久化存储
- **SQLite 数据库**：使用 SQLite 存储进程历史和统计信息
- **历史记录**：记录进程的每次启动、停止、重启
- **统计信息**：统计进程的总启动次数、失败次数、平均运行时间
- **自动清理**：支持自动清理旧的历史记录

## 编译安装

### 环境要求

- Rust 1.70+
- Node.js 18+ (前端开发)
- pnpm (前端包管理)

### 编译步骤

```bash
# 克隆项目
git clone https://github.com/gx1727/mi7soft-daemon.git
cd mi7soft-daemon

# 编译后端
cargo build --release

# 编译前端（可选）
cd web && pnpm install && cd ..
```

## 使用手册

### 配置文件

默认配置路径：`./daemon.toml`（当前目录）

示例配置：

```toml
# 守护进程设置
[daemon]
pid_file = "/var/run/mi7soft-daemon.pid"  # PID 文件路径
log_file = "/var/log/mi7soft-daemon.log"   # 日志文件路径
check_interval = 3                          # 进程检查间隔（秒）

# 要管理的进程列表
[[processes]]
name = "my-service"                         # 进程名称（唯一标识）
command = "/usr/bin/my-service"             # 要执行的命令
args = ["--config", "/etc/config.yml"]      # 命令参数
working_directory = "/opt/my-service"        # 工作目录
auto_restart = true                         # 进程退出后自动重启
capture_output = true                       # 🆕 捕获进程输出
log_file = "/var/log/my-service.log"        # 🆕 进程日志文件
max_log_size = 10485760                     # 🆕 最大日志文件大小（字节，可选）

[[processes]]
name = "web-server"
command = "python3"
args = ["-m", "http.server", "8080"]
working_directory = "/var/www"
auto_restart = true
capture_output = true
log_file = "/var/log/web-server.log"
```

### 命令行用法

#### 基本命令

```bash
# 启动守护进程（管理配置中的所有进程）
./target/release/m7d start

# 后台启动（Linux）
./target/release/m7d --daemonize start

# 指定配置文件
./target/release/m7d -c /path/to/config.toml start

# 启动单个进程
./target/release/m7d start-process my-service

# 停止单个进程
./target/release/m7d stop my-service

# 重启单个进程
./target/release/m7d restart my-service

# 查看状态
./target/release/m7d status              # 查看所有进程
./target/release/m7d status my-service  # 查看指定进程

# 优雅关闭守护进程（不杀死业务进程）
./target/release/m7d shutdown

# 或者使用 kill
kill $(cat /var/run/mi7soft-daemon.pid)

# 重载配置
kill -HUP $(cat /var/run/mi7soft-daemon.pid)
```

#### 🆕 日志系统

**设置日志级别：**

```bash
# 默认 info 级别
./target/release/m7d start

# debug 级别（更详细）
RUST_LOG=debug ./target/release/m7d start

# trace 级别（最详细）
RUST_LOG=trace ./target/release/m7d start

# 只看特定模块
RUST_LOG=mi7soft_daemon::process=debug ./target/release/m7d start

# 多个模块
RUST_LOG=mi7soft_daemon::daemon=debug,mi7soft_daemon::process=trace ./target/release/m7d start
```

**日志文件位置：**
- 守护进程日志：`/var/log/mi7soft-daemon.log`
- 进程日志：配置文件中指定的 `log_file` 路径

#### 🆕 进程日志查看

**查看进程日志：**

```bash
# 查看进程日志（默认最后 100 行）
./target/release/m7d logs my-service

# 查看最后 50 行
./target/release/m7d logs my-service --lines 50

# 查看最后 200 行
./target/release/m7d logs my-service -n 200

# 实时跟踪日志（类似 tail -f）
./target/release/m7d logs my-service --follow

# 查看最近 1 小时的日志（3600 秒）
./target/release/m7d logs my-service --since 3600

# 查看最近 30 分钟的日志
./target/release/m7d logs my-service --since 1800
```

**日志格式：**
```
[2026-02-28 21:00:00] [OUT] 进程标准输出内容
[2026-02-28 21:00:01] [ERR] 进程错误输出内容
```

#### 🆕 历史记录查看

**查看进程历史：**

```bash
# 查看进程历史（默认最后 10 条）
./target/release/m7d history my-service

# 查看最后 20 条记录
./target/release/m7d history my-service --number 20

# 查看最后 50 条记录
./target/release/m7d history my-service -n 50
```

**历史记录格式：**
```
History for process my-service (last 10 records):
--------------------------------------------------------------------------------
  PID 1234   | 2026-02-28 21:00:00 - 2026-02-28 21:30:00 | 1800s        | ✓ Success
  PID 5678   | 2026-02-28 20:00:00 - 2026-02-28 20:45:00 | 2700s        | ✗ Failed (code: 1)
  PID 9012   | 2026-02-28 19:00:00 - 2026-02-28 19:15:00 | 900s         | Running...
```

**历史记录包含：**
- 进程 PID
- 启动时间
- 结束时间（如果已结束）
- 运行时长
- 退出状态（成功/失败/运行中）

#### 🆕 详细模式

```bash
# 详细输出（-v）
./target/release/m7d -v status

# 更详细的输出（-vv）
./target/release/m7d -vv status

# 最详细的输出（-vvv）
./target/release/m7d -vvv status
```

### 状态持久化

守护进程会自动保存状态到 `.state` 文件（与 PID 文件同目录）：

- 守护进程正常/异常退出后，业务进程继续运行
- 重新启动守护进程，会自动接管仍在运行的业务进程
- 守护进程会定期检查进程状态，清理已退出的进程

### 信号说明

| 信号 | 作用 |
|------|------|
| SIGTERM | 优雅关闭守护进程，保存状态但不杀死业务进程 |
| SIGHUP | 重载配置文件 |

## 工作原理

```
┌─────────────────────────────────────────┐
│           mi7soft-daemon                │
│  ┌─────────────────────────────────┐    │
│  │     ProcessManager              │    │
│  │  - 监控进程状态                 │    │
│  │  - 自动重启失败进程             │    │
│  │  - 持久化进程信息               │    │
│  │  - 捕获进程输出 🆕              │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │     Storage (SQLite) 🆕         │    │
│  │  - 记录进程历史                 │    │
│  │  - 统计进程信息                 │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
          ▲                    ▲
          │                    │
    PID 文件              状态文件 + 数据库
    (锁)                  (.state + .db)
          │                    │
          ▼                    ▼
┌─────────────────────────────────────────┐
│           业务进程                       │
│  my-service  |  web-server  |  ...     │
│    ↓ stdout/stderr → 日志文件 🆕        │
└─────────────────────────────────────────┘
```

**关键特性**：
- 守护进程与业务进程是**分离**的
- 守护进程退出不会杀死业务进程
- 业务进程变成孤儿进程后由 init/systemd 托管
- 重启守护进程可继续监控
- 🆕 自动捕获进程输出到日志文件
- 🆕 持久化存储进程历史和统计

## 数据存储

### 文件位置

| 文件类型 | 路径 | 说明 |
|---------|------|------|
| PID 文件 | `/var/run/mi7soft-daemon.pid` | 进程锁 |
| 状态文件 | `/var/run/mi7soft-daemon.state` | 进程状态持久化 |
| 数据库 | `~/.local/share/mi7soft-daemon/daemon.db` | 🆕 SQLite 数据库 |
| 守护进程日志 | `/var/log/mi7soft-daemon.log` | 守护进程日志 |
| 进程日志 | 配置文件中指定 | 🆕 各进程的输出日志 |

### 数据库表结构

**process_history 表：**
- 记录每次进程启动/停止
- 包含：PID、时间戳、退出码、运行时长

**process_stats 表：**
- 统计进程信息
- 包含：总启动次数、失败次数、平均运行时间

## 常见问题

**Q: 守护进程退出后，业务进程会怎样？**

A: 业务进程继续运行，不受影响。重启守护进程后会接管监控。

**Q: 如何让进程真正脱离父子关系？**

A: 当前实现中，业务进程仍是守护进程的子进程。如需完全独立运行，可在 spawn 时使用 `setsid`。

**Q: 配置文件路径？**

A: 依次查找：1) `-c` 参数指定 2) 当前目录 `daemon.toml` 3) 自动创建默认配置。

**Q: 🆕 如何查看进程的实时日志？**

A: 使用 `m7d logs <name> --follow` 命令实时跟踪进程输出。

**Q: 🆕 如何查看进程的历史运行记录？**

A: 使用 `m7d history <name>` 命令查看进程的启动、停止历史。

**Q: 🆕 日志文件会无限增长吗？**

A: 可以在配置中设置 `max_log_size` 限制日志文件大小（未来版本将支持自动轮转）。

**Q: 🆕 数据库文件在哪里？**

A: 默认在 `~/.local/share/mi7soft-daemon/daemon.db`。

## 开发相关

```bash
# 开发模式编译
cargo build

# 运行测试
cargo test

# 代码检查
cargo clippy

# 前端开发
cd web
pnpm install
pnpm dev
```

## 更新日志

### v0.1.2 (2026-02-28)
- 🆕 统一日志系统（tracing 框架）
- 🆕 进程输出捕获（logs 命令）
- 🆕 持久化存储（SQLite + history 命令）
- 🆕 结构化日志
- 🆕 日志级别控制

### v0.1.1
- 状态持久化
- 二进制重命名为 m7d
- GitHub Actions 自动部署

### v0.1.0
- 初始版本
- 基础进程管理功能

## 贡献

欢迎提交 Issue 和 Pull Request！

## 协议

MIT License - 查看 [LICENSE](LICENSE) 文件。

---

**文档版本：** v0.1.2  
**最后更新：** 2026-02-28  
**维护者：** gx1727 + 星尘 (OpenClaw AI Assistant)
