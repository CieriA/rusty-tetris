# Rust Tetris with SDL2

A clean Tetris clone written in Rust using the SDL2 graphics library.

## Features

- Classic Tetris game mechanics
- Smooth 2D rendering via SDL2
- Keyboard controls
- Score system (?)

## Requirements

- **Rust** (stable) — install via [rustup](https://rustup.rs)
- **SDL2** — graphics and input
- **SDL2_ttf** — for rendering text (e.g., score display)

### SDL2 and SDL2_ttf installation

#### Linux
```bash
sudo apt install llibsdl2-dev libsdl2-ttf-dev
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
git clone https://github.come/{io}/rusty-tetris
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
MIT License

Copyright (c) 2025 Alessio Cieri

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
