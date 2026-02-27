# mi7soft-daemon

进程守护工具 - 让你的服务持续运行。

## 功能特性

- **进程管理**：启动、停止、重启、监控进程
- **自动拉起**：进程异常退出后自动重启
- **状态持久化**：守护进程退出不影响业务进程，重启后可继续监控
- **PID 文件管理**：通过 PID 文件防止重复启动
- **配置热重载**：支持 SIGHUP 信号重载配置
- **Web UI**：基于 React 的 Web 界面（开发中）

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
max_instances = 1                           # 最大实例数量（可选）

[[processes]]
name = "web-server"
command = "python3"
args = ["-m", "http.server", "8080"]
working_directory = "/var/www"
auto_restart = true
```

### 命令行用法

```bash
# 启动守护进程（管理配置中的所有进程）
./target/release/mi7soft-daemon start

# 后台启动（Linux）
./target/release/mi7soft-daemon --daemonize start

# 指定配置文件
./target/release/mi7soft-daemon -c /path/to/config.toml start

# 启动单个进程
./target/release/mi7soft-daemon start-process my-service

# 停止单个进程
./target/release/mi7soft-daemon stop my-service

# 重启单个进程
./target/release/mi7soft-daemon restart my-service

# 查看状态
./target/release/mi7soft-daemon status              # 查看所有进程
./target/release/mi7soft-daemon status my-service  # 查看指定进程

# 优雅关闭守护进程（不杀死业务进程）
kill $(cat /var/run/mi7soft-daemon.pid)

# 重载配置
kill -HUP $(cat /var/run/mi7soft-daemon.pid)
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
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
          ▲                    ▲
          │                    │
    PID 文件              状态文件
    (锁)                  (.state)
          │                    │
          ▼                    ▼
┌─────────────────────────────────────────┐
│           业务进程                       │
│  my-service  |  web-server  |  ...     │
└─────────────────────────────────────────┘
```

**关键特性**：
- 守护进程与业务进程是**分离**的
- 守护进程退出不会杀死业务进程
- 业务进程变成孤儿进程后由 init/systemd 托管
- 重启守护进程可继续监控

## 常见问题

**Q: 守护进程退出后，业务进程会怎样？**

A: 业务进程继续运行，不受影响。重启守护进程后会接管监控。

**Q: 如何让进程真正脱离父子关系？**

A: 当前实现中，业务进程仍是守护进程的子进程。如需完全独立运行，可在 spawn 时使用 `setsid`。

**Q: 配置文件路径？**

A: 依次查找：1) `-c` 参数指定 2) 当前目录 `daemon.toml` 3) 自动创建默认配置。

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

## 协议

MIT License - 查看 [LICENSE](LICENSE) 文件。
