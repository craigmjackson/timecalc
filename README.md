# timecalc

A time calculator written in Rust, available as both a GUI app and a CLI tool.

## Features

- Add, subtract, multiply, and divide time values in `HH:MM:SS` or `MM:SS` or raw seconds format
- **GUI** — compact desktop calculator window built with [egui](https://github.com/emilk/egui)
- **CLI** — interactive terminal session for quick calculations

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Cargo (included with Rust)

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/craigmjackson/timecalc.git
cd timecalc
cargo build --release
```

By default, both the GUI and CLI binaries are built. The compiled binaries will be at:

- `./target/release/timecalc-gui`
- `./target/release/timecalc-cli`

To build only one of them:

```bash
cargo build --release --no-default-features --features gui
cargo build --release --no-default-features --features cli
```

## Usage

### GUI

Launch the desktop calculator:

```bash
./target/release/timecalc-gui
```

The GUI displays a compact calculator window. Enter a time value using the keypad, then choose an operator (`+`, `-`, `x`, `÷`) and a second time value, and press `=` to calculate. The display shows the running result and current expression.

**Keyboard shortcuts:**

| Key | Action |
|---|---|
| `0`–`9`, `:` or `.` | Enter time digits / separator |
| `+`, `-`, `*` or `x`, `/` | Set operator |
| `Enter` | Calculate |
| `Escape` | Clear (AC) |
| `Cmd+Q` | Quit |

### CLI

Launch the interactive CLI session:

```bash
./target/release/timecalc-cli
```

The CLI starts at `00:00:00` and accepts one operation per line in the format `<operator> <time>`. Type `exit` or `quit` to close.

**Example session:**

```
Starting time: 00:00:00
> + 1:30:00
01:30:00
> + 45:00
02:15:00
> - 0:30
01:45:00
> / 3
00:35:00
> exit
```

## Time Format

Times can be entered in any of the following formats:

| Format | Example | Interpreted as |
|---|---|---|
| `HH:MM:SS` | `1:30:00` | 1 hour, 30 minutes |
| `MM:SS` | `45:30` | 45 minutes, 30 seconds |
| Raw seconds | `3600` | 1 hour |

## Building & Testing

```bash
# Run tests
cargo test

# Build debug
cargo build

# Build release
cargo build --release

# Cross-compile Windows build on non-Windows machine
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin
cargo xwin build --release --target x86_64-pc-windows-msvc

```

## License

See the repository for license details.
