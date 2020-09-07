use super::super::canvas;
use super::super::commands;
use super::utils;

/// Executes an OutlineRectangle command and returns a new canvas with the changes
pub fn execute(
    previous_state_canvas: &canvas::Canvas,
    command: &commands::DrawCommand,
) -> canvas::Canvas {
    match command.clone().dimensions {
        Some(dimensions) => {
            if utils::rectangle_size_is_none_zero(&dimensions) {
                return draw_outline(previous_state_canvas, &dimensions, command);
            }
            previous_state_canvas.clone()
        },
        None => {
            previous_state_canvas.clone()
        }
    }
}

/// Searches a canvas for the pixels to be updated, returns a new canvas with the changes
fn draw_outline(
    previous_state_canvas: &canvas::Canvas,
    dimensions: &canvas::Dimensions,
    command: &commands::DrawCommand,
) -> canvas::Canvas {
    let mut new_canvas = previous_state_canvas.clone();

    previous_state_canvas.pixels
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            row
                .iter()
                .enumerate()
                .for_each(|(column_index, _)| {
                    if utils::is_edge(
                        dimensions,
                        &command.position, 
                        row_index as i32, 
                        column_index as i32
                    ) {
                        new_canvas.pixels[row_index][column_index] = command.character;
                    }
                })
        });

        new_canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::{commands, canvas};

    #[test]
    fn test_simple_outline() {
        // simple ouline rectangle draw operation
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 4
            }),
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    XXX   \n    X X   \n    X X   \n    XXX   \n          \n";

        assert_eq!(expected, &actual.to_string());
        assert_eq!(actual.pixels[3][3], ' ');
        assert_eq!(actual.pixels[3][4], 'X');
        // second row of shape should be empty in the middle and only include edges
        assert_eq!(actual.pixels[4][4], 'X');
        // the middle is left empty
        assert_eq!(actual.pixels[4][5], ' ');
    }

    #[test]
    fn test_width_out_of_bounds() {
        // we don't draw an edge in this case where it runs out of bounds, but we could
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 3
            }),
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    XXXXXX\n    X     \n    XXXXXX\n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_height_out_of_bounds() {
        // we don't draw an edge in this case, but we could
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 6
            }),
            character: '!',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    !!!   \n    ! !   \n    ! !   \n    ! !   \n    ! !   \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_height_and_width_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 6
            }),
            character: '4',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    444444\n    4     \n    4     \n    4     \n    4     \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_height() {
        // a rectangle with negative height has no effect (Nb: one possiblity is to draw rectangle in the opposite direction, but since this feature is not specificed I will leave for now)
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
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
            name: commands::CommandName::OutlineRectangle,
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
            name: commands::CommandName::OutlineRectangle,
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
            name: commands::CommandName::OutlineRectangle,
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
        // a command with negative x start position draws if the edge is a valid position on canvas
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: -3, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 5
            }),
            character: '-',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n--        \n -        \n -        \n -        \n--        \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_negative_y_start_position() {
        // a command with negative y start position draws if the edge is a valid position on canvas
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 3, y: -3},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 5
            }),
            character: '*',
        };
        let actual = execute(&canvas, &command);
        let expected = "   *   *  \n   *****  \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_dimensions_empty() {
        // take no action if no dimension is specified (see similar note in FillRectangle)
        let canvas = canvas::Canvas::blank_canvas(10, 8, ' ');
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
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
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 6
            }),
            character: '!',
        };

        let first_canvas = execute(&canvas, &command);
        let actual = execute(&first_canvas, &command);

        let expected = "          \n          \n          \n    !!!!!!\n    !     \n    !     \n    !     \n    !     \n";

        assert_eq!(expected, &actual.to_string());
    }
}
