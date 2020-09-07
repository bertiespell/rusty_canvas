use std::sync::Arc;
use serde::{Deserialize, Serialize};
use warp::{Filter};
use parking_lot::RwLock;

use super::super::super::drawing_app::{application, canvas, commands};

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
        Err(_) => {
            Err(warp::reject())
        }
    }
}

pub fn construct_html_with_canvas(canvas: &canvas::Canvas) -> String {
    let mut start_tags: String = String::from("
        <html>
            <head>
                <title>HTML with warp!</title>
            </head>
        <body>"
    );
    let canvas_pixels: String = convert_canvas_to_html(canvas);
    let end_tags = String::from("
        <script type=\"text/javascript\">var uri = 'ws://' + location.host + '/canvas';
            var ws = new WebSocket(uri);
            console.log(ws);
            ws.onmessage = function(msg) {
                console.log(msg.data);
            };
        </script>
    </body></html>");
    start_tags.push_str(&canvas_pixels);
    start_tags.push_str(&end_tags);
    start_tags
}

pub fn convert_canvas_to_html(canvas: &canvas::Canvas) -> String { 
    canvas.pixels
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
        .fold(String::new(), |a, b| a + &b + "\n")
}

/// Check that the body is JSON and marshalls into correct format (and reject huge payloads)
pub fn draw_rectangle_json_body() -> impl Filter<Extract = (DrawRectangleOperation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Check that the body is JSON and marshalls into correct format (and reject huge payloads)
pub fn flood_fill_json_body() -> impl Filter<Extract = (FloodFillOperation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DrawRectangleOperation {
    pub position: canvas::Point,
    pub dimensions: canvas::Dimensions,
    pub fill_character: char,
    pub outline_character: char,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FloodFillOperation {
    pub position: canvas::Point,
    pub fill_character: char,
}
