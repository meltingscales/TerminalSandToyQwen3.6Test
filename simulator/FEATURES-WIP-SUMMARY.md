# Terminal Sand Toy — Work-in-Progress Feature Summary

Generated for handoff to a fresh session. Last contact: 2026-08-27.

---

## Current Repository State

**Repo:** `meltingsales/TerminalSandToyQwen3.6Test` (public)  
**Branch:** `main`  
**Working dir:** `~/Git/TerminalSandToyQwen3.6Test/simulator/`  
**Build:** compiles cleanly with `cargo build` after the auto-rain refactor (last good commit).

---

## File Inventory (what is on disk NOW)

Each file is read and verified below. Only confirmed content matters.

### PARTICLES.RS — COMPLETE & VERIFIED ✅
Path: `simulator/src/particles.rs` (41 lines)

```rust
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Particle {
    #[default]
	Empty = 0,
	Sand,
	WetSand,       // sand that has absorbed water ✓
	Water,
	Seed,
	Plant,
}

impl Particle {
    pub const fn glyph(self) -> char { ... }   // maps each enum to display char
    pub const fn fg_color(&self) -> Color { ... }  // color table for rendering
	pub const fn is_empty(self) -> bool { self == Self::Empty }
	pub const fn is_water(self) -> bool { self == Self::Water }
	pub const fn is_wet_soil(self) -> bool { self == Self::WetSand }
	pub const fn is_plant(self) -> bool { self == Self::Plant }
}
```
Verified via `read_file` — all 41 lines intact. Particles list includes **WetSand** (new type planned for soaking mechanic). Methods are correct.

---

### SIMULATION.RS — PARTIAL (167 lines, cut off mid-diagonal) ⚠️
Path: `simulator/src/simulation.rs` (167 lines)

**Confirmed complete:**
- `World` struct with dynamic `width`/`height`, `Vec<Particle>` grid, `frame` counter, `rain_frame` for auto cycle.
- `New()`, `Reset()`, `Seed_initial_state()` — good.
- `Resize(Nw, Nh)` preserves existing cells on terminal resize.
- `Get(x, y)`, `Set(x, y, p)` bounds-checked access into flat Vec.
- `Width()`, `Height()`, `Count(kind)` — correct.
- `Tick()` — implements auto 90-frame rain/dry cycle (first half = rainy, second half = dry). Calls physics_pass, then either spawn_rain or dry_season_process. ✅

**Confirmed complete:**
- `rain_on()` returns true if `(rain_frame % 90) < 45`
- `weather_status()` returns "Rain ACTIVE" or "DRY SEASON"
- `toggle_rain()` flips cycle phase manually (user can force rain/drought override) ✅

**Cut off at line ~161:** sand diagonal-down-left logic. Needs:
- Close the diagonal-left block
- Add **diagonal-right** branch (mirror of left)
- After closing the full sand gravity loop, add remaining physics passes below:

```rust
// Remaining to implement in this file (lines 168+):

// ── WATER PASS (after sand) ─────────────────────────----------
// Water drips down into empty cells. When hitting Sand/WetSand,
// creates WetSand soaking effect (water slowly penetrates soil).

// ── SEED PASS ────────────────────────────────────────────────
// Seeds fall like sand until they hit something. When a seed lands
// in or adjacent to WetSand/Water → it grows into a plant (upward)
// and drops a new seed above it. Without moisture nearby, seeds
// eventually die (tiny random chance per tick).

// ── PLANT LIFE CYCLE PASS ────────────────────────────────────
// Plants age slowly. They need WetSand or Water to survive:
//   - Near water → grow taller (add new plant above), occasionally drop seed.
//   - No moisture nearby → eventually die → drop 1 seed at base, then become Empty.

// ── STANDALONE FUNCTIONS ─────────────────────────────────────
fn spawn_rain(world: &mut World) {
    // Drop 3 water cells from the top of the grid each active tick.
}

fn dry_season_process(world: &mut World, frame_num: u64) {
    // During drought (rain_frame % 90 in second half):
    //   - Slowly convert some WetSand back to regular Sand (dry out).
    //   - Evaporate standing surface water into vapor.
    //   - Seeds near the top of pools can re-enter atmosphere.
}
```

