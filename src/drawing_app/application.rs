use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use super::commands::CommandName;
use super::operations;

pub struct ApplicationOptions {
    pub width: i32,
    pub height: i32,
    pub blank_character: char,
    pub canvas_path: String,
    pub canvas_temp_path: String,
}

pub struct DrawingApplication {
    options: ApplicationOptions,
}

impl DrawingApplication {
    /// Entry-point to the application
    /// Sets initial application variabls
    pub fn new(options: ApplicationOptions) -> DrawingApplication {
        DrawingApplication {
            options,
        }
    }

    /// Applies draw commands to the canvas
    /// Saves the canvas to file, creating one if none exists
    pub fn draw(
        &self,
        commands:Vec<super::commands::DrawCommand>,
    ) -> io::Result<super::canvas::Canvas> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.options.canvas_path);
        
        match file {
            Ok(mut file) => {

                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                let canvas: super::canvas::Canvas;

                if contents.is_empty() {
                    canvas = super::canvas::Canvas::blank_canvas(
                        self.options.width, 
                        self.options.height,
                        self.options.blank_character,
                    );
                } else {
                    canvas = super::canvas::Canvas::from_chars(
                        contents
                            .lines()
                            .map(|line| {
                                line
                                    .chars()
                                    .collect()
                            })
                            .collect(),
                        self.options.width, 
                        self.options.height,
                    );
                }
            
                let updated_canvas = apply_draw_commands(&canvas, commands);
            
                let temp_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&self.options.canvas_temp_path);
                
                match temp_file {
                    Ok(mut temp_file) => {
                        temp_file.write(updated_canvas.to_string().as_bytes())?;

                        fs::remove_file(&self.options.canvas_path)?;
                        fs::rename(
                            &self.options.canvas_temp_path, 
                            &self.options.canvas_path
                        )?;
                        Ok(updated_canvas)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }
}
/// Given a previous canvas and a draw command, return a new updated canvas state
/// Folds over a set of commands, returning a new canvas each time
pub fn apply_draw_commands(
    previous_state_canvas: &super::canvas::Canvas, 
    commands:Vec<super::commands::DrawCommand>,
) -> super::canvas::Canvas {
    commands
        .iter()
        .fold(previous_state_canvas.clone(), |previous_canvas, command| {
            match command.name {
                CommandName::FillRectangle => operations::draw_fill_rectangle::execute(&previous_canvas, command),
                CommandName::OutlineRectangle => operations::draw_outline_rectangle::execute(&previous_canvas, command),
                CommandName::FloodFill => operations::draw_flood_fill_rectangle::execute(&previous_canvas, command),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use drawing_app::{commands, canvas};

    #[test]
    fn test_apply_draw_command() {
        let canvas: canvas::Canvas = canvas::Canvas::blank_canvas(24, 9, ' ');

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
            name: commands::CommandName::OutlineRectangle,
            position: canvas::Point {x: 10, y: 3},
            dimensions: Some(canvas::Dimensions {
                width: 14,
                height: 6
            }),
            character: 'X'
        };

        let canvas = apply_draw_commands(&canvas, vec!(first_command, second_command, third_command, fourth_command));

        let actual = canvas.to_string();

        let expected = "                        \n                        \n   @@@@@                \n   @XXX@  XXXXXXXXXXXXXX\n   @@@@@  XOOOOOOOOOOOOX\n          XOOOOOOOOOOOOX\n          XOOOOOOOOOOOOX\n          XOOOOOOOOOOOOX\n          XXXXXXXXXXXXXX\n";

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_commands() {
        let canvas: canvas::Canvas = canvas::Canvas::blank_canvas(21, 8, ' ');

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

        let canvas = apply_draw_commands(&canvas, vec!(first_command, second_command, third_command, fourth_command));

        let actual = canvas.to_string();

        let expected = "              .......\n              .......\n              .......\nOOOOOOOO      .......\nO      O      .......\nO    XXXXX    .......\nOOOOOXXXXX           \n     XXXXX           \n";

        assert_eq!(expected, actual);

    }

    #[test]
    fn test_multiple_with_flood_fill_commands() {
        let canvas: canvas::Canvas = canvas::Canvas::blank_canvas(21, 8, ' ');

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

        let canvas = apply_draw_commands(&canvas, vec!(first_command, second_command, third_command, fourth_command, fifth_command));

        let actual = canvas.to_string();

        let expected = "--------------.......\n--------------.......\n--------------.......\nOOOOOOOO------.......\nO      O------.......\nO    XXXXX----.......\nOOOOOXXXXX-----------\n     XXXXX-----------\n";

        assert_eq!(expected, actual);
    }
}