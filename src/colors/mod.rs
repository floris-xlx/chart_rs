//! # Color utilities
//!  wip: make a main router to route what color was chosen etc

pub mod convert;


/// ## Color
/// 
/// ### Fields
/// - `Rgb` - RGB color
/// - `Hex` - Hex color
/// - `Standard` - Standard color
#[derive(Debug, Clone)]
pub enum Color {
    Rgb(Rgb),
    Hex(String),
    Standard
}

/// ## Hex Color
/// 
/// ### Fields
/// - `hex` - Hex color code
/// 
pub struct Hex{
    pub hex: String,
}

/// ## RGB Color
/// 
/// ### Fields
/// - `r` - Red
/// - `g` - Green
/// - `b` - Blue
#[derive(Debug, Clone)]
pub struct Rgb{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}



/// ## Standard colors
///  still needs routing to the standard colors via hex probably
/// 
/// ### Fields
/// - `Black` - Black
/// - `White` - White
/// - `Red` - Red
/// - `Green` - Green
/// - `Blue` - Blue
/// - `Yellow` - Yellow
/// - `Cyan` - Cyan
/// - `Magenta` - Magenta
/// - `Gray` - Gray
/// - `DarkGray` - Dark Gray
/// - `LightGray` - Light Gray
/// - `Orange` - Orange
/// - `Pink` - Pink
/// - `Purple` - Purple
/// - `Brown` - Brown
/// - `Maroon` - Maroon
/// - `Olive` - Olive
/// - `Teal` - Teal
/// - `Navy` - Navy
/// - `Indigo` - Indigo
/// - `Turquoise` - Turquoise
/// - `Violet` - Violet
/// - `Gold` - Gold
/// - `Silver` - Silver
/// - `Bronze` - Bronze
/// - `Copper` - Copper
/// - `Platinum` - Platinum
/// - `RoseGold` - Rose Gold
/// - `Brass` - Brass
///      
/// 
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