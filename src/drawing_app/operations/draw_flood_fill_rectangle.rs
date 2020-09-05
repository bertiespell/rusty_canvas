use super::super::canvas;
use super::super::commands;

/// Executes a FloodFill command and returns a new canvas with the changes
pub fn execute(
    previous_state_canvas: &super::super::canvas::Canvas,
    command: &super::super::commands::DrawCommand
) -> super::super::canvas::Canvas {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{draw_fill_rectangle, draw_outline_rectangle};
    use drawing_app::{commands, canvas};

    #[test]
    fn test_simple_flood_fill_operation() {
        // simple rectangle draw operation - should fill all space betewen two squares with space between them
        let canvas = canvas::Canvas::new(8, 9);
        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:0, y: 5},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:4, y: 0},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };

        let third_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FloodFill,
            position: canvas::Point {x:1, y: 0},
            dimensions: None,
            character: '.',
        };

        let canvas1 = draw_fill_rectangle::execute(&canvas, &first_command);
        let canvas2 = draw_fill_rectangle::execute(&canvas1, &second_command);
        let actual = execute(&canvas2, &third_command);

        let expected = "....XXX.\n....XXX.\n....XXX.\n........\n........\nXXX.....\nXXX.....\nXXX.....\n........\n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_fill_already_existing_rectangle() {
        // should replace the characters in a rectangle with new fill operator
        let canvas = canvas::Canvas::new(8, 9);
        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:0, y: 5},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FloodFill,
            position: canvas::Point {x:0, y: 5},
            dimensions: None,
            character: '.',
        };

        let canvas1 = draw_fill_rectangle::execute(&canvas, &first_command);
        let actual = execute(&canvas1, &second_command);

        let expected = "        \n        \n        \n        \n        \n...     \n...     \n...     \n        \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_rectangles_touching_at_corners_shouldnt_fill() {
        // when two corners are touching at the edge, the other rectangle isn't filled
        let canvas = canvas::Canvas::new(8, 9);
        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:0, y: 5},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:3, y: 8},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 1
            }),
            character: 'X',
        };

        let third_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:0, y: 5},
            dimensions: None,
            character: '.',
        };

        let canvas1 = draw_fill_rectangle::execute(&canvas, &first_command);
        let canvas2 = draw_fill_rectangle::execute(&canvas1, &second_command);
        let actual = execute(&canvas2, &third_command);
        let expected = "        \n        \n        \n        \n        \n...     \n...     \n...     \n   XXX  \n";

        assert_eq!(expected, &actual.to_string());
    }

    #[test]
    fn test_complex_flood_fill_commands() {
        let canvas: canvas::Canvas = canvas::Canvas::new(21, 8);

        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point{x: 14, y: 0},
            dimensions: Some(canvas::Dimensions {
                width: 7,
                height: 6
            }),
            character: '.',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 0, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 4
            }),
            character: ' ',
        };

        let third_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 0, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 8,
                height: 4
            }),
            character: 'O',
        };

        let fourth_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 5, y: 5},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 3
            }),
            character: 'X'
        };

        let fifth_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FloodFill,
            position: canvas::Point {x: 0, y: 0},
            dimensions: None,
            character: '-'
        };

        let canvas1 = draw_fill_rectangle::execute(&canvas, &first_command);
        let canvas2 = draw_fill_rectangle::execute(&canvas1, &second_command);
        let canvas3 = draw_outline_rectangle::execute(&canvas2, &third_command);
        let canvas4 = draw_fill_rectangle::execute(&canvas3, &fourth_command);
        
        let actual = execute(&canvas4, &fifth_command);

        let expected = "--------------.......\n--------------.......\n--------------.......\nOOOOOOOO------.......\nO      O------.......\nO    XXXXX----.......\nOOOOOXXXXX-----------\n     XXXXX-----------\n";

        assert_eq!(expected, actual.to_string());
    }

    #[test]
    fn test_out_of_bounds() {
        // do nothing when either x or y is out of bounds
        let canvas = canvas::Canvas::new(8, 9);
        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x:0, y: 5},
            dimensions: Some(canvas::Dimensions {
                width: 3,
                height: 3
            }),
            character: 'X',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FloodFill,
            position: canvas::Point {x:0, y: -1},
            dimensions: None,
            character: '.',
        };

        let canvas1 = draw_fill_rectangle::execute(&canvas, &first_command);
        let actual = execute(&canvas1, &second_command);
        let expected = "        \n        \n        \n        \n        \nXXX     \nXXX     \nXXX     \n        \n";

        assert_eq!(expected, &actual.to_string());

        let third_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FloodFill,
            position: canvas::Point {x:-1, y: 0},
            dimensions: None,
            character: '.',
        };

        let actual2 = execute(&actual, &third_command);

        assert_eq!(expected, &actual2.to_string());
    }
}
