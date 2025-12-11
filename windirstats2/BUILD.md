# Building WinDirStats2

## Prerequisites

- **Rust**: Version 1.70 or later (2021 edition)
  - Install from [rust-lang.org](https://www.rust-lang.org/tools/install)
  - Or use `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- **System Dependencies**:
  - **Linux**: GTK3 development libraries
    ```bash
    # Ubuntu/Debian
    sudo apt-get install libgtk-3-dev
    
    # Fedora
    sudo dnf install gtk3-devel
    
    # Arch
    sudo pacman -S gtk3
    ```
  - **macOS**: No additional dependencies
  - **Windows**: No additional dependencies

## Building

### Debug Build

```bash
cd windirstats2
cargo build
```

The binary will be in `target/debug/windirstats2` (or `.exe` on Windows).

### Release Build (Recommended)

```bash
cd windirstats2
cargo build --release
```

The optimized binary will be in `target/release/windirstats2` (or `.exe` on Windows).

### Run Directly

```bash
cd windirstats2
cargo run --release
```

## Build Options

### Optimized Native Build

For best performance on your specific CPU:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Cross-Compilation

To build for a different target:

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Code Quality

### Check for Errors

```bash
cargo check
```

### Lint with Clippy

```bash
cargo clippy
```

### Format Code

```bash
cargo fmt
```

Check formatting without changing files:

```bash
cargo fmt -- --check
```

## Troubleshooting

### GTK3 Errors on Linux

If you get errors about missing GTK3:
```bash
sudo apt-get install libgtk-3-dev pkg-config
```

### Slow Compilation

First build can be slow due to dependencies. Subsequent builds are much faster.

To speed up builds:
- Use `cargo build` (debug mode) during development
- Use `cargo check` for quick error checking
- Enable parallel compilation: Set `CARGO_BUILD_JOBS` environment variable

### Out of Memory During Build

If compilation fails with OOM:
```bash
# Reduce parallel jobs
cargo build -j 2
```

## Platform-Specific Notes

### Windows

- No special requirements
- MSVC or GNU toolchain both work
- Recommended: Use MSVC toolchain for better compatibility

### macOS

- Works on macOS 10.15+
- No special requirements
- May need Xcode Command Line Tools: `xcode-select --install`

### Linux

- Requires GTK3 development libraries
- Tested on Ubuntu 20.04+, Fedora 35+, Arch Linux
- Wayland and X11 both supported

## Development Environment

### VS Code

Recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- crates

### CLion / IntelliJ IDEA

Use the Rust plugin.

### Debugging

```bash
# Run with debug symbols
cargo build
RUST_BACKTRACE=1 ./target/debug/windirstats2

# Full backtrace
RUST_BACKTRACE=full ./target/debug/windirstats2
```

## Distribution

### Creating a Release

1. Build release binary:
   ```bash
   cargo build --release
   ```

2. Strip symbols (optional, reduces size):
   ```bash
   strip target/release/windirstats2
   ```

3. Binary is standalone and can be distributed

### Installation

Copy the binary to a location in your PATH:

```bash
# Linux/macOS
sudo cp target/release/windirstats2 /usr/local/bin/

# Or user-local
mkdir -p ~/.local/bin
cp target/release/windirstats2 ~/.local/bin/
```

On Windows, copy to any directory and add it to PATH, or run directly.
