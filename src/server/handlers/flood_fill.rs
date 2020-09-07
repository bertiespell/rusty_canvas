use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;
use super::request;

/// Handler for the flood fill route
/// Takes a valid request and transforms this into a draw operation
/// It then attempts to draw to canvas and returns the result
pub async fn handle_flood_fill_request(
    request: request::FloodFillOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {

    let request_character = request.fill_character;
    let blank_character = app.write().get_config().blank_character;

    match utils::parse_character(request_character, blank_character) {
        Ok(fill_character) => {
            let flood_fill_command = commands::DrawCommand {
                name: commands::CommandName::FloodFill,
                position: request.position.clone(),
                dimensions: None,
                character: fill_character,
            };
            
            utils::apply_draw_operation(vec!(flood_fill_command), app)
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use std::path::Path;
    use parking_lot::RwLock;
    use std::sync::Arc;
    use uuid::Uuid;
    use super::request;
    use super::super::super::super::drawing_app::{application, canvas};

    #[tokio::test]
    async fn test_flood_fill_request() {
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

        let request = request::FloodFillOperation {
            position: canvas::Point {
                x: 14,
                y: 0
            },
            fill_character: String::from("Y"),
        };

        let expected = "YYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\nYYYYYYYYYYYYYYYYYYYYY\n";

        handle_flood_fill_request(request, app.clone()).await.unwrap();

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
}