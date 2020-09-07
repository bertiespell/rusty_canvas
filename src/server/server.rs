use parking_lot::RwLock;
use std::sync::Arc;
use warp::{Filter};

use super::super::drawing_app::{application};
use super::handlers;

pub async fn run(app: Arc<RwLock<application::DrawingApplication>>) {
    let app_filter = warp::any().map(move || app.clone());

    let get_canvas = warp::get()
        .and(warp::path::end())
        .and(app_filter.clone())
        .and_then(handlers::get_canvas::handle_get_canvas);

    let fill_rectangle = warp::post()
        .and(warp::path("drawrectangle"))
        .and(warp::path::end())
        .and(handlers::utils::parse_draw_rectangle_request())
        .and(app_filter.clone())
        .and_then(handlers::draw_rectangle::handle_draw_rectangle_operation);

    let flood_fill = warp::post()
        .and(warp::path("floodfill"))
        .and(warp::path::end())
        .and(handlers::utils::parse_flood_fill_request())
        .and(app_filter.clone())
        .and_then(handlers::flood_fill::handle_flood_fill_operation);

    let ws_route = warp::path("canvas")
        .and(warp::ws())
        .and(app_filter.clone())
        .and_then(handlers::ws::ws_handler);

    let routes = fill_rectangle
        .or(flood_fill)
        .or(get_canvas)
        .or(ws_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}