//! # Error types and handling

pub mod io;
pub mod render;
pub mod data;
pub mod conversion;


/// Error types for the render engine
pub enum ErrorTypes{
    /// IO Errors: Filesystem, File, etc.
    IoError(IoError),
    /// Render Errors: Drawing, Rendering, etc.
    RenderError(RenderError),
    /// Data Errors: Data processing, Data handling, etc.
    DataError(DataError),
    /// Conversion Errors: Data conversion, Data transformation, etc.
    ConversionError(ConversionError),
}


/// IO Errors for the render engine
pub struct IoError{
    pub message: String,
}

/// Render Errors for the render engine
pub struct RenderError{
    pub message: String,
}

/// Data Errors for the render engine
pub struct DataError{
    pub message: String,
}


/// Conversion Errors for the render engine
pub struct ConversionError{
    pub message: String,
}