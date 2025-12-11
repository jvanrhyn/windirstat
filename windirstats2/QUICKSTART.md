# Quick Start Guide - WinDirStats2

Get up and running with WinDirStats2 in minutes!

## Installation

### Option 1: Build from Source (Recommended)

#### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- On Linux: GTK3 development libraries

```bash
# Linux only - install GTK3
sudo apt-get install libgtk-3-dev  # Ubuntu/Debian
sudo dnf install gtk3-devel         # Fedora
sudo pacman -S gtk3                 # Arch

# Build windirstats2
cd windirstats2
cargo build --release

# The binary is now at: target/release/windirstats2
```

### Option 2: Run with Cargo

```bash
cd windirstats2
cargo run --release
```

## First Use

1. **Launch the Application**
   ```bash
   ./target/release/windirstats2
   ```
   (On Windows: `target\release\windirstats2.exe`)

2. **Select a Directory**
   - Click the "Select Directory to Scan" button
   - Or use: File → Select Directory
   - Choose the folder you want to analyze

3. **Wait for Scan**
   - Watch the progress in the status bar
   - Larger directories take longer

4. **Explore Your Disk Usage**
   - **Left Panel** (Directory Tree): Navigate folders
   - **Center Panel** (TreeMap): Visual representation
   - **Right Panel** (Extensions): File type breakdown

## Understanding the TreeMap

- **Size = Area**: Bigger files = bigger rectangles
- **Colors**: Different file types have different colors
- **Click**: Select a file or folder
- **Hover**: See details in tooltip

## Keyboard Shortcuts

Currently available:
- `Ctrl+O` / `Cmd+O`: Open directory (via menu)
- `Alt+F4`: Exit application (Windows)
- `Ctrl+Q`: Quit (Linux/macOS, via menu)

## Tips & Tricks

### Finding Large Files
1. Scan your drive
2. Look for large colored blocks in the TreeMap
3. Click to select and see details
4. Check the directory tree to find the path

### Analyzing Disk Space by Type
1. Check the Extensions panel on the right
2. Sorted by size automatically
3. See which file types take up most space

### Performance
- **First scan**: May take a few minutes for large drives
- **Subsequent scans**: Much faster
- **Large directories**: Be patient, the app is working!

## Common Use Cases

### Find What's Taking Up Space
```
1. Select your home directory or C:\ drive
2. Wait for scan to complete
3. Look for large areas in the TreeMap
4. Click to identify large files/folders
```

### Clean Up Old Downloads
```
1. Scan your Downloads folder
2. Sort extensions by size
3. Find old video/archive files
4. Delete what you don't need
```

### Monitor Project Directories
```
1. Scan your projects folder
2. Find node_modules or build artifacts
3. Clean up unused dependencies
```

## Troubleshooting

### Build Errors

**"GTK3 not found"** (Linux only)
```bash
sudo apt-get install libgtk-3-dev pkg-config
```

**"Rust version too old"**
```bash
rustup update
```

### Runtime Issues

**"Permission Denied"**
- Try running with elevated privileges
- Or scan a directory you have access to

**"Application Crashes"**
- Check available memory
- Try scanning a smaller directory first

**"Very Slow Scan"**
- Normal for very large directories (>1M files)
- Network drives are slower
- System drives may have permission issues

## Platform-Specific Notes

### Windows
- Works on Windows 10/11
- No administrator rights needed (for accessible folders)
- Native file dialogs

### macOS
- Works on macOS 10.15+
- May need to allow in Security & Privacy settings
- Native file dialogs

### Linux
- Tested on Ubuntu 20.04+, Fedora 35+, Arch
- Requires GTK3
- Works on both Wayland and X11

## What's Next?

- Read [README.md](README.md) for full documentation
- Check [BUILD.md](BUILD.md) for detailed build options
- See [COMPARISON.md](COMPARISON.md) for differences from original WinDirStat

## Getting Help

If you encounter issues:
1. Check the documentation
2. Make sure you have the latest Rust version
3. Verify all dependencies are installed
4. Try a smaller test directory first

## Contributing

Found a bug? Have a feature idea?
- Open an issue on GitHub
- Submit a pull request
- Help improve documentation

---

**Happy disk analyzing!** 🎉
