use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Particle {
    #[default]
    Empty = 0,
    Sand,
    Wind,
    Water,
    Seed,
    Plant,
}

impl Particle {
    pub const fn glyph(self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Sand => '\u{2593}',
            Self::Wind => '~',
            Self::Water => '\u{2591}',
            Self::Seed => '\u{00B7}',
            Self::Plant => '\u{2596}',
        }
    }

    pub const fn fg_color(self) -> Color {
        match self {
            Self::Empty => Color::Rgb(17, 8, 4),
            Self::Sand => Color::Rgb(230, 194, 41),
            Self::Wind => Color::Rgb(63, 176, 245),
            Self::Water => Color::Rgb(79, 170, 245),
            Self::Seed => Color::Rgb(148, 226, 213),
            Self::Plant => Color::Rgb(61, 170, 18),
        }
    }

    pub fn is_empty(self) -> bool { self == Self::Empty }

    pub fn is_water(self) -> bool { self == Self::Water }

    pub fn is_plant(self) -> bool { self == Self::Plant }
}