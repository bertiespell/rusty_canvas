mod server;
mod drawing_app;

use parking_lot::RwLock;
use std::sync::Arc;
use drawing_app::{application};
use server::{server as serverApp};

// TODO: configure via env variables or command line arguments
const CANVAS: &'static str = "canvas_data.txt";
const TEMP_CANVAS: &'static str = "temp_canvas_data.txt";
const CANVAS_WIDTH: i32 = 10;
const CANVAS_HEIGHT: i32 = 10;
const BLANK_CHARACTER: char = '⬛';

#[tokio::main]
async fn main() {
    let app = Arc::new(RwLock::new(application::DrawingApplication::initialize(
        application::ApplicationOptions {
            width: CANVAS_WIDTH,
            height: CANVAS_HEIGHT,
            blank_character: BLANK_CHARACTER,
            canvas_path: String::from(CANVAS),
            canvas_temp_path: String::from(TEMP_CANVAS),
        }
    )));

    serverApp::run(app).await
}
