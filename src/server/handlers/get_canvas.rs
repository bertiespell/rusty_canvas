use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application};
use super::utils;

pub async fn handle_get_canvas(
    app: Arc<RwLock<application::DrawingApplication>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let canvas = app.write().draw(vec!());
    let html_string = utils::construct_html_with_canvas(&canvas.unwrap());
    Ok(warp::reply::html(html_string))
}
