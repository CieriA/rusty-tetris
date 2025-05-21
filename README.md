# Rust Tetris with SDL2

A Tetris clone written in Rust using the SDL2 graphics library.

## Features

- Classic Tetris game mechanics
- Smooth 2D rendering via SDL2
- Keyboard controls
- Score system

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

Or, if you do **not** have Rust or SDL2, run the [binary file](binaries/release_tetris)
with
```bash
chmod +x releasw_tetris
./release_tetris
```
while being on the [binaries](binaries) folder.

NOTE: the binary for now present is compiled with macOS (Apple Silicon).

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
[ISC License](LICENSE)
