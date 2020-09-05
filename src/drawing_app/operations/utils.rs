use super::super::canvas;

pub fn get_canvas_pixel(
    canvas: &canvas::Canvas,
    position: &canvas::Point, 
) -> char {
    canvas.pixels[position.y as usize][position.x as usize]
}

pub fn position_is_on_canvas(
    canvas: &canvas::Canvas, 
    start_point: &canvas::Point, 
) -> bool {
    start_point.x >= 0 && 
    start_point.y >= 0 && 
    start_point.x < canvas.dimensions.width &&
    start_point.y < canvas.dimensions.height
}

/// Determine whether an index occurs within a specific range
pub fn index_is_in_bound(
    index: i32,
    start: i32, 
    length: i32,
) -> bool {
    index >= start && index < length  + start
}

/// Determine whether the dimensionality is none 0
pub fn rectangle_size_is_none_zero(dimensions: &canvas::Dimensions) -> bool {
    dimensions.width > 0 && dimensions.height > 0
}

fn is_row_edge(
    dimensions: &canvas::Dimensions,
    start_point: &canvas::Point,
    row_index: i32,
    column_index: i32,
) -> bool {
    is_on_line(dimensions.width , dimensions.height, start_point.y, start_point.x, row_index, column_index)
}

fn is_column_edge(
    dimensions: &canvas::Dimensions,
    start_point: &canvas::Point,
    row_index: i32,
    column_index: i32,
) -> bool {
    is_on_line(dimensions.height , dimensions.width, start_point.x, start_point.y, column_index, row_index)
}

/// Given two parallel lines of a set `distance` apart and defined by a starting position [x, y]
/// Determine whether a point [x, y] occurs on lines of a given `length`.
fn is_on_line(
    distance: i32,
    length: i32, 
    start_position_x: i32,
    start_position_y: i32,
    point_x: i32,
    point_y: i32,
) -> bool {
    (point_x == start_position_x || point_x == start_position_x + length - 1) &&
    (point_y >= start_position_y && point_y <= start_position_y + distance - 1)
}


/// Given a rectangle with specific dimensions and start point:
/// Deterime whether a pixel at [row_index, column_index] should be updated
pub fn is_edge(
    dimensions: &canvas::Dimensions,
    start_point: &canvas::Point,
    row_index: i32,
    column_index: i32,
) -> bool {
    is_row_edge(dimensions, start_point, row_index, column_index) || is_column_edge(dimensions, start_point, row_index, column_index)
}
