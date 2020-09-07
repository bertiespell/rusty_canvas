use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;
use super::request;

/// Handler for the draw rectangle route
/// Takes a valid request and transforms this into a draw operation
/// It then attempts to draw to canvas and returns the result
pub async fn handle_draw_rectangle_operation(
    request: request::DrawRectangleOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let fill_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::FillRectangle,
        position: request.position.clone(),
        dimensions: Some(request.dimensions.clone()),
        character: request.fill_character,
    };

    let outline_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::OutlineRectangle,
        position: request.position,
        dimensions: Some(request.dimensions),
        character: request.outline_character,
    };

    utils::apply_draw_operation(vec!(fill_rectangle_command, outline_rectangle_command), app)
}