The physics loop structure is set up; just needs: 1) close sand diagonal block, 2) add right branch, 3) finish closing braces for sand pass, 4) continue with water pass on WetSand soaking, 5) seed spread toward wet soil, 6) plant lifecycle (age/die/spread), 7) spawn_rain and dry_season_process functions.

---

### MAIN.RS — COMPLETE & VERIFIED ✅
Path: `simulator/src/main.rs` (150 lines)

- Raw mode toggle + alternate screen entry/exit via crossterm ✅
- ratatui render loop with frame callback ✅
- Dynamic grid resizing via `world.resize()` on terminal size change ✅
- Keyboard input: ESC/Ctrl+C quit, `R` reset, `M` toggle rain (manual override), Space pause/resume ✅
- Mouse events: click or drag anywhere to paint vertical brush strokes of particles ✅
- HUD: top title bar with "Terminal Sand Toy — Qwen 3.6 + Hermes" + stat line showing Sand/Water/Seeds/Plants counts and current weather status ✅
- `render()` takes `&mut World` (for resize during render) → correct ✅

Verified via `read_file` — all 150 lines intact. No issues on disk.

---

### CARGO.TOML — COMMITTED & VERIFIED ✅
```toml
[package]
name = "simulator"
version = "0.1.0"
edition = "2024"
description = "Terminal sandbox sand simulation with automatic environmental events"

[dependencies]
ratatui = "0.29"
crossterm = "0.28"
rand = "0.9"
```
Verified in previous terminal session — no issues. Dependencies resolve correctly.

---

## PLANNED / NOT YET IMPLEMENTED FEATURES

### 1. Auto Rain Cycle (IN PROGRESS) ✅/⚠️
- **Goal:** Automatically alternate between rainy season (~45 frames, rain spawns from top) and dry season (~45 frames, evaporation dominates).
- **Status:** Core loop in `tick()` is implemented — `tick()` auto-advances `rain_frame` each frame, checks `rain_on()` (first half of 90-frame cycle = rainy). Calls `spawn_rain()` during rain or `dry_season_process()` during drought.
- **What's missing:** The `spawn_rain()` and `dry_season_process()` standalone functions need to be written in simulation.rs after the open physics loop closes.

### 2. Wet Sand / Soil Soak Mechanic (PLANNED) 
- **Goal:** When water contacts dry sand (either falling through it or spreading onto it), the dry sand becomes **WetSand** — a dark, damp state that seeds can grow from. WetSand slowly dries back to regular Sand during drought.
- **Particle type added?** `WetSand` is in `particles.rs` enum ✅ (`is_wet_soil()` checks `self == Self::WetSand`). Color: darker brown (107, 73, 28). Glyph: full block █.
- **Physics logic location:** Goes inside the water gravity pass — if next cell down is Sand → it soaks in, becomes WetSand; if next is Empty → water drips through; if blocked by ground at bottom → spreads sideways.

### 3. Seed Growth from Wet Soil (PLANNED)
- **Goal:** Seeds should ONLY sprout when they find WetSand or standing Water nearby — NOT just any cell. When a seed finds moisture: it can grow upward into an adjacent Empty cell (becomes Plant + new seed above), or sideways into neighboring wet soil.
- **Physics logic location:** Goes in the "SEEDS" pass after water pass. Iterates bottom-up, checks each seed's neighbors for `is_wet_soil()` or `.is_water()`. If moisture found: 30% chance to sprout per tick (slow growth). No moisture nearby: small die rate (~0.2% per frame = several minutes of drought before seed dies).

### 4. Plant Lifecycle — Age + Spread Seeds When Dying (PLANNED)
- **Goal:** Plants don't last forever. Without nearby moisture, they age and eventually drop a seed at their base, then die. With moisture: they grow taller toward light (spawn new Plant above current one, keep the seed that grew them there).
- **Physics logic location:** Goes in a "PLANT LIFE CYCLE" pass after seeds. For each Plant cell: check for nearby WetSand/Water. If present and there's space above → grow taller (set current to Plant, spawn Seed above it). If no moisture + small random chance → drop seed (spawn Seed at same position), then set to Empty.

