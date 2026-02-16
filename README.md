# rustlex

A terminal-based analogue wristwatch built with Rust and [Ratatui](https://github.com/ratatui/ratatui). A love letter to iconic dive watches, rendered in Braille characters for high-resolution terminal graphics.

<img src="assets/2026-02-16_12-15.png" width="50%">

## Features

- Submariner-inspired watch face with rotating bezel, chapter ring, and date window
- Mercedes hour hand, sword minute hand, lollipop second hand
- Lume mode — simulates viewing the watch in darkness
- Smooth sweep toggle — switch between quartz tick and mechanical glide
- Twinkling star background
- Resizes dynamically with the terminal window
- Cross-platform: Windows Terminal, Kitty, iTerm2, and other Unicode-capable terminals

## Building

Requires [Rust](https://www.rust-lang.org/tools/install) 1.70 or later.

```bash
# Clone the repo
git clone https://github.com/g2h0/rustlex.git
cd rustlex

# Debug build (faster compile)
cargo build

# Release build (smaller binary, optimized)
cargo build --release
```

The release binary will be at `target/release/rustlex` (or `rustlex.exe` on Windows).

## Usage

```bash
# Run directly
cargo run

# Or run the built binary
./target/release/rustlex
```

The watch face will fill your terminal window. Resize the terminal and the watch scales with it.

## Controls

| Key | Action |
|---|---|
| `q` / `Esc` / `Ctrl+C` | Quit |
| `Scroll wheel` | Rotate bezel (120 clicks per full rotation) |
| `s` | Toggle twinkling star background |
| `l` | Toggle lume mode |
| `m` | Toggle movement (quartz / smooth sweep) |

### Lume Mode

Press `l` to simulate darkness. The dial, bezel, crown, and logo disappear — only the luminous markers and hands glow green, just like a real dive watch in the dark.

### Smooth Sweep

Press `m` to switch between quartz (1 tick per second) and mechanical smooth sweep. In smooth mode the second hand glides continuously like a Spring Drive movement.

### Rotating Bezel

Use the scroll wheel to rotate the outer bezel. It clicks in 120 discrete positions matching the real dive watch mechanism.

## Terminal Compatibility

Rustlex uses Braille characters (U+2800 block) for high-resolution rendering. This works well in:

- **Windows Terminal** (recommended on Windows)
- **Kitty** (recommended on Linux)
- **iTerm2** (recommended on macOS)
- Most modern terminal emulators with Unicode support

Legacy terminals like `cmd.exe` may not render correctly. If the display looks broken, try a different terminal emulator.

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [ratatui](https://crates.io/crates/ratatui) | 0.29 | TUI framework with Canvas widget |
| [crossterm](https://crates.io/crates/crossterm) | 0.28 | Cross-platform terminal backend |
| [chrono](https://crates.io/crates/chrono) | 0.4 | Local time for clock hands |
