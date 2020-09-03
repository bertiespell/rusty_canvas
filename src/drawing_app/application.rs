use std::fs::File;
use std::io;
use std::io::prelude::*;
use super::commands::CommandName;

/// temporary location for global canvas
const CANVAS: &'static str = "canvas_data.txt";

/// Given a previous canvas and a draw command, return a new updated canvas state
pub fn apply_draw_commands(
    previous_state_canvas: &super::canvas::Canvas, 
    commands:Vec<super::commands::DrawCommand>
) -> super::canvas::Canvas {
    commands
        .iter()
        .fold(previous_state_canvas.clone(), |previous_canvas, command| {
            match command.name {
                CommandName::FillRectangle => draw_fill_rectangle(&previous_canvas, command),
                CommandName::OutlineRectangle => draw_outline_rectangle(&previous_canvas, command),
                CommandName::FloodFill => draw_flood_fill_rectangle(&previous_canvas, command),
            }
        })
}

pub fn draw_fill_rectangle(
    previous_state_canvas: &super::canvas::Canvas, 
    command: &super::commands::DrawCommand
) -> super::canvas::Canvas {
    unimplemented!();
}

pub fn draw_outline_rectangle(
    previous_state_canvas: &super::canvas::Canvas, 
    command: &super::commands::DrawCommand
) -> super::canvas::Canvas {
    unimplemented!();
}

pub fn draw_flood_fill_rectangle(
    previous_state_canvas: &super::canvas::Canvas, 
    command: &super::commands::DrawCommand
) -> super::canvas::Canvas {
    unimplemented!();
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

        let first_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point{x: 3, y: 2},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 3
            }),
            character: 'X',
        };

        let second_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point{x: 3, y: 2},
            dimensions: Some(canvas::Dimensions {
                width: 5,
                height: 3
            }),
            character: '@'
        };

        let third_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 10, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 14,
                height: 6
            }),
            character: 'O',
        };

        let fourth_command: commands::DrawCommand = commands::DrawCommand {
            name: commands::CommandName::FillRectangle,
            position: canvas::Point {x: 10, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 14,
                height: 6
            }),
            character: 'X'
        };

        let canvas = apply_draw_commands(&canvas, vec!(first_command, second_command, third_command, fourth_command));

        let actual = canvas.to_string();

        let expected =
        "                    
                             
                             
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