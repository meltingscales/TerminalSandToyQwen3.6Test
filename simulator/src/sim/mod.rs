// ─── Types / Enum constants for cell occupancy kinds ────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entity {
    Empty,
    Sand,
    Water,
    Seed,
    Plant,
}

impl Default for Entity {
    fn default() -> Self { Entity::Empty }
}

// Render colours (ansi 256 palette).
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (fg_reset, ch) = match self {
            Entity::Empty      => ("\x1b[40m", '\u{2800}'), // dark-block space char
            Entity::Sand       => ("#e6c229", '▓'),
            Entity::Water      => ("#4faaf5", '░'),
            Entity::Seed       => ("#93bb5f", '·'),
            Entity::Plant      => ("#3daa12", '█'),
        };
        // Write the ANSI foreground + background reset sequence before the glyph:
        write!(f, "\x1b[4m;fg {};{}", fg_reset, ch)
    }
}

// ─── World struct that owns grid state / frame count / config ──────
pub struct World {
    width:  usize,
    height: usize,
    grid:   Vec<Entity>,                // flattened [y * width + x]
    frame:  u64,                        // total simulation frames elapsed
    rain_on: bool,                      // user toggle for rain events
}

impl World {
    // -- Constructor ------------------------------------------------------------

    pub fn new(w: usize, h: usize) -> Self {
        let mut wld = Self {
            width:  w,
            height: h,
            grid:   vec![Entity::Empty; w * h],
            frame:  0,
            rain_on: true,
        };
        wld.seed_initial_state();
        wld
    }

    // -- Core update / render loop helpers -------------------------------------

    pub fn tick(&mut self) {
        // Apply physics sweep (top-down pass so grains always fall *down*).
        self.physics_step();

        // Add environmental events (rain, evaporation, seed-fall).
        if self.rain_on { self.spawn_rain(); }
        self.evaporate();
        self.seed_fall();

        self.frame += 1;
    }

