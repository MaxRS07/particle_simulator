# Particle Simulator

A real-time interactive particle physics sandbox built with Rust. Simulate falling sand, spreading fire, flowing water, and more on a grid-based world using cellular automaton mechanics.

## Demo

Place particles, watch them interact, and experiment with physics:

- **Sand** falls and slides around obstacles
- **Fire** spreads to adjacent wood blocks and burns out
- **Stone** and **Wood** act as immovable solids
- **Erase** particles with right-click to reshape the environment

## Features

- 5 particle types with distinct behaviors (Sand, Stone, Wood, Fire, Water)
- Real-time physics simulation at ~60 FPS
- Mouse-driven painting and erasing
- Keyboard shortcuts for particle selection and world reset
- 2× scaled display (256×256 world rendered at 512×512)

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/MaxRS07/particle_simulator.git
cd particle_simulator

# Run in release mode (recommended for performance)
cargo run --release

# Or run in debug mode
cargo run
```

## Controls

| Input | Action |
|-------|--------|
| `1` | Select **Sand** |
| `2` | Select **Stone** |
| `3` | Select **Wood** |
| `4` | Select **Fire** |
| Left Mouse | Paint selected particle |
| Right Mouse | Erase particles |
| `R` | Reset / clear world |
| `Esc` | Quit |

## Particle Types

| Particle | Behavior |
|----------|----------|
| **Sand** | Falls due to gravity; slides left or right when blocked |
| **Stone** | Static, immovable solid |
| **Wood** | Static solid; flammable — turns to Fire when adjacent to Fire |
| **Fire** | Spreads to adjacent Wood; extinguishes after one tick |
| **Water** | Defined but not yet fully implemented |

## Project Structure

```
src/
├── main.rs        # Entry point — initializes the world and launches the window
├── game.rs        # World state, cell management, and per-frame update logic
├── cell_types.rs  # Particle type definitions, physics, and colour values
├── window.rs      # Window creation, input handling, and render loop
└── utils.rs       # Colour encoding helper (RGB → u32)
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [`minifb`](https://crates.io/crates/minifb) | 0.25 | Cross-platform framebuffer window |
| [`rand`](https://crates.io/crates/rand) | 0.8 | Random number generation (fire spreading) |
| [`vek`](https://crates.io/crates/vek) | 0.16 | 2D vector math for positions and velocities |

## Configuration

World parameters are set in `src/main.rs`:

```rust
World::new(
    256,  // width  (pixels)
    256,  // height (pixels)
    8,    // cell_size (pixels per cell — gives a 32×32 grid)
    8,    // tick_rate (simulate every N frames)
    0, 0  // tick_speed, gravity (reserved)
)
```

Adjust `cell_size` to change grid resolution, or `tick_rate` to speed up / slow down the simulation.

## License

This project does not currently include a license file. Please contact the repository owner for usage terms.
