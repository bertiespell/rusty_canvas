mod server;
mod drawing_app;
mod config;

use std::process;
use std::env;
use std::sync::Arc;
use parking_lot::RwLock;

use config::Config;
use drawing_app::{application};
use server::{server as serverApp};

#[tokio::main]
async fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let app = Arc::new(RwLock::new(application::DrawingApplication::initialize(
        application::ApplicationOptions {
            width: config.width,
            height: config.height,
            blank_character: config.blank_character,
            canvas_path: config.canvas_location,
            canvas_temp_path: config.temp_canvas_location,
        }
    )));

    serverApp::run(app).await
}