    pub fn render(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error> {
        let mut buf = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                buf.push_str(&format!("{}", self.grid[y * self.width + x]));
            }
            buf.push('\n');
        }
        out.write_all(buf.as_bytes())?;
        // Clear screen and move cursor to top-left corner for next redraw:
        write!(out, "\x1b[H\x1b[J")?;
        Ok(())
    }

    pub fn toggle_rain(&mut self) { self.rain_on = !self.rain_on; }
    pub fn reset(&mut self)       { self.frame = 0; self.seed_initial_state(); }

    // -- Helpers (private) -----------------------------------------------------

    #[allow(dead_code)] // used in tests / feature-gated features
    fn idx(&self, x: usize, y: usize) -> usize { y * self.width + x }

    #[allow(dead_code)]
    fn get(&self, x: usize, y: usize) -> Entity {
        if x < self.width && y < self.height {
            self.grid[self.idx(x, y)]
        } else { Entity::Empty }
    }

    #[allow(dead_code)]
    fn set(&mut self, x: usize, y: usize, e: Entity) {
        if x < self.width && y < self.height {
            self.grid[self.idx(x, y)] = e;
        }
    }

    // -- Environment events (procedural generation helpers) ---------------------

    fn spawn_rain(&mut self) {
        const RAIN_CHANCE: f64 = 0.12; // 12 % chance each tick per column
        for x in 0..self.width {
            if rand::random::<f64>() < RAIN_CHANCE {
                // Pour water down until we hit solid ground (sand/plant).
                let mut y = 0;
                while y < self.height && self.get(x, y) == Entity::Empty {
                    y += 1;
                }
                if y > 0 { self.set(x, y.saturating_sub(1), Entity::Water); }
            }
        }
    }

    fn evaporate(&mut self) {
        const EVAP_RATE: f64 = 0.02;
        for y in (1..self.height).rev() {
            for x in 0..self.width {
                if self.get(x, y) == Entity::Water && rand::random::<f64>() < EVAP_RATE {
                    self.set(x, y, Entity::Empty); // water disappears into air!
                }
            }
        }
    }

    fn seed_fall(&mut self) {
        const SEED_CHANCE: f64 = 0.005; // Very rare, naturally random drops from sky
        if rand::random::<f64>() < SEED_CHANCE && self.frame % 3 == 0 {
            let x = rand::thread_rng().gen_range(1..self.width - 1);
            let mut y = 0;
            while y + 1 < self.height && self.get(x, y) == Entity::Empty { y += 1; }
            if y > 0 { self.set(x, y.saturating_sub(1), Entity::Seed); }
        }
    }

    // -- Physics step: gravity / sand flow / plant growth ----------------------

    fn physics_step(&mut self) {
        let w = self.width;
        let h = self.height;

        // Process rows top → bottom so grains fall naturally (never jump past layers).
        for y in 0..h {
            // Left-to-right and right-to-left alternating scan-directions for variety:
            let direction_sign = if (y % 2) == 0 { -1i32 } else { 1 };

            let mut x_start: i32;
            let mut x_end:   i32;
            let mut step:    i32;

            if direction_sign > 0 {
                x_start = 0;   x_end   = w as i32 - 1;   step = 1;
            } else {
                x_start = (w as i32) - 1;   x_end = 0;    step = -1;
            }

            // Sand grains: try to drop diagonally / straight down.
            for xi in (x_start..=x_end).step_by(step.abs() as usize) {
                let xi_u = xi as usize;
                if self.get(xi_u, y) != Entity::Sand { continue; }

                // Below?
                if y + 1 < h && self.get(xi_u, y + 1) == Entity::Empty {
                    self.set(xi_u, y + 1, Entity::Sand);
                    self.set(xi_u, y,     Entity::Empty);
                // Diagonal down-left?
                } else if xi > 0 && y + 1 < h && self.get(xi - 1, y + 1) == Entity::Empty {
                    self.set(xi - 1, y + 1, Entity::Sand);
                    self.set(xi,     y,     Entity::Empty);
                // Diagonal down-right?
                } else if xi < (w as i32) - 1 && y + 1 < h && self.get(xi + 1, y + 1) == Entity::Empty {
                    self.set(xi + 1, y + 1, Entity::Sand);
                    self.set(xi,     y,     Entity::Empty);
                }
            }

            // Water grains: drip down / flow sideways more freely.
            for xi in (x_start..=x_end).step_by(step.abs() as usize) {
                let xi_u = xi as usize;
                if self.get(xi_u, y) != Entity::Water { continue; }

                // Below?
                if y + 1 < h && self.get(xi_u, y + 1) == Entity::Empty {
                    self.set(xi_u, y + 1, Entity::Water);
                    self.set(xi_u, y,     Entity::Empty);
                // Sideways (any direction) with equal preference:
                } else if !self.try_side_flow(xi_u, y, w, h) {
                    continue;
                }
            }

            // Plant growth: seeds expand upward + sideways when watered nearby.
            for xi in (x_start..=x_end).step_by(step.abs() as usize) {
                let xi_u = xi as usize;
                if self.get(xi_u, y) != Entity::Seed { continue; }

                // Grow straight up if space above is empty:
                if y > 0 && self.get(xi_u, y - 1) == Entity::Empty {
                    // Check neighbouring water for food (evaporation model):
                    let has_water_adjacent = (xi as i32 + 1..=(w.saturating_sub(1)) as i32)
                        .chain((1_i32..=xi as i32).rev())
                        .any(|dx| self.get(dx.max(0) as usize, y) == Entity::Water)
                        || (y > 0 && self.get(xi_u, y - 1) == Entity::Water);

                    if has_water_adjacent {
                        // Turn seed into plant stem:
                        self.set(xi_u, y, Entity::Plant);
                        self.set(xi_u, y.saturating_sub(1), Entity::Seed); // push "growing tip" upward!
                    } else {
                        // Without water nearby the seed simply dies (evaporates):
                        self.set(xi_u, y, Entity::Empty);
                    }
                }

                // Once a plant segment exists at this cell, it can grow sideways:
                if self.get(xi_u, y) == Entity::Plant {
                    let dirs = &[-1i32, 1];
                    for &d in dirs {
                        let nx = (xi + d).max(0) as usize;
                        // Only expand into an empty cell that has water below it:
                        if self.get(nx, y) == Entity::Empty && y + 1 < h && self.get(nx, y + 1) == Entity::Water {
                            self.set(nx, y, Entity::Plant);
                        }
                    }
                }
            }
        }
    }

    // Helper: attempt single-direction side-flow for water.
    fn try_side_flow(&mut self, x: usize, y: usize, w: usize, h: usize) -> bool {
        // Flip direction each frame via a deterministic alternating pattern
        // based on coordinate parity (avoids bias issues that could otherwise emerge).
        let flip = ((x + y / 2) % 2) == 0;

        #[allow(unreachable_patterns)]
        if flip || rand::random::<f64>() < 0.5 { // Alternate directions each tick pass!
            // Try right, then left
            if x + 1 < w && self.get(x + 1, y) == Entity::Water
                || x < w && self.get(x + 1, y) == Entity::Empty {
                return true; // Already water there or empty to flow into. Skip processing this direction!
        } else if rand::random::<f64>() < 0.3 {
            if x > 0 && self.get(x - 1, y) == Entity::Empty {
                self.set(x - 1, y, Entity::Water);
                self.set(x,     y, Entity::Water);
                return true;
            }
        }
        false // Nothing worked out so just leave alone for now until next update tick!
    }

}
