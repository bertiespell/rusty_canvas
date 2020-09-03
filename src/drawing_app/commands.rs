/// Represents a rectangle drawing operation
pub struct DrawRectangleCommand {
    pub position: super::canvas::Point, // upper-left corner start coordinates
    pub dimensions: super::canvas::Dimensions,
    pub fill: char, // character used to fill rectangle
    pub outline: char // character used to outline rectangle
}

/// Repreents a flood fill drawing operation
pub struct FloodFillCommand {
    pub position: super::canvas::Point, // start coordinates.
    pub fill: char, // character used to fill empty spaces on canvas
}

impl DrawRectangleCommand {
    pub fn new() -> DrawRectangleCommand {
        DrawRectangleCommand {
            position: super::canvas::Point{ x: 3, y: 2},
            dimensions: super::canvas::Dimensions {
                width: 5,
                height: 3
            },
            fill: 'X',
            outline: '@'
        }
    }
}