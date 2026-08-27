# Terminal Sand Toy - Qwen 3.6 35B Test Project

This is a test to see how Qwen 3.6 35B and Hermes can be used to automatically code. This project has a rust TUI based program that is a sand simulation program with some fun automatic events like plant growth, rain, evaporation, seedfall, etc.

## Features (Planned)

- Terminal-based UI using `crossterm`
- Sand particle physics simulation
- Automatic environmental events:
  - 🔥 **Plant Growth** - seeds sprout and grow plants
  - 🌧 **Rain** - water that flows downhill and erodes sand
  - 💨 **Evaporation** - water that evaporates back into the atmosphere
  - 🌱 **Seedfall** - random seeds that fall from the top of the canvas

## Tech Stack

- ✅ Rust as language choice for performance-critical simulation workloads
- ✅ `crossterm` for terminal UI
- ✅ `raylib-rs` or vanilla OpenGL for rendering (future enhancement)
- ✅ Cross-platform compatibility via Rust's standard tools
- ✅ Cargo.toml for dependency management with semantic versioning
- ✅ Clippy and fmt linting for code quality standards

## Getting Started

### Prerequisites
```bash
cargo --version  # Check that you have cargo installed
rustc --version  # Verify rustc installation is working properly
tput cols         # Test terminal capabilities before running the simulation program
```

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/meltingscales/TerminalSandToyQwen3.6Test.git
   cd TerminalSandToyQwen3.6Test
   ```
2. Install dependencies and build:
   ```bash
   cargo update  # Update all dependencies to latest versions first
   cd sandbox    # Change into the sandbox directory before building the simulation
   ```

### Usage
```bash
# Run the sand simulation
cargo run --release

# For development (faster compile times but slower runtime)
cd simulator/src/sim
cargo run
```

## Project Structure

```
simulator/              # Main application code for the terminal-based simulation
├── src/                # Source directory for Rust sim code
│   └── main.rs        # Entry point for the sand simulation program
├── Cargo.toml          # Simulator dependencies configuration file
└── docs/               # Simulation documentation and architecture notes
    └── design.md       # Sim design documents and simulation rules documentation
```

## Architecture Notes

The terminal UI uses `crossterm` for terminal control, with a grid-based approach to:
- Track particle positions in the 2D sandbox environment
- Update physics rules each frame (gravity, water flow, plant growth)
- Render the updated state back to the terminal each tick
- Handle user input via keyboard events for pause/resume and zoom controls

## Future Enhancements

- [ ] Add soil erosion mechanics that track sand loss over time periods
- [ ] Implement temperature modeling that affects evaporation rates dynamically
- [ ] Create a "flood" event with configurable intensity levels for simulating extreme weather conditions 
- [ ] Support for custom events through configuration files at runtime
- [ ] Multi-threaded physics updates using `rayon` parallel processing libraries

## License

MIT License - see the LICENSE file for details on how you can use this code.

### Contributing

1. Fork it
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

Built with ❤️ using Qwen 3.6 35B + Hermes Agent
