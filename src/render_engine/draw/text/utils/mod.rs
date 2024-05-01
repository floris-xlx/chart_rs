//! # Rendering text over the engine

pub mod center;
pub mod wrapping;


/// Text object to be rendered
pub struct TextObject {
    pub text: String,
    pub font_size: f32,
    pub font_family: String,
    pub color: String,
    pub opacity: f32,
    pub x: f32,
    pub y: f32
}