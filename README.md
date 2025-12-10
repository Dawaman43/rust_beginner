# lesson_1 — Rust basics for beginners

This repository contains very small, focused Rust examples intended for people who are new to Rust. Each example demonstrates one concept and is heavily commented for clarity.

Quickstart

1. Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build the project:

```bash
cargo build
```

3. Run an example (replace `<name>` with a bin file name, e.g. `demo`):

```bash
cargo run --bin <name>
```

Structure

- `src/bin/` — contains small example binaries, one concept per file.
- `Cargo.toml` — project configuration.

