# AGENTS.md - mi7soft-daemon Development Guide

## Project Overview

This is a hybrid Rust + React/TypeScript project consisting of:
- **Backend**: Rust daemon process manager (`src/*.rs`)
- **Frontend**: React + TypeScript + Vite + TailwindCSS (`web/`)

---

## Build Commands

### Backend (Rust)

```bash
# Build
cargo build

# Build release
cargo build --release

# Run
cargo run -- [args]

# Run tests
cargo test

# Run single test
cargo test test_name

# Lint
cargo clippy

# Format
cargo fmt
```

### Frontend (React/TypeScript)

```bash
# Install dependencies (uses pnpm)
cd web && pnpm install

# Development server
pnpm dev

# Build for production
pnpm build

# Lint
pnpm lint

# Type check
pnpm check

# Preview production build
pnpm preview
```

---

## Code Style Guidelines

### General Rules

1. **Never use `as any` or type suppression** - Fix type errors properly
2. **Never leave empty catch blocks** - Handle errors appropriately
3. **Never commit without verifying** - Run lint/type-check before commit
4. **Never delete tests to pass** - Fix the code, not the tests

---

### Rust Backend

**Formatting:**
- 4-space indentation
- Use `rustfmt.toml` defaults (run `cargo fmt` before committing)
- Max line length: 100 characters

**Naming Conventions:**
- Types: `PascalCase` (e.g., `DaemonError`, `ProcessConfig`)
- Functions: `snake_case` (e.g., `run_daemon`, `stop_process`)
- Variables: `snake_case` (e.g., `config_path`, `pid_file`)
- Constants: `SCREAMING_SNAKE_CASE`

**Imports:**
- Group imports: std → external → internal
- Use absolute paths for project modules (`crate::`, `super::`)

**Error Handling:**
- Use `thiserror` for custom error types (see `src/error.rs`)
- Always return `Result<T, DaemonError>` for fallible functions
- Use `?` operator for propagation
- Include context in error messages

**Code Example:**
```rust
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Process '{name}' is already running (PID: {pid})")]
    AlreadyRunning { name: String, pid: u32 },
}

impl DaemonError {
    pub fn exit_code(&self) -> i32 {
        match self {
            DaemonError::Config(_) => 78,
            DaemonError::AlreadyRunning { .. } => 1,
        }
    }
}
```

**Testing:**
- Place tests in same file using `#[cfg(test)]` and `#[test]`
- Follow the pattern in `src/error.rs` and `src/cli.rs`

---

### Frontend (React/TypeScript)

**Formatting:**
- ESLint + Prettier compatible (run `pnpm lint` before commit)
- 2-space indentation in TSX/TS files
- TailwindCSS for styling

**TypeScript Config:**
- Strict mode: OFF (see `tsconfig.json`)
- Path aliases: `@/*` maps to `./src/*`

**Naming Conventions:**
- Components: `PascalCase` (e.g., `Layout.tsx`, `Home.tsx`)
- Hooks: `camelCase` starting with `use` (e.g., `useAuth.ts`)
- Utils: `camelCase` (e.g., `cn.ts`)
- Types/Interfaces: `PascalCase` (e.g., `ProcessStatus`)

**Imports:**
- Order: React imports → external libs → internal components/hooks → utils → types
- Use path aliases: `import Button from '@/components/ui/Button'`

**State Management:**
- Use `zustand` for global state (see `web/src/lib/` for stores)
- Use `react-hook-form` for form handling

**TailwindCSS:**
- Use custom colors from `tailwind.config.js`:
  - `background`, `primary`, `accent`, `surface`
- Use `clsx` + `tailwind-merge` (`cn()` utility) for conditional classes

**Code Example:**
```tsx
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

function cn(...inputs: (string | undefined | null | false)[]) {
  return twMerge(clsx(inputs));
}

interface ButtonProps {
  variant?: 'primary' | 'secondary';
  className?: string;
}

export function Button({ variant = 'primary', className }: ButtonProps) {
  return (
    <button
      className={cn(
        'px-4 py-2 rounded',
        variant === 'primary' && 'bg-blue-600 text-white',
        className
      )}
    >
      Click me
    </button>
  );
}
```

**i18n:**
- Use `i18next` with `react-i18next`
- Translation files in `web/src/locales/`
- Keys follow `page.section.subsection` pattern

---

## Project Structure

```
mi7soft-daemon/
├── Cargo.toml              # Rust dependencies
├── src/
│   ├── main.rs            # Entry point
│   ├── cli.rs             # Command-line interface (clap)
│   ├── config.rs          # Configuration loading
│   ├── daemon.rs          # Daemon logic
│   ├── error.rs           # Error types
│   ├── pidfile.rs         # PID file management
│   ├── process.rs         # Process management
│   └── signal.rs          # Signal handling
├── web/                   # React frontend
│   ├── package.json       # Node dependencies (pnpm)
│   ├── tsconfig.json      # TypeScript config
│   ├── eslint.config.js   # ESLint config
│   ├── tailwind.config.js # Tailwind config
│   ├── vite.config.ts     # Vite config
│   └── src/
│       ├── App.tsx        # Root component
│       ├── components/    # React components
│       ├── pages/         # Page components
│       ├── hooks/         # Custom hooks
│       ├── lib/           # Utilities & stores
│       ├── locales/       # i18n translations
│       └── i18n/          # i18n config
```

---

## Development Workflow

1. **Before writing code:**
   - Run `pnpm lint` (frontend) or `cargo clippy` (backend)
   - Run `pnpm check` (frontend) or `cargo check` (backend)

2. **Testing:**
   - Backend: `cargo test` - runs all unit tests in `src/`
   - Frontend: No test framework configured yet

3. **Committing:**
   - Format code: `cargo fmt` (Rust) / let ESLint handle JS
   - Lint: `pnpm lint` / `cargo clippy`
   - Type-check: `pnpm check` / `cargo check`

---

## Common Issues

- **Windows:** Daemon mode not supported (falls back to error)
- **PID files:** Unix: `/var/run/mi7soft-daemon.pid`, Windows: `mi7soft-daemon.pid`
- **Config path:** Defaults to `~/.config/mi7soft-daemon/daemon.toml`
