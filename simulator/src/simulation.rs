use crate::particles::Particle;
use rand::Rng;
use rand::rng;

const RAIN_CYCLE_LENGTH: u64 = 90;

#[derive(Debug)]
pub struct World {
    width: usize,
    height: usize,
    grid: Vec<Particle>,
    frame: u64,
    rain_frame: u64, // tracks position in the auto rain cycle (0..90)
}

impl World {
    pub fn new() -> Self {
        let w = 120;
        let h = 35;
        Self {
            width: w,
            height: h,
            grid: vec![Particle::Empty; w * h],
            frame: 0,
            rain_frame: RAIN_CYCLE_LENGTH / 3, // start in middle of rainy season
        }
    }

    pub fn resize(&mut self, new_w: usize, new_h: usize) {
        if new_w == self.width && new_h == self.height {
            return;
        }
        let old = std::mem::replace(
            &mut self.grid,
            vec![Particle::Empty; new_w.saturating_mul(new_h)],
        );
        for y in 0..self.height.min(new_h) {
            for x in 0..self.width.min(new_w) {
                let val = if x < self.width && y < self.height {
                    old[y * self.width + x]
                } else {
                    Particle::Empty
                };
                self.grid[y * new_w + x] = val;
            }
        }
        self.width = new_w;
        self.height = new_h;
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x]
        } else {
            Particle::Empty
        }
    }

    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = p;
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn count(&self, kind: Particle) -> usize {
        self.grid.iter().filter(|&&p| p == kind).count()
    }

    /// True during rainy season (first half of cycle: frames 0..45)
    pub fn rain_on(&self) -> bool {
        (self.rain_frame % RAIN_CYCLE_LENGTH) < (RAIN_CYCLE_LENGTH / 2)
    }

    pub fn weather_status(&self) -> &str {
        if self.rain_on() { "Rain ACTIVE" } else { "DRY SEASON" }
    }

    /// Toggle rains manually (user can force a cycle phase switch)
    pub fn toggle_rain(&mut self) {
        if self.rain_on() {
            self.rain_frame = 2 * RAIN_CYCLE_LENGTH / 3; // jump to dry season
        } else {
            self.rain_frame = RAIN_CYCLE_LENGTH / 3; // jump to rainy season
        }
    }

    pub fn paint_at(&mut self, x: usize, start_y: usize) {
        if x >= self.width || start_y >= self.height { return; }
        let mut y = start_y;
        let types = [Particle::Sand, Particle::Water, Particle::Seed, Particle::Plant];
        while y < self.height && self.get(x, y).is_empty() {
            self.set(x, y, types[(y * 7) % 4]);
            y += 1;
        }
    }

    fn seed_initial_state(&mut self) {
        let mut rng = rng();
        // Fill the bottom portion with sand to create a base
        for y in (self.height / 2..self.height).rev() {
            for x in 0..self.width {
                if rng.random_bool(0.8) {
                    self.set(x, y, Particle::Sand);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        let w = self.width;
        let h = self.height;
        self.grid = vec![Particle::Empty; w.saturating_mul(h)];
        self.frame = 0;
        self.rain_frame = RAIN_CYCLE_LENGTH / 3;
        self.seed_initial_state();
    }

    pub fn tick(&mut self) {
        self.rain_frame += 1; // advance rain cycle every frame
        self.physics_pass();

        if self.rain_on() && (self.frame % RAIN_CYCLE_LENGTH) < 3 {
            spawn_rain(self);
        } 
        
        spawn_seeds(self);

        if !self.rain_on() {
            evaporate_and_soak(self, self.frame);
        }

        self.frame += 1;
    }

    fn physics_pass(&mut self) {
        let mut rng = rng();

        // 1) SAND: fall down / diagonally into empty cells
        for y in (0..self.height.saturating_sub(1)).rev() {
            let xs: Vec<usize> = if y % 2 == 0 {
                (0..self.width).collect()
            } else {
                (0..self.width).rev().collect()
            };
            for x in xs {
                if self.get(x, y) != Particle::Sand { continue; }

                // down
                if self.get(x, y + 1).is_empty() {
                    self.set(x, y + 1, Particle::Sand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                // Blocked by water: sand falls through but saturates below -> WetSand
                if self.get(x, y + 1) == Particle::Water {
                    self.set(x, y + 1, Particle::WetSand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                // diagonal down-left
                if x > 0 && self.get(x - 1, y + 1).is_empty() {
                    if y % 2 == 0 || rng.random_bool(0.5) {
                        self.set(x - 1, y + 1, Particle::Sand);
                        self.set(x, y, Particle::Empty);
                        continue;
                    }
                }

                // diagonal down-right
                if x < self.width - 1 && self.get(x + 1, y + 1).is_empty() {
                    if y % 2 != 0 || rng.random_bool(0.5) {
                        self.set(x + 1, y + 1, Particle::Sand);
                        self.set(x, y, Particle::Empty);
                        continue;
                    }
                }
            }
        }

        // 2) WATER: flow down / sideways
        for y in (0..self.height.saturating_sub(1)).rev() {
            for x in 0..self.width {
                if self.get(x, y) != Particle::Water { continue; }

                // down
                if self.get(x, y + 1).is_empty() {
                    self.set(x, y + 1, Particle::Water);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                // soak sand into wet sand
                if self.get(x, y + 1) == Particle::Sand {
                    self.set(x, y + 1, Particle::WetSand);
                    self.set(x, y, Particle::Empty);
                    continue;
                }

                // sideways / diagonal flow
                let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
                let mut moved = false;
                for (dx, dy) in directions {
                    let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                    if self.get(nx, ny).is_empty() {
                        self.set(nx, ny, Particle::Water);
                        self.set(x, y, Particle::Empty);
                        moved = true;
                        break;
                    }
                }
                if moved { continue; }

                // simple horizontal flow if blocked vertically
                let side_directions = [(-1, 0), (1, 0)];
                for (dx, _) in side_directions {
                    let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                    if self.get(nx, y).is_empty() {
                        self.set(nx, y, Particle::Water);
                        self.set(x, y, Particle::Empty);
                        break;
                    }
                }
            }
        }

        // 3) SEEDS & PLANTS
        for y in 0..self.height {
            for x in 0..self.width {
                let p = self.get(x, y);
                if p == Particle::Seed {
                    let mut has_moisture = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                            let neighbor = self.get(nx, ny);
                            if neighbor.is_water() || neighbor.is_wet_soil() {
                                has_moisture = true;
                                break;
                            }
                        }
                        if has_moisture { break; }
                    }

                    if has_moisture {
                        if rng.random_bool(0.05) && y > 0 && self.get(x, y - 1).is_empty() {
                           self.set(x, y, Particle::Plant);
                        }
                    } else if rng.random_bool(0.001) {
                        self.set(x, y, Particle::Empty);
                    }
                } else if p == Particle::Plant {
                    let mut has_moisture = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                            let neighbor = self.get(nx, ny);
                            if neighbor.is_water() || neighbor.is_wet_soil() {
                                has_moisture = true;
                                break;
                            }
                        }
                        if has_moisture { break; }
                    }

                    if has_moisture {
                        if rng.random_bool(0.02) && y > 0 && self.get(x, y - 1).is_empty() {
                            self.set(x, y - 1, Particle::Plant);
                            if rng.random_bool(0.1) {
                                self.set(x, y, Particle::Seed);
                            }
                        }
                    } else if rng.random_bool(0.005) {
                        if rng.random_bool(0.5) {
                            self.set(x, y, Particle::Seed);
                        } else {
                            self.set(x, y, Particle::Empty);
                        }
                    }
                }
            }
        }
    }
}

fn spawn_rain(world: &mut World) {
    let mut rng = rng();
    for _ in 0..3 {
        let x = rng.random_range(0..world.width);
        world.set(x, 0, Particle::Water);
    }
}

fn spawn_seeds(world: &mut World) {
    let mut rng = rng();
    // Randomly drop seeds from the sky
    if rng.random_bool(0.01) {
        for _ in 0..2 {
            let x = rng.random_range(0..world.width);
            world.set(x, 0, Particle::Seed);
        }
    }
}

fn evaporate_and_soak(world: &mut World, _frame: u64) {
    let mut rng = rng();
    for y in 0..world.height {
        for x in 0..world.width {
            let p = world.get(x, y);
            if p == Particle::WetSand {
                // Dry out soil
                if rng.random_bool(0.005) {
                    world.set(x, y, Particle::Sand);
                }
            } else if p == Particle::Water {
                // Evaporate water
                if rng.random_bool(0.002) {
                    world.set(x, y, Particle::Empty);
                }
            }
        }
    }
}
