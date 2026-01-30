# TaskFlow - Project Plan

## Overview
A small, efficient GUI application for batch file processing with drag-and-drop functionality and queue management. Run any script on any file type sequentially with an intuitive interface. Built with Rust and egui.

## Core Features

### User Interface
- **Compact window design** with minimal, functional layout
- **Drop zone** with icon for intuitive file addition
- **Script selector** dropdown to choose processing operation
- **Output location** selector with browse button
- **Log checkbox** to enable/disable logging
- **Queue list** with scrollbar showing pending files
- **Start/Cancel button** that toggles based on processing state
- **Status bar** displaying current operation

### Queue Management
#### Adding Files
- Drag and drop files onto drop zone
- No file type filtering (accepts all files)

#### Selection & Manipulation
- **Single click** to select item
- **Ctrl+click** for multi-select (individual items)
- **Shift+click** for range selection
- **Delete key** removes selected items from queue
- **Click and drag** to reorder selected items

#### Processing States
- **Pending items**: Normal appearance, fully interactive
- **Active item** (currently processing):
  - Visually distinguished (greyed out or highlighted)
  - Locked (cannot be moved or deleted)
- **Completed items**: Automatically removed from queue

### Configuration & Persistence
- **Script selection** saved and restored between launches
- **Output location** saved and restored between launches
- **Settings storage** using platform-appropriate config file location

### Processing
#### Threading Model
- **Main thread**: UI rendering and queue management
- **Worker thread**: Script execution and file processing
- Queue remains fully interactive during processing

#### Workflow
1. Application launches with previous script and output settings
2. User configures logging preference (optional)
3. User adds files via drag-and-drop
4. User arranges queue order as needed
5. User clicks "Start" to begin processing
6. Files process sequentially, one at a time
7. Status bar updates with current file
8. Completed files are removed from queue
9. User can continue adding/removing/reordering pending items
10. User can click "Cancel" to stop processing

#### Error Handling
- **On error**: Skip failed file and continue to next in queue
- **No visual feedback** for errors in UI
- **Errors logged** to log file (if logging enabled)
- Processing continues uninterrupted

### Logging
- **Optional feature** controlled by checkbox
- **Log file location**: Output directory
- **Log file naming**: TBD (timestamp-based or single rolling log)
- **Log contents**:
  - Timestamp for each operation
  - File being processed
  - Script being executed
  - Success/failure status
  - Error messages and details

## Technical Stack

### Core Technologies
- **Language**: Rust
- **GUI Framework**: egui (immediate mode GUI)
- **File Operations**: std::fs
- **Process Execution**: std::process::Command
- **Threading**: std::thread with channels for communication
- **Config Storage**: TBD (serde + toml/json)

### Dependencies (Preliminary)
```toml
egui = "0.XX"
eframe = "0.XX"  # egui framework wrapper
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"  # or toml for config
```

## Project Structure
```
taskflow/
├── Cargo.toml
├── PLAN.md
├── README.md
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Main application state and UI
│   ├── queue.rs          # Queue management logic
│   ├── processor.rs      # File processing worker
│   ├── config.rs         # Configuration persistence
│   └── logger.rs         # Logging functionality
└── assets/
    └── drop_zone_icon.svg  # Drop zone icon (TBD)
```

## Development Phases

### Phase 1: Basic UI
- [ ] Set up Rust project with egui
- [ ] Create basic window layout
- [ ] Implement drop zone (visual only)
- [ ] Add script selector dropdown
- [ ] Add output location selector
- [ ] Add start/cancel button
- [ ] Add status bar

### Phase 2: Queue Functionality
- [ ] Implement file drop handling
- [ ] Create queue list display
- [ ] Add single-click selection
- [ ] Implement multi-select (Ctrl+click, Shift+click)
- [ ] Add delete key handling
- [ ] Implement drag-to-reorder

### Phase 3: Processing
- [ ] Set up worker thread architecture
- [ ] Implement script execution
- [ ] Add queue processing loop
- [ ] Handle item state changes (pending → active → removed)
- [ ] Implement cancel functionality
- [ ] Update status bar during processing

### Phase 4: Configuration & Logging
- [ ] Implement config file persistence
- [ ] Save/restore script selection
- [ ] Save/restore output location
- [ ] Add logging checkbox
- [ ] Implement log file creation
- [ ] Add error logging

### Phase 5: Polish & Testing
- [ ] Add visual feedback for active item
- [ ] Improve UI responsiveness
- [ ] Test error handling
- [ ] Test multi-threading stability
- [ ] Handle edge cases
- [ ] Add user documentation

## Open Questions & Decisions
1. **Log file naming**: Single rolling log or timestamp-based files?
2. **Script configuration**: How are scripts defined? Config file with paths?
3. **Icon design**: What should the drop zone icon look like?
4. **Window sizing**: Fixed size or resizable?
5. **Platform support**: Linux only, or cross-platform (Windows/macOS)?
6. **Script output parsing**: Should we parse script output for progress?

## Future Enhancements (Out of Scope for v1)
- Progress bars for individual files
- Parallel processing option
- Preset configurations
- Queue save/restore
- Batch operations (clear queue, select all, etc.)
- Recent files list
- Script editor/manager

