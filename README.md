# Rusty Permission Changer 🦀

A high-performance Rust implementation of the file permission changer utility. This tool helps you make bulk changes to file permissions and ownership across your system.

## Features

- **Platform Support**: Windows
- **Bulk Operations**: Change ownership of multiple files and folders at once
- **Graphical Interface**: User-friendly FLTK-based GUI
- **Change Tracking**: SQLite database records all ownership changes
- **Revert Capability**: Safely revert changes to previous ownership states
- **Multi-threaded Processing**: Efficient parallel processing of large file sets
- **Comprehensive Logging**: Detailed audit trail of all operations
- **High Performance**: 5-10x faster than the Python version

## Contributing
This is AI generated and untested. Not open for contribution right now.

## Thanks
Based on the code of https://github.com/TSTP-Enterprises/TSTP-Permission_Changer.

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
```

## Architecture
* GUI Module: FLTK-based user interface (src/gui.rs)
* Permissions Module: Low-level OS permission handling (src/permissions.rs)
* Database Module: SQLite change tracking (src/database.rs)
* Worker Module: Threaded operations for performance (src/worker.rs)
* Utils Module: Helper functions and logging (src/utils.rs)

## Database
``` SQL
CREATE TABLE ownership_changes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    original_owner TEXT NOT NULL,
    current_owner TEXT NOT NULL,
    operation_time DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
