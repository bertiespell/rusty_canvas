use std::sync::Arc;
use warp::{Filter};
use parking_lot::RwLock;
use warp::http::StatusCode;

use super::super::super::drawing_app::{application, canvas, commands};
use super::errors;
use super::request;

/// Attempts to apply draw operation to the canvas
/// Returns a Result type
pub fn apply_draw_operation(
    command: Vec<commands::DrawCommand>,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let canvas = app
        .write()
        .draw(command);

    match canvas {
        Ok(canvas) => {
            Ok(warp::reply::with_status(
                canvas.to_string(),
                StatusCode::OK,
            ))
        },
        Err(_) => Err(warp::reject::custom(errors::ApplyOperationError))
    }
}

/// Check that the body is JSON and marshalls into correct draw rectangle request format
/// Rejects big payloads
pub fn parse_draw_rectangle_request() -> impl Filter<Extract = (request::DrawRectangleOperation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Check that the body is JSON and marshalls into correct flood fill request format
/// Rejects big payloads
pub fn parse_flood_fill_request() -> impl Filter<Extract = (request::FloodFillOperation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Parse the operation's outline or fill character
/// Checks whether this is specified as none
pub fn field_is_not_none(field: &str) -> bool {
    field.to_ascii_lowercase() != request::NONE_CHARACTER
}

/// Parse the operation's outline or fill character
/// Checks that it is a single valid unicode character
pub fn valid_character(field: &str) -> Result<char, warp::Rejection> {
    match field.chars().take(1).next() {
        Some(character) => {
            if field.chars().count() == 1 {
                return Ok(character);
            } 
            
            return Err(warp::reject::custom(super::errors::StringTooLong));
        },
        _ => Err(warp::reject::custom(super::errors::StringTooLong)),
    }
}

/// Constructs a valid HTML element out of the canvas
pub fn convert_canvas_to_html(canvas: &canvas::Canvas) -> String { 
    let canvas_element = canvas.pixels
        .iter()
        .map(|row| {
            let mut html_string = String::from("<div>");
            let pixels: String = String::from(row
                .iter()
                .collect::<String>());
            html_string.push_str(&pixels);
            html_string.push_str(&String::from("</div>"));
            html_string
        })
        .fold(String::new(), |a, b| a + &b + "\n");

    format!("{}{}{}","<div class=\"canvas\">", canvas_element, "</div>")
}

/// Constructs a basic HTML document containing the canvas
/// Contains basic web socket initialisation scripts 
pub fn construct_html_with_canvas(canvas: &canvas::Canvas) -> String {
    let start_tags: String = String::from("
        <!DOCTYPE html>
        <html lang=\"en\">
            <head>
                <title>Rusty Canvas!</title>
            </head>
        <body>"
    );
    let canvas_element: String = convert_canvas_to_html(canvas);
    let end_tags = String::from("</body></html>");

    format!("{}{}{}", start_tags, canvas_element, end_tags)
}