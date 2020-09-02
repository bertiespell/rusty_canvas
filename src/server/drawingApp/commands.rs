/// Represents a single point on the canvas
struct Point {
    x: i32,
    y: i32,
}; // this could be stored simply as a tuple, with x, y order by convention.

/// Represents the any rectangle, can also be canvas
struct Dimensions {
    width: i32,
    height: i32,
};

/// Represents a rectangle drawing operation
struct DrawRectangleCommand {
    position: Point, /// upper-left corner start coordinates
    dimensions: Dimensions,
    fill: char, /// character used to fill rectangle
    outline: char /// character used to outline rectangle
}

// Repreents a flood fill drawing operation
struct FloodFillCommand {
    position: Point, /// start coordinates.
    fill: char, /// character used to fill empty spaces on canvas
}

