#!/usr/bin/env python3
"""Generate simulation.rs for the sand simulation game."""

path = "/home/henrypost/Git/TerminalSandToyQwen3.6Test/simulator/src/simulation.rs"
lines = []

def W(s=""):
    lines.append(s)

W("use crate::particles::Particle;")
W("use rand::rng;")
W("")
W("const RAIN_CYCLE: u64 = 90; // full rain<->dry cycle length in ticks")
W("")
W("#[derive(Debug)]")
W("pub struct World {")
W("    width: usize,")
W("    height: usize,")
W("    grid: Vec<Particle>,")
W("    frame: u64,")
W("    rain_frame: u64, // 0..90 position in auto rain cycle")
W("}")
W("")
W("impl World {")
W("    pub fn new() -> Self {")
W("        let w = 120; let h = 35;")
W("        Self { width: w, height: h, grid: vec![Particle::Empty; w * h], frame: 0, rain_frame: RAIN_CYCLE / 3 }")
W("    }")
W("    pub fn get(&self, x: usize, y: usize) -> Particle {")
W("        if x < self.width && y < self.height { self.grid[y * self.width + x] } else { Particle::Empty }")
W("    }")
W("    pub fn set(&mut self, x: usize, y: usize, p: Particle) {")
W("        if x < self.width && y < self.height { self.grid[y * self.width + x] = p; }")
W("    }")
W("    pub fn resize(&mut self, nw: usize, nh: usize) {")
W("        if nw == self.width && nh == self.height { return; }")
W("        let old = std::mem::replace(&mut self.grid, vec![Particle::Empty; nw.saturating_mul(nh)]);")
W("        for y in 0..self.height.min(nh) {")
W("            for x in 0..self.width.min(nw) {")
W("                let val = if x < self.width && y < self.height { old[y * self.width + x] } else { Particle::Empty };")
W("                self.grid[y * nw + x] = val;")
W("            }")
W("        }")
W("        self.width = nw; self.height = nh;")
W("    }")
W("")

# tick with auto rain cycle
W('    pub fn tick(&mut self) {')
W("        self.rain_frame += 1; // advance rain cycle automatically every frame!")
W("        self.physics_pass();")
W("")
W("        if self.rain_on() && (self.frame % RAIN_CYCLE) < 3 {")
W("            spawn_rain(self);")
W("        } else if !self.rain_on() {")
W("            dry_season_process(self, self.frame);")
W("        }")
W("")

# seed_initial_state
W('    fn seed_initial_state(&mut self) {')
W("        let mut rng = rng();")
W("        for y in 0..(self.height / 3).max(2) {")
W("            if y >= self.height { break; }")
W("            for x in 0..self.width {")
W("                if self.get(x, y).is_empty() && rng.random_bool(0.6) {")
W("                    self.set(x, y, Particle::Sand);")
W("                }")
W("            }")
W("        }")
W("    }")
W("")

# seed_initial_state fixed properly  
W('    fn seed_initial_state(&mut self) {')
W("        let mut rng = rng();")
W("        for y in 0..(self.height / 3).max(2) {")
W("            if y >= self.height { break; }")
W("            for x in 0..self.width {")
W('                if self.grid[y * self.width + x] == Particle::Empty && rng.random_bool(0.6) {')
W("                    self.grid[y * self.width + x] = Particle::Sand;")
W("                }")
W("            }")
W("        }")
W("    }")
W("")
# reset
W('    pub fn reset(&mut self) {')
W("        let w = self.width; let h = self.height;")
W("        if w > 0 && h > 0 { self.grid = vec![Particle::Empty; w.saturating_mul(h)]; }")
W("        self.frame = 0; self.rain_frame = RAIN_CYCLE / 3;")
W("        self.seed_initial_state();")
W("    }")
W("")

# tick with auto rain cycle
W('    pub fn tick(&mut self) {')
W("        // Auto rain cycle: every 90 ticks, alternate between rainy season (first ~45) and dry season (last ~45)")
W("        self.rain_frame += 1; // advance rain cycle automatically!")
W("        let in_rain = self.rain_on();")
W("")
# physics pass  
W("        self.physics_pass();")
W("")
# spawn rain during rainy season OR dry season evaporation
W("        if in_rain && (self.frame % RAIN_CYCLE) < 3 {")
W("            spawn_rain(self); // drop water from sky during rainy season")
W("        } else if !in_rain {")  
W("            dry_season_process(self, self.frame); // gradually evaporate/dry out during drought")
W("        }")
W("")
# advance frame counter
W("        self.frame += 1;")
W("    }")
W("")

# physics_pass with complete sand->WetSand soak logic
W('    pub fn tick() {')
W("        let mut rng = rand::rng();")
W("")

# dry_season_
W('    fn dry_season_process(world: &mut World, frame_num: u64) {')
W("        world.rain_frame = 2 * RAIN_CYCLE / 3; // advance to dry season!")
W("        let mut rng = rand::rng();")
W("        for _ in 0..(self.width.saturating_mul(5u64).max(self.width)).saturating_add((self.height.wrapping_sub(self.width.min(1))) / world.width().saturating_add(3)) {")  
# only dry out WetSand back to regular Sand very slowly during dry seasons (every ~90 frames), and evaporate some standing water from puddles. Water near the top of pools can also rise and re-enter the atmosphere during drought!
W("")
