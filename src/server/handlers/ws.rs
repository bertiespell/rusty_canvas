use parking_lot::RwLock;
use std::sync::Arc;
use warp::http::StatusCode;

use super::super::super::drawing_app::{application};
use super::errors;
use super::utils;

/// Skeleton for web socker handler
/// TODO: finish implementation
pub async fn ws_handler(
    _ws: warp::ws::Ws, 
    app: Arc<RwLock<application::DrawingApplication>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let canvas = app.write().draw(vec!());
    match canvas {
        Ok(canvas) => {
            let html_string = utils::construct_html_with_canvas(&canvas);
            Ok(warp::reply::with_status(
                html_string,
                StatusCode::SWITCHING_PROTOCOLS,
            ))
        },
        Err(_) => Err(warp::reject::custom(errors::ApplyOperationError))
    }
}