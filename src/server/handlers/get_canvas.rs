use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application};
use super::utils;
use super::errors;

pub async fn handle_get_canvas(
    app: Arc<RwLock<application::DrawingApplication>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let canvas = app.write().draw(vec!());
    match canvas {
        Ok(canvas) => {
            let html_string = utils::construct_html_with_canvas(&canvas);
            Ok(warp::reply::html(html_string))
        },
        Err(_) => Err(warp::reject::custom(errors::ApplyOperationError))
    }
}
