//! # Render Engine 

pub mod charts;
pub mod figures;
pub mod utils;
pub mod draw;
pub mod canvas;
pub mod legend;


use crate::colors::Color;


/// ## A canvas is a rectangle where the chart is drawn.
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub opacity: f32,
}