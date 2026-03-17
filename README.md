# horda

Keyboard-driven command launcher for Linux (X11).

This project is at a very early stage of development. Not even 󰀫

All code handwritten with AI just whisperint to my ears.

Press `Alt+Space` to open the launcher, type a prefix of a command name, press `Enter` to execute.

## Features

- `Launch` — run a program or script
- `Keystrokes` — send key combinations to the previously focused window
* Auto-executes on unambiguous match

## Config

Commands are defined in `config.toml`:

```toml
[[commands]]
name = "browser"
description = "open browser"
action_type = "launch"
action_value = "firefox"

[[commands]]
name = "lock"
description = "lock screen"
action_type = "keystrokes"
action_value = "ctrl+l"
```

## Requirements

- Linux with X11
- [Rust](https://rustup.rs)

## Build & run

```bash
cargo run
```
