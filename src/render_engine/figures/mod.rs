//! This module contains all the figures that can be drawn on the chart.

pub mod candle;

use crate::colors::Color;


pub struct Candlestick {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub color_body: Color,
    pub color_wick: Color
}