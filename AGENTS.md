# AGENTS.md - TimeCalc Development Guide

This document provides guidance for agentic coding agents working in the TimeCalc repository.

## Project Overview

TimeCalc is a Rust time calculator with two entry points:
- **timecalc-gui**: Desktop GUI app using egui/eframe
- **timecalc-cli**: Interactive CLI calculator

## Build Commands

```bash
# Build debug (default features)
cargo build

# Build release
cargo build --release

# Build only GUI
cargo build --no-default-features --features gui

# Build only CLI
cargo build --no-default-features --features cli

# Build with all features (used for release)
cargo build --release --all-features
```

## Test Commands

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test <test_name>

# Run tests with output displayed
cargo test -- --nocapture
```

## Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting (without modifying)
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all lints enabled
cargo clippy --all-targets --all-features -- -D warnings
```

## Code Style Guidelines

### Naming Conventions
- Functions: `snake_case` (e.g., `parse_int`, `seconds_from_string`)
- Structs/Types: `PascalCase` (e.g., `TimeCalc`)
- Variables: `snake_case` (e.g., `display_text`, `calculated_seconds`)
- Modules: `snake_case` (e.g., `calc`)

### Function Signature Patterns
- Parse/convert functions return `Option<T>` for error cases
- Use `?` operator for early returns on Option
- Public library functions in `src/calc.rs` should handle errors gracefully without panicking

### Error Handling
- Use `Option<T>` for operations that may fail (parsing)
- Use `println!` only in binary targets (`src/bin/*.rs`)
- Library functions should return `Option<T>` or `Result<T, E>`, not print

### Imports
- External crates first, then local imports
- Group std imports from same crate
- Use absolute paths from crate root where clear: `use timecalc::calc::...`

### Type Usage
- Time values are stored as `i64` (seconds)
- Use `String` for owned text, `&str` for borrowed text
- Prefer `.to_owned()` for converting `&str` to `String`

### String Formatting
- Use `format!` macro for building strings
- Use `format!("{:0>2}", value)` for zero-padded numbers
- Format strings like: `format!("{0}:{1}:{2}", h, m, s)`

### Control Flow
- Use `match` for Option/Result handling
- Avoid deeply nested conditionals; prefer early returns
- Use `if let` for simple pattern matching

### GUI Code (egui/eframe)
- Implement `eframe::App` trait for the main app struct
- Use `ctx.input()` to access keyboard events
- Use `egui::Grid` for calculator button layout
- Handle viewport close request in `update()` method
- Use `add_sized()` to set exact widget dimensions

### CLI Code
- Use `std::io::stdin().read_line()` for input
- Always flush stdout before reading stdin
- Loop indefinitely with `loop {}` for REPL

## File Structure

```
src/
├── lib.rs           # Library root - exports calc module
├── calc.rs          # Core calculation logic (public functions)
└── bin/
    ├── cli.rs       # CLI entry point
    └── gui.rs       # GUI entry point
```

## Architecture Notes

### Core Logic (calc.rs)
The library contains pure functions for time calculations:
- `parse_int()`: Parse integer from string
- `seconds_from_string()`: Convert "HH:MM:SS" or "MM:SS" or raw seconds to i64
- `string_from_seconds()`: Convert i64 seconds to "HH:MM:SS" string
- `calculate_input()`: Parse and execute operator+time expression

### Time Formats Supported
| Format | Example | Interpretation |
|--------|---------|----------------|
| HH:MM:SS | `1:30:00` | 1 hour, 30 minutes |
| MM:SS | `45:30` | 45 minutes, 30 seconds |
| Raw seconds | `3600` | 1 hour |

### Operators
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division

## Common Patterns

### Parsing with Option
```rust
pub fn example_parse(input: &str) -> Option<i64> {
    let value = input.trim().parse::<i64>().ok()?;
    Some(value)
}
```

### Iteration with Early Return
```rust
pub fn process_input(input: &str) -> Option<i64> {
    let chars: Vec<char> = input.chars().collect();
    let first = chars.first()?;
    // ... process
    Some(result)
}
```

### Struct with Default
```rust
struct MyStruct {
    field: String,
}

impl Default for MyStruct {
    fn default() -> Self {
        Self {
            field: "default".to_owned(),
        }
    }
}
```

## CI/CD

Releases are automated via GitHub Actions (`.github/workflows/release.yml`):
- Triggers on version tags
- Builds for: Linux (x64), Windows (x64), macOS (x64 + ARM64)
- Uses `cargo build --release --all-features`
