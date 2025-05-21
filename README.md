# Rust Tetris with SDL2

A Tetris clone written in Rust using the SDL2 graphics library.

## Features

- Classic Tetris game mechanics
- 2D rendering via SDL2
- Keyboard controls
- Hard and Soft Drop
- Score system

## Requirements

- **Rust** (stable) - install via [rustup](https://rustup.rs)
- **SDL2** - graphics and input
- **SDL2_ttf** - for rendering text

### SDL2 and SDL2_ttf installation

#### Linux
```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev
```

#### macOS (using Homebrew)
```bash
brew install sdl2 sdl2_ttf
```

#### Windows
Use [vcpkg](https://github.com/microsoft/vcpkg) or manually install SDL2 and SDL2_ttf developer packages.
Make sure the .ddl files are in your `PATH` or project folder at runtime.

## Building the Project
Clone the repository and build in release mode:
```bash
git clone https://github.come/CieriA/rusty-tetris
cd rusty-tetris
cargo build --release
```

## Running the game
```bash
cargo run --release
```

## Controls
- Left Arrow / A — Move the piece left
- Right Arrow / D — Move the piece right
- Down Arrow / S — Soft Drop
- Space — Hard Drop
- Up Arrow / W / Right Control — Rotate the piece clockwise
- Q / Left Control — Rotate the piece counterclockwise

## Development Notes
This project uses the following crates:
- sdl2
- rand

TO regenerate documentation locally:
```bash
cargo doc --open
```

## License
ISC License

Copyright (c) 2025, CieriA

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.