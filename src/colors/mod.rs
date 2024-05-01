//! # Color utilities
//!  wip: make a main router to route what color was chosen etc

pub mod convert;


/// ## Color
#[derive(Debug, Clone)]
pub enum Color {
    Rgb(Rgb),
    Hex(String),
    Standard
}

/// ## Hex Color
pub struct Hex{
    pub hex: String,
}

/// ## RGB Color
#[derive(Debug, Clone)]
pub struct Rgb{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}



/// ## Standard colors
///  still needs routing to the standard colors via hex probably
pub enum Standard{
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Gray,
    DarkGray,
    LightGray,
    Orange,
    Pink,
    Purple,
    Brown,
    Maroon,
    Olive,
    Teal,
    Navy,
    Indigo,
    Turquoise,
    Violet,
    Gold,
    Silver,
    Bronze,
    Copper,
    Platinum,
    RoseGold,
    Brass
}