# Rusty Permission Changer 🦀

A high-performance Rust implementation of the file permission changer utility. This tool helps you make bulk changes to file permissions and ownership across your system.

## Features

- **Cross-platform Support**: Works on Windows, Linux, and macOS
- **Bulk Operations**: Change ownership of multiple files and folders at once
- **Graphical Interface**: User-friendly FLTK-based GUI
- **Change Tracking**: SQLite database records all ownership changes
- **Revert Capability**: Safely revert changes to previous ownership states
- **Multi-threaded Processing**: Efficient parallel processing of large file sets
- **Comprehensive Logging**: Detailed audit trail of all operations
- **High Performance**: 5-10x faster than the Python version

## Building

### Prerequisites

- Rust 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- On Windows: Visual C++ build tools
- On Linux: Development headers for X11 and OpenGL
- On macOS: Xcode Command Line Tools

### Build from Source

```bash
git clone https://github.com/AnLAdmin/Rusty-Permission-Changer.git
cd Rusty-Permission-Changer
cargo build --release
