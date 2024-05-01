//! # Color converting 
//! 

use crate::colors::Color;
use crate::colors::{Hex, Rgb};


/// Convert a string to a hex color
impl Hex {
    /// Creates a Hex color from a string
    ///
    /// # Arguments
    /// * `hex_str` - A string slice that holds the hex color code
    ///
    /// # Examples
    /// ```
    /// let color = Color::Hex(String::from("#FFFFFF"));
    /// ```
    pub fn from_string(
        hex_str: &str
    ) -> Color {
    
        Color::Hex(String::from(hex_str))
    }
}


impl Rgb {
    /// Creates a Rgb color from a hex
    ///
    /// # Arguments
    /// * `hex` - A hex color
    /// 
    /// # Examples
    /// ```
    /// let color = Color::from_hex(&Hex { hex: String::from("#FFFFFF") });
    /// 
    /// assert_eq!(color, Color::Rgb(Rgb { r: 255, g: 255, b: 255 }));
    /// ```
    pub fn from_hex(
        hex: &Hex
    ) -> Color {
        let hex_code: &&str = &hex.hex.trim_start_matches('#');

        if hex_code.len() == 6 {
            let r: u8 = u8::from_str_radix(&hex_code[0..2], 16).unwrap_or(0);
            let g: u8 = u8::from_str_radix(&hex_code[2..4], 16).unwrap_or(0);
            let b: u8 = u8::from_str_radix(&hex_code[4..6], 16).unwrap_or(0);

            Color::Rgb(Rgb { r, g, b })

        } else {
            Color::Rgb(Rgb { r: 0, g: 0, b: 0 })
        }


    }
}
