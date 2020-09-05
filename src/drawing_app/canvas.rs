/// Represents a single point on the canvas
#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
} // this could be stored simply as a tuple, with x, y order by convention.

/// Represents the any rectangle, can also be canvas
#[derive(Clone, Debug)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

/// Data structure to represent the canvas
#[derive(Clone, Debug)]
pub struct Canvas {
    pub dimensions: Dimensions,
    pub pixels: Vec<Vec<char>>, // for strictly ASCII we could use a u8 here. Char gives us Unicode which is fun 😉
}

impl Canvas {
    pub fn from_chars(
        ascii: Vec<Vec<char>>, 
        width: i32, 
        height: i32
    ) -> Canvas {
        let canvas = Canvas {
            dimensions: Dimensions {
                width,
                height,
            },
            pixels: ascii,
        };

        return canvas;
    }

    pub fn blank_canvas(width: i32, height: i32, blank_character: char) -> Canvas {
        let canvas = Canvas {
            dimensions: Dimensions {
                width: width,
                height: height,
            },
            pixels: vec![vec![blank_character; width as usize]; height as usize]
        };

        return canvas;
    }

    pub fn to_string(&self) -> String { 
        self.pixels
            .iter()
            .map(|row| {
                row
                    .iter()
                    .collect::<String>()
            })
            .fold(String::new(), |a, b| a + &b + "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use drawing_app::{commands, canvas};

    #[test]
    fn test_new_canvas() {
        let actual = canvas::Canvas::blank_canvas(21, 9, ' ').to_string();
        let expected = "                     \n                     \n                     \n                     \n                     \n                     \n                     \n                     \n                     \n";

        assert_eq!(expected, actual);
    }
}