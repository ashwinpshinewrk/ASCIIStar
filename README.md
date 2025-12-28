# ASCII Star

Fun little project that uses ASCII character of your choice to display a christmas tree , which is 3D.

Implementation of LOD, that makes the star HD when zoomed in.

---


## Tech Stack
- **Language**: Rust
- **Libraries** : [Ratatui](https://ratatui.rs) & [Crossterm](https://github.com/crossterm-rs/crossterm) for rendering. [Glam](https://github.com/bitshifter/glam-rs) for Math.
- **RNG**: Simple XOR-Shift Implementation for prcedural scattering.
---

## Controlls

| Key | Action |
| :--- | :--- |
|`Type Word`| Set seed for 3D prcedural|
|`Enter`|Start fractal growth animation|
|`s`|Toggle zooming to star (Cinematic)|
|`Arrow Keys`|Set the orientation of the tree model|
|`ESC`|Exit the program|

---
## Project structure

```
src/
├── main.rs      # Terminal objects and event loop
├── app.rs       # Global State
├── engine.rs    # DS, Snow Particles and RNG
└── ui.rs        # Rendering Engine
```
---
## Highlight

1. Depth Sorting
2. Surface Sampling

---

## Installation 

1. Ensure you have [Rust](https://rust-lang.org/) installed.
2. Clone the repo.
3. Run the application:
    ```bash
    cargo run -r
    ```
## Media


**Created for MAKE A STAR 2025 Challenge.**
