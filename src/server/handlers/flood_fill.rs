use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;
use super::request;

/// Handler for the flood fill route
/// Takes a valid request and transforms this into a draw operation
/// It then attempts to draw to canvas and returns the result
pub async fn handle_flood_fill_operation(
    request: request::FloodFillOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let flood_fill_command = commands::DrawCommand {
        name: commands::CommandName::FloodFill,
        position: request.position.clone(),
        dimensions: None,
        character: request.fill_character,
    };
    
    utils::apply_draw_operation(vec!(flood_fill_command), app)
}