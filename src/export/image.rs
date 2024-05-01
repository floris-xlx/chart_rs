//! # Exporting to image types
//! 

use anyhow::Error;
use image::*;

use crate::render_engine::Canvas;
use crate::colors::{Color, Rgb, Hex};

impl Canvas {
    /// Export the canvas to an image and save it to a filepath 
    pub fn export_to_image(
        &self, 
        filepath: &str
    ) -> Result<(), Error> {

        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(
            self.width as u32, 
            self.height as u32
        );

        let rgb_color: Rgba<u8> = match &self.color {
            Color::Rgb(rgb) => Rgba([rgb.r, rgb.g, rgb.b, (self.opacity * 255.0) as u8]),
            
            Color::Hex(hex) => {
                let rgb: Color = Rgb::from_hex(&Hex { hex: hex.clone() });
                
                if let Color::Rgb(rgb) = rgb {
                    Rgba([rgb.r, rgb.g, rgb.b, (self.opacity * 255.0) as u8])
                } else {
                    Rgba([0, 0, 0, (self.opacity * 255.0) as u8]) // Fallback in case of conversion failure
                }
            },
            _ => Rgba([0, 0, 0, (self.opacity * 255.0) as u8]), // Default to black for other color types
        };

        for pixel in img.pixels_mut() {
            *pixel = rgb_color;
        }

        img.save(filepath).map_err(Error::new)?;

        Ok(())
    }
}