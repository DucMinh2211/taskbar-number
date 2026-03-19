# Taskbar Number Overlay

A lightweight Windows utility that displays numbers over your taskbar icons to help you quickly identify the index for `Win + [Number]` shortcuts.

## Features

- **Icon Indexing**: Automatically detects pinned and running apps on the taskbar and labels them with numbers (1, 2, 3, ..., 0).
- **Smart Overlay**: Uses a transparent, topmost window that doesn't interfere with your clicks or typing.
- **System Tray Integration**:
  - **Pause/Resume**: Quickly toggle the overlay when watching videos or in fullscreen apps.
  - **Exit**: Close the application cleanly.
- **Dynamic Updates**: Refreshes every 500ms to stay in sync with your taskbar as you open or close apps.

## Installation

### Prerequisites

- **Windows 10/11**
- **Rust** (if building from source): [Install Rust](https://rustup.rs/)

### Building from source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/taskbar-number.git
   cd taskbar-number
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the executable found in `target/release/taskbar-number.exe`.

## How to Use

1. Run the application. You will see numbers appearing over your taskbar icons.
2. Press `Win + [Number]` to open or switch to the corresponding app.
3. To **Pause** the overlay (e.g., when watching YouTube in fullscreen):
   - Right-click the application icon in the System Tray (near the clock).
   - Select **Pause**. The numbers will disappear.
   - Select **Resume** to show them again.
4. To **Exit**, right-click the tray icon and select **Exit**.

## Tech Stack

- **Rust**: For high-performance and safety.
- **windows-rs**: To interact directly with the Windows Win32 and UI Automation APIs.
- **UI Automation**: For robust taskbar icon detection across different Windows versions and configurations.

## License

[MIT](LICENSE)
