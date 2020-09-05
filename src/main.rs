mod drawing_app;

use drawing_app::{application, canvas, commands};

// TODO: configure via env variables or command line arguments
const CANVAS: &'static str = "canvas_data.txt";
const TEMP_CANVAS: &'static str = "temp_canvas_data.txt";
const CANVAS_WIDTH: i32 = 10;
const CANVAS_HEIGHT: i32 = 10;
const BLANK_CHARACTER: char = '⬛';

fn main() {
    let app = application::DrawingApplication::new(
        application::ApplicationOptions {
            width: CANVAS_WIDTH,
            height: CANVAS_HEIGHT,
            blank_character: BLANK_CHARACTER,
            canvas_path: String::from(CANVAS),
            canvas_temp_path: String::from(TEMP_CANVAS),
        }
    );

    // test palette: 🟥🟧🟨🟩🟦🟪🟫⬛⬜
    let command: commands::DrawCommand = commands::DrawCommand {
        name: commands::CommandName::FillRectangle,
        position: canvas::Point{ x: 2, y: 2 },
        dimensions: Some(canvas::Dimensions {
            width: 4,
            height: 3
        }),
        character: '🟪',
    };

    let canvas = app.draw(vec!(command));

    match canvas {
        Ok(canvas) => {
            let painted_canvas = canvas.to_string();
            println!("{}", painted_canvas);
        },
        Err(err) => {
            println!("Error running canvas application: {}", err);
        }
    }
}
