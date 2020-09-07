use parking_lot::RwLock;
use std::sync::Arc;

use super::super::super::drawing_app::{application, commands};
use super::utils;

pub async fn handle_draw_rectangle_operation(
    command: utils::DrawRectangleOperation,
    app: Arc<RwLock<application::DrawingApplication>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let fill_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::FillRectangle,
        position: command.position.clone(),
        dimensions: Some(command.dimensions.clone()),
        character: command.fill_character,
    };

    let outline_rectangle_command = commands::DrawCommand {
        name: commands::CommandName::OutlineRectangle,
        position: command.position,
        dimensions: Some(command.dimensions),
        character: command.outline_character,
    };

    utils::apply_draw_operation(vec!(fill_rectangle_command, outline_rectangle_command), app)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn handle_draw_operation() {
        //TODO: test that the handlers work as expected. 
        // let mut response;
        // let res = warp::test::request()
        //     .method("POST")
        //     .path("fillrectangle")
        //     .reply() 
        //     .await;

        // assert_eq!(res.status(), 200, "Should return 200 OK.");

        // println!("{:#?}", res.body());
    }
}