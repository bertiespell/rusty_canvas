use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application};
use super::utils;

pub async fn ws_handler(
    _ws: warp::ws::Ws, 
    app: Arc<RwLock<application::DrawingApplication>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let canvas = app.write().draw(vec!());
    let html_string = utils::construct_html_with_canvas(&canvas.unwrap());

    Ok(warp::reply::with_status(
        html_string,
        http::StatusCode::from_u16(101).unwrap(),
    ))
}