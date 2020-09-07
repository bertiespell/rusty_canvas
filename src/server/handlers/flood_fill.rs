use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;

pub async fn handle_flood_fill_operation(
    command: utils::FloodFillOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let flood_fill_command = commands::DrawCommand {
        name: commands::CommandName::FloodFill,
        position: command.position.clone(),
        dimensions: None,
        character: command.fill_character,
    };
    
    utils::apply_draw_operation(vec!(flood_fill_command), app)
}