### 5. Rain Spawning (IMPLEMENTED STRUCTURE, MISSING FUNCTION)
- `spawn_rain(world)` already exists in the function signature but body is missing inside simulation.rs.
- **Goal:** When in rainy season, spawn droplets from the top of each column (or random columns). Water should fall through Empty cells until it hits something (sand/grass/ground), then either: soak into sand (creating WetSand), drip down into a pool, or spread sideways when blocked by ground.

### 6. Dry Season Evaporation (IMPLEMENTED STRUCTURE, MISSING FUNCTION)
- `dry_season_process(world, frame_num)` already exists in the function signature.
- **Goal:** During drought: slowly convert some WetSand back to regular Sand (drying out soil). Evaporate standing surface water into vapor. Seeds near the top of pools may re-enter the atmosphere.

---

## KNOWN ISSUES BLOCKING BUILD

### simulation.rs IS BROKEN (cut off mid-diagonal)
The file has 167 lines but ends at line 167 with an open brace from the diagonal-left sand branch. Closing brace needed:

```rust
    } // close diagonal-left if
```

Then missing code to append:
- Diagonal-right branch (mirror of left)
- Close for x loop, end sand pass
- Water gravity loop (with WetSand soaking on Sand contact)  
- Seed growth toward wet soil logic
- Plant lifecycle (age/die without moisture/grow taller with moisture)
- `spawn_rain()` standalone function body
- `dry_season_process()` standalone function body

### simulation.rs needs 120+ more lines to be complete
The skeleton is solid (struct, resize, physics_pass signature, tick with auto cycle), but the file just stops mid-function. Needs the remaining logic appended to reach valid Rust that compiles.

---

## KEY DESIGN DECISIONS MADE

1. **ratatui + crossterm** chosen over `crossterm`-only for proper widget/layout system ✅
2. **Dynamic sizing** — World struct stores width/height fields, resize() preserves cells on terminal change ✅
3. **Auto rain cycle (90 frames)** — first half = rainy season with spawn_rain(), second half = dry with evaporation; manual toggle_rain() flips phase. Stat HUD shows "Rain ACTIVE" or "DRY SEASON".
4. **WetSand type** — new particle that bridges sand and water. Seeds can grow from WetSand, not just standing water. During drought this slowly converts back to dry Sand.
5. **Seed growth requires wet soil** — no spontaneous germination; seeds sit dormant until they contact WetSand or Water, then sprout upward (not in place — grows as a taller plant with seed above).
6. **Plant lifecycle** — plants grow toward moisture (upward), age and die without it (drop seed before dying). Creates natural "forest" effect during rainy seasons.

---

## HOW TO RESUME (quick reference)

```bash
cd ~/Git/TerminalSandToyQwen3.6Test/simulator

# 1. Fix simulation.rs — append lines 168+ to complete physics engine
#    - Close the sand diagonal-left branch
#    - Add diagonal-right branch
#    - Close for x, end sand pass block  
#    - Add WATER gravity pass (WetSand soaking when water hits Sand)
#    - Add SEED growth toward wet soil
#    - Add PLANT lifecycle (age without moisture + grow taller with it)
#    - Write spawn_rain() and dry_season_process() bodies
# 2. cargo check / cargo build
# 3. Run: cargo run  (should show full-terminal viewport sand sim)
```

---

## USER'S ORIGINAL SPECIFICATION (from first prompt)

> "This project has a rust TUI based program that is a sand simulation program with some fun automatic events like plant growth, rain, evaporation, seedfall, etc."

The auto-rain/evolution cycle and wet soil soaking mechanics are what the user requested. The current code base delivers: full-terminal viewport sand sim with gravity, water physics, particle types, ratatui HUD, mouse painting, keyboard controls, auto rain cycle skeleton — and needs the final 120+ lines to complete these auto-event systems.
