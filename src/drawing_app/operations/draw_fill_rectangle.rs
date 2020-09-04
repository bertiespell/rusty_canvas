pub fn execute(
    previous_state_canvas: &super::super::canvas::Canvas, 
    command: &super::super::commands::DrawCommand
) -> super::super::canvas::Canvas {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use drawing_app::{commands, canvas};

    #[test]
    fn test_simple_draw_fill() {
        // simple rectangle draw operation
        let canvas = canvas::Canvas::new(10, 8);
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n    XXX   \n    XXX   \n    XXX   \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
        assert_eq!(actual.pixels[3][3].occupied, false);
        assert_eq!(actual.pixels[3][3].character, ' ');
        assert_eq!(actual.pixels[3][4].occupied, true);
        assert_eq!(actual.pixels[3][4].character, 'X');
    }

    fn test_width_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::new(10, 8);
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
        let expected = "          \n          \n          \n   XXXXXX\n   XXXXXX\n   XXXXXX\n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    fn test_height_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::new(10, 8);
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
        let expected = "          \n          \n          \n    XXX   \n    XXX   \n    XXX   \n    XXX   \n    XXX   \n";

        assert_eq!(expected, &actual.to_string());
    }

    fn test_height_and_width_out_of_bounds() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::new(10, 8);
        let command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 4, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 6
            }),
            character: '!',
        };
        let actual = execute(&canvas, &command);
        let expected = "          \n          \n          \n   XXXXXX\n   XXXXXX\n   XXXXXX\n   XXXXXX\n   XXXXXX\n";

        assert_eq!(expected, &actual.to_string());
    }

    fn test_negative_height() {
        // a rectangle with negative height has no effect (Nb: one possiblity is to draw rectangle in the opposite direction, but since this feature is not specificed I will leave for now)
        let canvas = canvas::Canvas::new(10, 8);
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

    fn test_negative_width() {
        // a rectangle with negative width has no effect
        let canvas = canvas::Canvas::new(10, 8);
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

    fn test_0_height() {
        // a rectangle with 0 height has no effect
        let canvas = canvas::Canvas::new(10, 8);
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

    fn test_0_width() {
        // a rectangle with 0 width has no effect
        let canvas = canvas::Canvas::new(10, 8);
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

    fn test_negative_x_start_position() {
        // a command with negative x start position draws the first valid place on the cavnas
        let canvas = canvas::Canvas::new(10, 8);
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
        let expected = "          \n          \n          \n-         \n-         \n-         \n-         \n-         \n";

        assert_eq!(expected, &actual.to_string());
    }

    fn test_negative_y_start_position() {
        // a command with negative y start position draws the first valid place on the cavnas
        let canvas = canvas::Canvas::new(10, 8);
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
        let expected = "   *****  \n          \n          \n          \n          \n          \n          \n          \n";

        assert_eq!(expected, &actual.to_string());
    }

    fn test_dimensions_empty() {
        // for now, we'll do nothing if no dimensions is present (no dimensions == no rectangle)
        // or we could throw...
        // or we could beef out our return type, with an "error" or "notice" field, using a monad, so that we could chain together operations even if one of them is potentially erroneous
        // "Notice: Fill Command posted with no dimensions, did you mean to send flood filL?"
        let canvas = canvas::Canvas::new(10, 8);
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

    fn test_repeated_operation_idempotence() {
        // draw operation should draw up to the end of the canvas and handle error
        let canvas = canvas::Canvas::new(10, 8);
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

        let expected = "          \n          \n          \n   XXXXXX\n   XXXXXX\n   XXXXXX\n   XXXXXX\n   XXXXXX\n";

        assert_eq!(expected, &actual.to_string());
    }
}
