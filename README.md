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
git clone https://github.com/CieriA/rusty-tetris
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
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.
