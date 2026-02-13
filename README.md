# mi7soft-daemon

A cross-platform daemon process manager that keeps your services running. Built with Rust (backend) and React + TypeScript (frontend).

## Features

- **Process Management**: Start, stop, restart, and monitor processes automatically
- **Auto-restart**: Automatically restart failed processes
- **PID File Management**: Track running processes with PID files
- **Cross-platform**: Works on Linux (with full daemon support) and Windows
- **Web UI**: Modern React-based web interface for monitoring

## Installation

### Prerequisites

- Rust 1.70+
- Node.js 18+
- pnpm (for frontend)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/mi7soft-daemon.git
cd mi7soft-daemon

# Build backend
cargo build --release

# Install frontend dependencies
cd web && pnpm install && cd ..

# Or build everything
./build_and_test.sh
```

## Usage

### Backend CLI

```bash
# Start the daemon
cargo run --release -- start

# Start the daemon in background (Linux only)
cargo run --release -- start --daemonize

# Start a specific process
cargo run --release -- start-process my-service

# Stop a process
cargo run --release -- stop my-service

# Restart a process
cargo run --release -- restart my-service

# Check status
cargo run --release -- status
cargo run --release -- status my-service

# Shutdown daemon
cargo run --release -- shutdown
```

### Configuration

Default config path: `~/.config/mi7soft-daemon/daemon.toml`

Example configuration:

```toml
[daemon]
pid_file = "/var/run/mi7soft-daemon.pid"
log_file = "/var/log/mi7soft-daemon.log"
check_interval = 5

[[processes]]
name = "my-service"
command = "/usr/bin/my-service"
args = ["--config", "/etc/config.yml"]
working_directory = "/opt/my-service"
auto_restart = true
```

### Web UI

```bash
cd web
pnpm dev
```

Open http://localhost:5173 in your browser.

## Development

```bash
# Backend
cargo build
cargo test
cargo clippy

# Frontend
cd web
pnpm install
pnpm dev
pnpm lint
pnpm check
```

## License

MIT License - see [LICENSE](LICENSE) for details.
