use crate::particles::Particle;
use rand::Rng;

const WIDTH: usize = 140;
const HEIGHT: usize = 35;

/// Simulation grid state. Flat Vec<Particle> indexed by y*WIDTH+x.
#[derive(Debug)]
pub struct World {
    grid: Vec<Particle>,
    frame: u64,
    rain_active: bool,
}

impl World {
    pub fn new() -> Self {
        let mut w = Self {
            grid: vec![Particle::Empty; WIDTH * HEIGHT],
            frame: 0,
            rain_active: true,
        };
        w.seed_initial_state();
        w
    }

    pub fn width(&self) -> usize { WIDTH }
    pub fn height(&self) -> usize { HEIGHT }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x < WIDTH && y < HEIGHT {
            self.grid[y * WIDTH + x]
        } else {
            Particle::Empty
        }
    }

    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < WIDTH && y < HEIGHT {
            self.grid[y * WIDTH + x] = p;
        }
    }

    pub fn count(&self, kind: Particle) -> usize {
        self.grid.iter().filter(|&&p| p == kind).count()
    }

    pub fn rain_on(&self) -> bool { self.rain_active }

    pub fn toggle_rain(&mut self) {
        self.rain_active = !self.rain_active;
    }

    /// Paint a vertical brush stroke at column x starting from row y downward.
    /// Cycles through Sand / Water / Seed / Plant for variety.
    pub fn paint_at(&mut self, x: usize, start_y: usize) {
        let mut y = start_y;
        let types = [Particle::Sand, Particle::Water, Particle::Seed, Particle::Plant];
        while y < HEIGHT && self.get(x, y).is_empty() {
            self.set(x, y, types[(y * 7) % 4]);
            y += 1;
        }
    }

    fn seed_initial_state(&mut self) {
        let mut rng = rand::rng();
        // Sand in top half of the grid
        for y in 0..HEIGHT / 2 {
            for x in 0..WIDTH {
                if rng.random_bool(0.6) {
                    self.set(x, y, Particle::Sand);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.grid = vec![Particle::Empty; WIDTH * HEIGHT];
        self.frame = 0;
        self.seed_initial_state();
    }

    /* ── Main simulation tick ───────────────────────────────*/

    pub fn tick(&mut self) {
        self.physics_pass();
        if self.rain_active && self.frame % 15 < 3 {
            spawn_rain(self);
        }
        evaporate(self, self.frame);
    }

    /* ── Physics passes (one complete tick per call) ────────*/

    fn physics_pass(&mut self) {
        let mut rng = rand::rng();

        // SAND: fall down / diagonally into empty cells
        for y in (0..HEIGHT - 1).rev() {
            let rev = y % 2 == 0;                          // alternate scan direction per row
            let xs: Vec<usize> = if rev {
                (0..WIDTH).collect()
            } else {
                (0..WIDTH).rev().collect()
            };

            for &x in &xs {
                if self.get(x, y) != Particle::Sand { continue; }

                // Priority 1: straight down?
                if self.get(x, y + 1) == Particle::Empty {
                    self.set(x, y + 1, Particle::Sand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                // Priority 2: diagonal down-left or right (random pick)
                if x > 0 && self.get(x.wrapping_sub(1), y + 1) == Particle::Empty
                    && (rng.random_bool(0.5) || !rev) {
                    self.set(x.wrapping_sub(1), y + 1, Particle::Sand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                if x + 1 < WIDTH && self.get(x + 1, y + 1) == Particle::Empty
                    && (rng.random_bool(0.5) || rev) {
                    self.set(x + 1, y + 1, Particle::Sand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }
            }
        }

        // WATER: drip down then flow sideways
        for y in (0..HEIGHT - 1).rev() {
            let rev = y % 2 == 0;
            let xs: Vec<usize> = if rev {
                (0..WIDTH).collect()
            } else {
                (0..WIDTH).rev().collect()
            };

            for &x in &xs {
                if self.get(x, y) != Particle::Water { continue; }

                // Down?
                if self.get(x, y + 1) == Particle::Empty {
                    self.set(x, y + 1, Particle::Water);
                    self.set(x, y, Particle::Empty);
                    continue;  // skip horizontal spread this pass if moved down already!
                }

                // Horizontal spread: alternate left/right bias via rng
                let spread = rng.random_range(0..4);
                if spread < 2 && x > 0 && self.get(x.wrapping_sub(1), y) == Particle::Empty {
                    self.set(x.wrapping_sub(1), y, Particle::Water); // flow left
                } else if spread > 1 && x + 1 < WIDTH && self.get(x + 1, y) == Particle::Empty {
                    self.set(x + 1, y, Particle::Water); // flow right!
                }
            }
        }

        // SEED GROWTH: seeds sprout upward & sideways when near water.
        for y in (0..HEIGHT).rev() {
            let rev = y % 2 == 0;
            let xs: Vec<usize> = if rev {
                (0..WIDTH).collect()
            } else {
                (0..WIDTH).rev().collect()
            };

            for &x in &xs {
                if self.get(x, y) != Particle::Seed || y == 0 { continue; }

                // Check adjacents for water to feed the seed.
                let has_adj_water = (x > 0 && self.get(x.wrapping_sub(1), y).is_water())
                    || (x + 1 < WIDTH && self.get(x + 1, y).is_water())
                    || (y > 0 && self.get(x, y.wrapping_sub(1)).is_water());

                if has_adj_water {
                    // Grow upward: seed becomes plant here, spawn new seed above.
                    if self.get(x, y.wrapping_sub(1)) == Particle::Empty {
                        self.set(x, y, Particle::Plant);
                        self.set(x, y.wrapping_sub(1), Particle::Seed);
                    } else {
                        // No space above: try sideways growth into empty cell.
                        let dirs = if rev {
                            [x.wrapping_sub(1).saturating_sub(0), x + 1]
                        } else {
                            [x + 1, x.wrapping_sub(1)]
                        };
                        for &nx in &dirs {
                            if nx < WIDTH && self.get(nx, y) == Particle::Empty {
                                // Only expand if there is water below the target or adjacent plant to branch from.
                                let valid = (y > 0 && self.get(nx, y.wrapping_sub(1)).is_water())
                                    || (nx > 0 && self.get(nx.wrapping_sub(1), y).is_plant())
                                    || (nx + 1 < WIDTH && self.get(nx + 1, y).is_plant());

                                if valid {
                                    self.set(nx, y, Particle::Plant);
                                    break;
                                }
                            }
                        }
                    }
                } else if rng.random_bool(0.02) {
                    // No moisture nearby: slow death chance per tick
                    self.set(x, y, Particle::Empty);
                }
            }
        }

        self.frame += 1;
    }
}

/* ── Environment helpers (standalone functions) ───────────*/

fn spawn_rain(world: &mut World) {
    let mut rng = rand::rng();
    for _ in 0..3 {
        let x = rng.random_range(1..WIDTH - 1);
        let mut y = 0;
        while y + 1 < HEIGHT && world.get(x, y).is_empty() {
            y += 1;
        }
        if y > 0 {
            world.set(x, y.saturating_sub(1), Particle::Water);
        }
    }
}

fn evaporate(world: &mut World, frame: u64) {
    let mut rng = rand::rng();
    // Only evaporate a few water grains every 30 frames (slows things down)
    if !frame.is_multiple_of(30) { return; }
    for _ in 0..5 {
        let x = rng.random_range(0..WIDTH);
        let y = rng.random_range(1..HEIGHT);
        if world.get(x, y) == Particle::Water && rng.random_bool(0.1) {
            world.set(x, y, Particle::Empty);
        }
    }
}