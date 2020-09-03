/// Represents a single point on the canvas
pub struct Point {
    pub x: i32,
    pub y: i32,
} // this could be stored simply as a tuple, with x, y order by convention.

/// Represents the any rectangle, can also be canvas
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

/// Data structure to represent the canvas
pub struct Canvas {
    dimensions: Dimensions,
    pixels: Vec<Vec<Pixel>>
}

/// Each pixel represents one space on the canvas
#[derive(Clone, Copy)]
pub struct Pixel {
    occupied: bool,
    character: char // for strictly ASCII we could use a u8 here. Char gives us Unicode
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let canvas = Canvas {
            dimensions: Dimensions {
                width: width,
                height: height,
            },
            pixels: vec![vec![Pixel{ occupied: false, character: ' '}; height as usize]; width as usize]
        };

        return canvas;
    }

    pub fn to_string(&self) -> String { 
        unimplemented!()
    }
}
