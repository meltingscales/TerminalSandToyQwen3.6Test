use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Particle {
    #[default]
    Empty = 0,
    Sand,
    WetSand,       // sand that has absorbed water
    Water,
    Seed,
    Plant,
}

impl Particle {
    pub const fn glyph(self) -> char {
        match self {
            Self::Empty   => ' ',
            Self::Sand    => '▓', // full block █
            Self::WetSand => '█', // medium block ▌
            Self::Water   => '≈', // wavy line ≈
            Self::Seed    => '·',   // bullet ·
            Self::Plant   => 'P',             // leafy plant
        }
    }

    pub fn fg_color(&self) -> Color {
        match self {
            Self::Empty   => Color::Rgb(56, 28, 14),
            Self::Sand    => Color::Rgb(194, 150, 66),
            Self::WetSand => Color::Rgb(107, 73, 28),
            Self::Water   => Color::Rgb(56, 120, 165),
            Self::Seed    => Color::Rgb(255, 140, 0),
            Self::Plant   => Color::Rgb(34, 139, 34),
        }
    }

    pub fn is_empty(self) -> bool { self == Self::Empty }
    pub fn is_water(self) -> bool { self == Self::Water }
    pub fn is_wet_soil(self) -> bool { self == Self::WetSand }
    pub fn is_plant(self) -> bool { self == Self::Plant }
}
