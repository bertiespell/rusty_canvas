use std::sync::Arc;
use warp::{Filter};
use parking_lot::RwLock;

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
                http::StatusCode::CREATED,
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

    format!("{}{}{}","<div class=\"canvas\"", canvas_element, "</div>")
}

/// Constructs a basic HTML document containing the canvas
/// Contains basic web socket initialisation scripts 
pub fn construct_html_with_canvas(canvas: &canvas::Canvas) -> String {
    let start_tags: String = String::from("
        <html>
            <head>
                <title>HTML with warp!</title>
            </head>
        <body>"
    );
    let canvas_element: String = convert_canvas_to_html(canvas);
    let end_tags = String::from("</body></html>");

    format!("{}{}{}", start_tags, canvas_element, end_tags)
}