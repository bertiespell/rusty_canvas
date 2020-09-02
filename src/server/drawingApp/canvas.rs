use drawingApp;

use drawingApp::{
	Dimensions
};

/// Data structure to represent the canvas
pub struct Canvas {
    dimensions: Dimensions,
    pixels: Vec<Box<Pixel>>
}

/// Each pixel represents one space on the canvas
pub struct Pixel {
    occupied: bool,
    character: char
}