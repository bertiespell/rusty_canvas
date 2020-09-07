use super::super::canvas;
use super::super::commands;
use super::utils;

/// Executes a FillRectangle command and returns a new canvas with the changes
pub fn execute(
    previous_state_canvas: &canvas::Canvas, 
    command: &commands::DrawCommand
) -> canvas::Canvas {
    match command.clone().dimensions {
        Some(dimensions) => {
            if utils::rectangle_size_is_none_zero(&dimensions) {
                return fill_rectangle(previous_state_canvas, &dimensions, command);
            }
            previous_state_canvas.clone()
        },
        None => {
            previous_state_canvas.clone()
        }
    }
}

/// Searches a canvas for the pixels to be updated, returns a new canvas with the changes
fn fill_rectangle(
    previous_state_canvas: &canvas::Canvas, 
    dimensions: &canvas::Dimensions,
    command: &commands::DrawCommand
) -> canvas::Canvas {
    let mut new_canvas = previous_state_canvas.clone();

    previous_state_canvas.pixels
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            row
                .iter()
                .enumerate()
                .for_each(|(column_index, pixel)| {
                    if pixel_should_change(
                        &dimensions,
                        &command.position, 
                        row_index as i32, 
                        column_index as i32
                    ) {
                        new_canvas.pixels[row_index][column_index] = command.character;
                    } else {
                        new_canvas.pixels[row_index][column_index] = pixel.clone()
                    }
                })
        });

        new_canvas
}

/// Given a rectangle with specific dimensions and start point:
/// Deterime whether a pixel at [row_index, column_index] should be updated
fn pixel_should_change(
    dimensions: &canvas::Dimensions,
    start_point: &canvas::Point, 
    row_index: i32,
    column_index: i32
) -> bool {
    utils::index_is_in_bound(column_index, start_point.x, dimensions.width ) && utils::index_is_in_bound(row_index, start_point.y, dimensions.height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::{commands, canvas};

    #[test]
    fn test_simple_draw_fill() {
        // simple rectangle draw operation
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 4
            }),
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    XXX   \n    XXX   \n    XXX   \n    XXX   \n          \n";

        assert_eq!(expected, &actual.to_string());
        assert_eq!(actual.pixels[3][3], ' ');
        assert_eq!(actual.pixels[3][4], 'X');
    }

    #[test]
    fn test_width_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 3
            }),
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    XXXXXX\n    XXXXXX\n    XXXXXX\n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_height_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 6
            }),
            character: '!',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    !!!   \n    !!!   \n    !!!   \n    !!!   \n    !!!   \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_height_and_width_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 6
            }),
            character: '4',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    444444\n    444444\n    444444\n    444444\n    444444\n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_height() {
        // a rectangle with negative height has no effect (Nb: one possiblity is to draw rectangle in the opposite direction, but since this feature is not specificed I will leave for now)
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: -5
            }),
            character: '!',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_width() {
        // a rectangle with negative width has no effect
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: -3,
                height: 5
            }),
            character: '-',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_0_height() {
        // a rectangle with 0 height has no effect
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 0
            }),
            character: '!',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_0_width() {
        // a rectangle with 0 width has no effect
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 0,
                height: 5
            }),
            character: '-',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_x_start_position() {
        // a command with negative x start position draws the first valid place on the canas
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: -3, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 5
            }),
            character: '-',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n--        \n--        \n--        \n--        \n--        \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_y_start_position() {
        // a command with negative y start position draws the first valid place on the canvas
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 3, y: -3},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 5
            }),
            character: '*',
        };
        let actual = execute(&canvas, &command);
        let expected = "   *****  \n   *****  \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_dimensions_empty() {
        // for now, we'll do nothing if no dimensions is present (no dimensions == no rectangle)
        // or we could throw...
        // or we could beef out our return type, with an "error" or "notice" field, using a monad, so that we could chain together operations even if one of them is potentially erroneous
        // "Notice: Fill Command posted with no dimensions, did you mean to send flood filL?"
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: None,
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_repeated_operation_idempotence() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 6
            }),
            character: '!',
        };

        let first_canvas = execute(&canvas, &command);
        let actual = execute(&first_canvas, &command);

        let expected = "          \n          \n          \n    !!!!!!\n    !!!!!!\n    !!!!!!\n    !!!!!!\n    !!!!!!\n";

        assert_eq!(expected, &actual.to_string());
    }
}
