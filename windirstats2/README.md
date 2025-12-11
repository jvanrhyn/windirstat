# WinDirStats2 - Cross-Platform Disk Usage Analyzer

A modern, cross-platform reimplementation of WinDirStat in Rust, featuring a beautiful UI and treemap visualization.

## Description

WinDirStats2 is a disk usage analyzer that helps you visualize where your storage space is being used. It provides:

- **Directory Tree View**: Navigate through your file system hierarchically
- **TreeMap Visualization**: See file sizes represented as colored rectangles - bigger files get bigger rectangles
- **Extension Statistics**: View file type breakdown with counts, sizes, and percentages
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Features

### Core Features
- 🗂️ **Directory Scanning**: Fast, efficient directory tree scanning
- 🎨 **TreeMap Visualization**: Interactive squarified treemap layout
- 📊 **Extension Statistics**: Comprehensive file type analysis
- 🎯 **Interactive Selection**: Click on treemap or tree to select items
- 🔍 **Tooltips**: Hover over treemap rectangles for detailed information

### Technical Features
- Written in 100% safe Rust
- Cross-platform GUI using `egui`/`eframe`
- Multi-threaded scanning for better performance
- Memory-efficient data structures
- GPL v2 licensed (same as original WinDirStat)

## Installation

### Prerequisites

- Rust 1.70 or later (2021 edition)
- Cargo (comes with Rust)

### Building from Source

```bash
cd windirstats2
cargo build --release
```

The compiled binary will be in `target/release/windirstats2` (or `windirstats2.exe` on Windows).

### Running

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/windirstats2
```

## Usage

1. Launch WinDirStats2
2. Click "Select Directory to Scan" or use File → Select Directory
3. Choose a directory to analyze
4. Wait for the scan to complete
5. Explore your disk usage with:
   - **Directory Tree**: Left panel showing hierarchical structure
   - **TreeMap**: Center panel with visual representation
   - **Extension List**: Right panel showing file type statistics

### Navigation

- **Click** on tree items to select them
- **Click** on treemap rectangles to select files/directories
- **Hover** over treemap to see details in tooltips
- **Scroll** in panels to see more content

## Architecture

### Project Structure

```
windirstats2/
├── src/
│   ├── main.rs            # Application entry point
│   ├── app.rs             # Main application state and UI
│   ├── scanner.rs         # Directory scanning logic
│   ├── file_item.rs       # File/directory data structures
│   ├── treemap.rs         # TreeMap visualization algorithm
│   ├── extension_stats.rs # Extension statistics collection
│   └── ui.rs              # UI rendering components
├── assets/
│   └── icon.png           # Application icon
├── Cargo.toml             # Rust dependencies and project metadata
└── README.md              # This file
```

### Key Components

1. **Scanner**: Walks the file system using `walkdir`, building a tree structure
2. **FileItem**: Represents files and directories with size, metadata, and children
3. **TreeMap**: Implements squarified treemap layout algorithm for visualization
4. **ExtensionStats**: Tracks file type statistics and assigns colors
5. **UI Components**: Modular rendering for tree view, extension list, and progress

## Dependencies

- `eframe` / `egui`: Cross-platform GUI framework
- `walkdir`: Efficient directory traversal
- `rfd`: Native file dialogs
- `anyhow`: Error handling
- `serde`: Serialization (for future config support)

## Platform Support

### Tested Platforms
- ✅ Windows 10/11
- ✅ macOS 10.15+
- ✅ Linux (Ubuntu, Fedora, Arch)

### Platform-Specific Notes

**Windows**: 
- Full support for all features
- Native file dialogs
- Windows-style paths

**macOS**:
- Full support for all features  
- Native file dialogs
- Proper handling of bundle structures

**Linux**:
- Full support for all features
- GTK file dialogs (requires GTK3)
- Handles symbolic links properly

## Comparison with Original WinDirStat

| Feature | Original WinDirStat | WinDirStats2 |
|---------|---------------------|--------------|
| Platform | Windows only | Windows, macOS, Linux |
| Language | C++ (MFC) | Rust |
| UI Framework | MFC | egui |
| Directory Tree | ✅ | ✅ |
| TreeMap | ✅ | ✅ |
| Extension List | ✅ | ✅ |
| Duplicate Detection | ✅ | ⏳ Planned |
| File Search | ✅ | ⏳ Planned |
| Top Files | ✅ | ⏳ Planned |
| Cleanup Actions | ✅ | ⏳ Planned |

## Future Enhancements

- [ ] Duplicate file detection
- [ ] File search functionality
- [ ] Top files view
- [ ] User-defined cleanup actions
- [ ] Configuration persistence
- [ ] Dark/light theme support
- [ ] Export functionality
- [ ] Performance optimizations for very large directories
- [ ] File filtering options
- [ ] Bookmarks for frequently scanned directories

## Development

### Testing

```bash
cargo test
```

### Code Quality

```bash
# Check for errors
cargo check

# Run clippy for lints
cargo clippy

# Format code
cargo fmt
```

### Building for Release

```bash
cargo build --release
```

For optimized builds with better performance:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to:

1. Report bugs
2. Suggest features
3. Submit pull requests
4. Improve documentation

## License

WinDirStats2 is licensed under the GNU General Public License v2.0, the same license as the original WinDirStat.

Copyright © WinDirStat Team

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 2 of the License, or at your option any later version.

## Credits

- Original WinDirStat by the WinDirStat Team
- Logo by Robin "tuqueque" Marín
- Rust implementation: WinDirStat Team

## Resources

- Original WinDirStat: https://windirstat.net/
- WinDirStat GitHub: https://github.com/windirstat/windirstat
- Rust Programming Language: https://www.rust-lang.org/
- egui GUI Framework: https://github.com/emilk/egui

## Acknowledgments

This project is a reimplementation inspired by the excellent original WinDirStat. All credit for the concept and design goes to the original WinDirStat team.
