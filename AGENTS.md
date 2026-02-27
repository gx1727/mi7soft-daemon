# AGENTS.md - mi7soft-daemon Development Guide

## Project Overview

Hybrid Rust + React/TypeScript project:
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
- 4-space indentation, `cargo fmt` before committing
- Max line length: 100 characters

**Naming Conventions:**
- Types: `PascalCase` (e.g., `DaemonError`, `ProcessConfig`)
- Functions/Variables: `snake_case` (e.g., `run_daemon`, `config_path`)
- Constants: `SCREAMING_SNAKE_CASE`

**Imports:**
- Group imports: std → external → internal
- Use absolute paths for project modules (`crate::`, `super::`)

**Error Handling:**
- Use `thiserror` for custom error types (see `src/error.rs`)
- Return `Result<T, DaemonError>` for fallible functions
- Use `?` operator for propagation, include context in messages

**Testing:**
- Place tests in same file using `#[cfg(test)]` and `#[test]`
- Follow patterns in `src/error.rs` and `src/cli.rs`

---

### Frontend (React/TypeScript)

**Formatting:**
- ESLint + Prettier, run `pnpm lint` before commit
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
- Custom colors from `tailwind.config.js`: `background`, `primary`, `accent`, `surface`
- Use `clsx` + `tailwind-merge` (`cn()` utility) for conditional classes

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
│   └── src/
│       ├── App.tsx        # Root component
│       ├── components/    # React components
│       ├── pages/         # Page components
│       ├── hooks/         # Custom hooks
│       ├── lib/           # Utilities & stores
│       └── locales/       # i18n translations
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
- **Config path:** Defaults to `~/.config/mi7soft-daemon/daemon.toml` or current directory `daemon.toml`
