use std::fs::File;
use std::io;
use std::io::prelude::*;

/// temporary location for global canvas
const CANVAS: &'static str = "canvasData.txt";

/// Given a previous canvas and a draw command, return a new updated canvas state
pub fn apply_draw_command(previous_state_canvas: &super::canvas::Canvas, command:&super::commands::DrawRectangleCommand) -> super::canvas::Canvas {
    super::canvas::Canvas::new(24, 6)
}

pub fn update_canvas(ascii: &str) -> io::Result<()> {
    let mut file = File::create(CANVAS)?; // does this just read if not created?
    file.write_all(ascii.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use drawing_app::{commands, canvas};

    #[test]
    fn test_apply_draw_command() {
        let canvas: canvas::Canvas = canvas::Canvas::new(24, 9);

        let first_command: commands::DrawRectangleCommand = commands::DrawRectangleCommand {
            position: canvas::Point{x: 3, y: 2},
            dimensions: canvas::Dimensions {
                width: 5,
                height: 3
            },
            fill: 'X',
            outline: '@'
        };

        let second_command: commands::DrawRectangleCommand = commands::DrawRectangleCommand {
            position: canvas::Point {x: 10, y: 3},
            dimensions: canvas::Dimensions {
                width: 14,
                height: 6
            },
            fill: 'X',
            outline: 'O'
        };

        let first_canvas = apply_draw_command(&canvas, &first_command);
        let second_canvas = apply_draw_command(&first_canvas, &second_command);

        let actual = second_canvas.to_string();

        let expected = "


        @@@@@
        @XXX@  XXXXXXXXXXXXXX
        @@@@@  XOOOOOOOOOOOOX
               XOOOOOOOOOOOOX
               XOOOOOOOOOOOOX
               XOOOOOOOOOOOOX
               XXXXXXXXXXXXXX
        ";
        
        assert_eq!(expected, actual);
    }
}