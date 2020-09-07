use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;
use super::request;

/// Handler for the draw rectangle route
/// Takes a valid request and transforms this into a draw operation
/// It then attempts to draw to canvas and returns the result
pub async fn handle_draw_rectangle_request(
    request: request::DrawRectangleOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request_fill_character = request.fill_character;
    let request_outline_character = request.outline_character;
    let blank_character = app.write().get_config().blank_character;

    let parsed_fill_character;
    match utils::parse_character(request_fill_character, blank_character) {
        Ok(fill_character) => {
            parsed_fill_character = fill_character;
        },
        Err(e) => {
            return Err(e);
        },
    }

    let parsed_outline_character;
    match utils::parse_character(request_outline_character, blank_character) {
        Ok(fill_character) => {
            parsed_outline_character = fill_character;
        },
        Err(e) => {
            return Err(e);
        },
    }

    let fill_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::FillRectangle,
        position: request.position.clone(),
        dimensions: Some(request.dimensions.clone()),
        character: parsed_fill_character,
    };

    let outline_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::OutlineRectangle,
        position: request.position,
        dimensions: Some(request.dimensions),
        character: parsed_outline_character,
    };

    utils::apply_draw_operation(vec!(fill_rectangle_command, outline_rectangle_command), app)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use parking_lot::RwLock;
    use std::sync::Arc;
    use uuid::Uuid;
    
    use super::*;
    use super::request;
    use super::super::super::super::drawing_app::{application, canvas};

    #[tokio::test]
    async fn test_handle_draw_rectangle_request() {
        let canvas_location = Uuid::new_v4().to_string();
        let temp_canvas_location = Uuid::new_v4().to_string();

        let app = Arc::new(RwLock::new(application::DrawingApplication::initialize(
            application::ApplicationOptions {
                width: 21,
                height: 8,
                blank_character: ' ',
                canvas_path: canvas_location.clone(),
                canvas_temp_path: temp_canvas_location.clone(),
            }
        )));
        
        // ensure our test files are empty
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }

        let request_one = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 14,
                y: 0
            },
            dimensions: canvas::Dimensions {
                width: 7,
                height: 6,
            },
            fill_character: String::from("."),
            outline_character: String::from("."),
        };

        let request_two = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 0,
                y: 3
            },
            dimensions: canvas::Dimensions {
                width: 8,
                height: 4,
            },
            fill_character: String::from(" "),
            outline_character: String::from("O"),
        };

        let request_three = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 5,
                y: 5
            },
            dimensions: canvas::Dimensions {
                width: 5,
                height: 3,
            },
            fill_character: String::from("X"),
            outline_character: String::from("X"),
        };

        let expected = "              .......\n              .......\n              .......\nOOOOOOOO      .......\nO      O      .......\nO    XXXXX    .......\nOOOOOXXXXX           \n     XXXXX           \n";

        handle_draw_rectangle_request(request_one, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_two, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_three, app.clone()).await.unwrap();

        let actual = app
            .write()
            .draw(vec!());
        
        assert_eq!(expected, actual.unwrap().to_string());

        // clean up
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }
    }

    #[tokio::test]
    async fn test_none_keyword() {
        let canvas_location = Uuid::new_v4().to_string();
        let temp_canvas_location = Uuid::new_v4().to_string();

        let app = Arc::new(RwLock::new(application::DrawingApplication::initialize(
            application::ApplicationOptions {
                width: 21,
                height: 8,
                blank_character: ' ',
                canvas_path: canvas_location.clone(),
                canvas_temp_path: temp_canvas_location.clone(),
            }
        )));
        
        // ensure our test files are empty
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }

        let request_one = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 14,
                y: 0
            },
            dimensions: canvas::Dimensions {
                width: 7,
                height: 6,
            },
            fill_character: String::from('.'),
            outline_character: String::from('.'),
        };

        let request_two = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 0,
                y: 3
            },
            dimensions: canvas::Dimensions {
                width: 8,
                height: 4,
            },
            fill_character: String::from("none"),
            outline_character: String::from('O'),
        };

        let request_three = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 5,
                y: 5
            },
            dimensions: canvas::Dimensions {
                width: 5,
                height: 3,
            },
            fill_character: String::from('X'),
            outline_character: String::from('X')
        };

        let expected = "              .......\n              .......\n              .......\nOOOOOOOO      .......\nO      O      .......\nO    XXXXX    .......\nOOOOOXXXXX           \n     XXXXX           \n";

        handle_draw_rectangle_request(request_one, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_two, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_three, app.clone()).await.unwrap();

        let actual = app
            .write()
            .draw(vec!());
        
        assert_eq!(expected, actual.unwrap().to_string());

        // clean up
        // ensure our test files are empty
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }
    }

    #[tokio::test]
    async fn test_none_written_in_any_case_keyword() {
        let canvas_location = Uuid::new_v4().to_string();
        let temp_canvas_location = Uuid::new_v4().to_string();

        let app = Arc::new(RwLock::new(application::DrawingApplication::initialize(
            application::ApplicationOptions {
                width: 21,
                height: 8,
                blank_character: ' ',
                canvas_path: canvas_location.clone(),
                canvas_temp_path: temp_canvas_location.clone(),
            }
        )));
        
        // ensure our test files are empty
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }

        let request_one = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 14,
                y: 0
            },
            dimensions: canvas::Dimensions {
                width: 7,
                height: 6,
            },
            fill_character: String::from('.'),
            outline_character: String::from('.'),
        };

        let request_two = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 0,
                y: 3
            },
            dimensions: canvas::Dimensions {
                width: 8,
                height: 4,
            },
            fill_character: String::from("NoNe"),
            outline_character: String::from('O'),
        };

        let request_three = request::DrawRectangleOperation {
            position: canvas::Point {
                x: 5,
                y: 5
            },
            dimensions: canvas::Dimensions {
                width: 5,
                height: 3,
            },
            fill_character: String::from('X'),
            outline_character: String::from('X')
        };

        let expected = "              .......\n              .......\n              .......\nOOOOOOOO      .......\nO      O      .......\nO    XXXXX    .......\nOOOOOXXXXX           \n     XXXXX           \n";

        handle_draw_rectangle_request(request_one, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_two, app.clone()).await.unwrap();
        handle_draw_rectangle_request(request_three, app.clone()).await.unwrap();

        let actual = app
            .write()
            .draw(vec!());
        
        assert_eq!(expected, actual.unwrap().to_string());

        // clean up
        if Path::new(&canvas_location).exists() {
            fs::remove_file(&canvas_location).unwrap();
        }
        if Path::new(&temp_canvas_location).exists() {
            fs::remove_file(&temp_canvas_location).unwrap();
        }
    }
}