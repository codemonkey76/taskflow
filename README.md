# TaskFlow

A lightweight, efficient GUI application for batch file processing. Drop files, select a script, and watch TaskFlow process them sequentially.

## Features

- 🎯 **Drag & Drop Interface** - Simple file addition via drag and drop
- 📝 **Queue Management** - Reorder, remove, and manage your processing queue
- 🔄 **Sequential Processing** - Process files one at a time with any script
- 📊 **Real-time Status** - See what's currently being processed
- 💾 **Persistent Settings** - Remembers your script and output preferences
- 📋 **Optional Logging** - Keep track of processed files and errors
- 🎨 **Lightweight UI** - Small, focused interface built with egui

## Use Cases

- Video encoding and conversion
- Image batch processing
- File format conversion
- Data transformation pipelines
- Any repetitive file processing task

## Quick Start

*(Coming soon - application is in development)*

## Building from Source

```bash
# Clone the repository
git clone https://github.com/codemonkey76/taskflow.git
cd taskflow

# Build and run
cargo run --release
```

## Requirements

- Rust 1.70 or higher
- Your processing scripts (bash, python, etc.)

## Configuration

TaskFlow stores its configuration in platform-specific locations:

- **Linux**: `~/.config/taskflow/config.json`
- **macOS**: `~/Library/Application Support/taskflow/config.json`
- **Windows**: `%APPDATA%\taskflow\config.json`

## Usage

1. Launch TaskFlow
2. Select your processing script from the dropdown
3. Choose an output directory
4. Optionally enable logging
5. Drag and drop files onto the drop zone
6. Arrange the queue as needed
7. Click "Start" to begin processing

### Queue Controls

- **Click** - Select item
- **Ctrl+Click** - Multi-select individual items
- **Shift+Click** - Select range
- **Delete** - Remove selected items
- **Drag** - Reorder items in queue

## Roadmap

See [PLAN.md](PLAN.md) for detailed development phases and features.

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
