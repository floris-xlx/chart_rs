//! # Canvas rendering
//! 

use crate::render_engine::Canvas;
use crate::colors::Color;


impl Canvas {
    /// Create a new canvas
    /// 
    /// # Examples
    /// 
    /// ```
    /// use render_engine::Canvas;
    /// 
    /// let canvas = Canvas::new();
    /// assert_eq!(canvas.width, 0.0);
    /// assert_eq!(canvas.height, 0.0);
    /// assert_eq!(canvas.color, "");
    /// assert_eq!(canvas.opacity, 0.0);
    /// ```
    pub fn new() -> Canvas {
        
        Canvas {
            width: 0.0,
            height: 0.0,
            color: Color::Hex(String::from("")),
            opacity: 0.0
        }
    }


    /// Set the canvas width
    /// 
    /// # Examples
    /// 
    /// ```
    /// use render_engine::Canvas;
    /// 
    /// let mut canvas = Canvas::new();
    /// canvas.set_width(100.0);
    /// assert_eq!(canvas.width, 100.0);
    /// ```
    pub fn set_width(
        &mut self, 
        width: f32
    ) {

        self.width = width;
    }


    /// Set the canvas height
    /// 
    /// # Examples
    /// 
    /// ```
    /// use render_engine::Canvas;
    /// 
    /// let mut canvas = Canvas::new();
    /// canvas.set_height(50.0);
    /// assert_eq!(canvas.height, 50.0);
    /// ```
    pub fn set_height(
        &mut self, 
        height: f32
    ) {

        self.height = height;
    }


    /// Set the canvas color
    /// 
    /// # Examples
    /// 
    /// ```
    /// use render_engine::Canvas;
    /// 
    /// let mut canvas = Canvas::new();
    /// canvas.set_color(String::from("blue"));
    /// assert_eq!(canvas.color, "blue");
    /// ```
    pub fn set_color(
        &mut self, 
        color: Color
    ) {

        self.color = color;
    }


    /// Set the canvas opacity
    /// 
    /// # Examples
    /// 
    /// ```
    /// use render_engine::Canvas;
    /// 
    /// let mut canvas = Canvas::new();
    /// canvas.set_opacity(0.5);
    /// assert_eq!(canvas.opacity, 0.5);
    /// ```
    pub fn set_opacity(
        &mut self, 
        opacity: f32
    ) {

        self.opacity = opacity;
    }
}