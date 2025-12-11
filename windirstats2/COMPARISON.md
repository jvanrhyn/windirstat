# Comparison: WinDirStat vs WinDirStats2

This document compares the original C++ WinDirStat with the new Rust WinDirStats2 implementation.

## Overview

**Original WinDirStat**
- Language: C++ (MFC Framework)
- Platform: Windows only
- Code: ~109 files, ~12,600 lines
- GUI: Microsoft Foundation Classes (MFC)
- License: GPL v2

**WinDirStats2**
- Language: Rust (2021 edition)
- Platform: Cross-platform (Windows, Linux, macOS)
- Code: 7 modules, ~700 lines
- GUI: egui/eframe
- License: GPL v2 (same as original)

## Architecture Comparison

### Original WinDirStat (C++)

```
windirstat/
├── WinDirStat.cpp/h      - Main application
├── MainFrame.cpp/h       - Main window frame
├── DirStatDoc.cpp/h      - Document class (MVC pattern)
├── Controls/
│   ├── TreeMap.cpp/h     - TreeMap visualization
│   ├── TreeListControl.cpp/h - Tree view
│   └── ExtensionListControl.cpp/h - Extension list
├── Dialogs/              - Various dialog boxes
├── Views/                - MVC view classes
└── ...                   - Many more files
```

### WinDirStats2 (Rust)

```
windirstats2/
├── src/
│   ├── main.rs           - Entry point
│   ├── app.rs            - Main application state & UI
│   ├── scanner.rs        - Directory scanning
│   ├── file_item.rs      - Data structures
│   ├── treemap.rs        - TreeMap visualization
│   ├── extension_stats.rs - File type statistics
│   └── ui.rs             - UI components
├── Cargo.toml            - Dependencies
├── README.md             - Documentation
├── BUILD.md              - Build guide
└── LICENSE               - GPL v2 license
```

## Feature Comparison

| Feature | Original | WinDirStats2 | Notes |
|---------|----------|--------------|-------|
| **Core Features** | | | |
| Directory scanning | ✅ | ✅ | Multi-threaded in both |
| TreeMap visualization | ✅ | ✅ | Squarified algorithm |
| Extension list | ✅ | ✅ | Color-coded |
| Directory tree | ✅ | ✅ | Hierarchical view |
| File selection | ✅ | ✅ | Click to select |
| Size calculation | ✅ | ✅ | Recursive aggregation |
| **Advanced Features** | | | |
| Duplicate detection | ✅ | ⏳ Future | Planned |
| File search | ✅ | ⏳ Future | Planned |
| Top files list | ✅ | ⏳ Future | Planned |
| Cleanup actions | ✅ | ⏳ Future | Planned |
| User-defined actions | ✅ | ⏳ Future | Planned |
| **UI Features** | | | |
| Resizable panels | ✅ | ✅ | |
| Tooltips | ✅ | ✅ | |
| Status bar | ✅ | ✅ | |
| Menu bar | ✅ | ✅ | |
| Keyboard shortcuts | ✅ | ⏳ Future | |
| Context menus | ✅ | ⏳ Future | |
| **Configuration** | | | |
| Settings persistence | ✅ | ⏳ Future | |
| Color customization | ✅ | ⏳ Future | |
| Portable mode | ✅ | N/A | Always portable |
| **Platform Support** | | | |
| Windows | ✅ | ✅ | Native on both |
| Linux | ❌ | ✅ | New in Rust version |
| macOS | ❌ | ✅ | New in Rust version |

## Technical Differences

### Memory Safety

**Original (C++)**
- Manual memory management
- Potential for memory leaks, buffer overflows
- Uses smart pointers where available
- MFC framework handles some memory management

**WinDirStats2 (Rust)**
- Guaranteed memory safety at compile time
- No garbage collector needed
- Ownership system prevents memory leaks
- Zero-cost abstractions

### Concurrency

**Original (C++)**
- Windows threading API
- Manual synchronization with mutexes
- Potential for race conditions if not careful

**WinDirStats2 (Rust)**
- Safe concurrency with Rust's type system
- Data race prevention at compile time
- std::thread with Arc/Mutex for sharing

### Dependencies

**Original (C++)**
```
- Windows SDK
- Microsoft Foundation Classes (MFC)
- Visual Studio (for building)
- Windows-specific APIs
```

**WinDirStats2 (Rust)**
```toml
- eframe/egui (cross-platform GUI)
- walkdir (directory traversal)
- rfd (native file dialogs)
- anyhow (error handling)
- serde (serialization)
- chrono (date/time)
```

## Performance Characteristics

### Original WinDirStat
- **Pros**: Highly optimized C++ code, native Windows APIs
- **Cons**: Windows-only, potential memory issues

### WinDirStats2
- **Pros**: Memory-safe, cross-platform, modern concurrency
- **Cons**: Slightly larger binary due to included GUI framework

### Build Times
- **Original**: ~2-5 minutes (incremental)
- **WinDirStats2**: ~3 minutes (release, first build), seconds (incremental)

### Binary Size
- **Original**: ~2-3 MB (with MFC)
- **WinDirStats2**: ~15-20 MB (includes GUI framework, can be reduced with strip)

## Code Complexity

### Lines of Code

**Original WinDirStat**
- Total: ~12,600 lines
- Files: 109 (.cpp + .h)
- Average: ~115 lines per file

**WinDirStats2**
- Total: ~700 lines
- Files: 7 (.rs)
- Average: ~100 lines per file

### Maintainability

**Original**: 
- More complex due to MFC framework
- Windows-specific APIs throughout
- Requires Windows development expertise

**WinDirStats2**:
- Simpler, more modern code
- Platform-agnostic core logic
- Standard Rust patterns
- Easier to understand and modify

## User Experience

### Similarities
- Same visual concept (treemap + tree + extension list)
- Similar color scheme possibilities
- Same workflow: select directory → scan → visualize

### Differences
- **WinDirStats2** has a more modern UI look (egui style)
- **Original** has more polish and features
- **WinDirStats2** works on macOS and Linux

## Future Roadmap for WinDirStats2

### Short Term
1. Add configuration persistence
2. Implement duplicate file detection
3. Add file search functionality
4. Implement top files view

### Medium Term
5. Add cleanup actions
6. User-defined commands
7. Keyboard shortcuts
8. Context menus

### Long Term
9. Performance optimizations
10. Themes (dark/light mode)
11. File filtering
12. Export functionality
13. Bookmarks

## Why Reimplement in Rust?

### Advantages

1. **Cross-Platform**: Works on Windows, Linux, and macOS
2. **Memory Safety**: No buffer overflows or memory leaks
3. **Modern Tooling**: Cargo, crates.io ecosystem
4. **Maintainability**: Cleaner, more concise code
5. **Safety**: Thread safety guaranteed by compiler
6. **Future-Proof**: Modern language with active development

### Trade-offs

1. **Feature Completeness**: Original has more features (for now)
2. **Binary Size**: Slightly larger due to included dependencies
3. **Learning Curve**: Requires Rust knowledge to modify

## Conclusion

WinDirStats2 is a modern, cross-platform reimplementation that brings the excellent WinDirStat concept to Linux and macOS users while providing a more maintainable codebase. While it doesn't yet have all the features of the original, its foundation is solid and extensible.

The original WinDirStat remains an excellent choice for Windows users who need the full feature set, while WinDirStats2 offers a compelling alternative for those who:
- Use multiple operating systems
- Want a more lightweight, memory-safe solution
- Prefer modern Rust code
- Want to contribute to an easier-to-understand codebase

Both projects share the same GPL v2 license and spirit of helping users understand their disk usage visually.
