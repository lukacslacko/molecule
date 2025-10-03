# molecule

A simple 3D visualization application built with Bevy game engine.

## Features

- 3D scene with multiple colored spheres
- Interactive camera controls:
  - **Mouse**: Hold left mouse button and drag to rotate the camera
  - **Keyboard**:
    - `W` - Move forward
    - `S` - Move backward
    - `A` - Move left
    - `D` - Move right
    - `Space` - Move up
    - `Left Shift` - Move down

## Building and Running

Make sure you have Rust installed, then:

```bash
cargo build --release
cargo run --release
```

## Dependencies

- Bevy 0.15.x - Game engine framework