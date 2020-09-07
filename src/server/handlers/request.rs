use serde::{Deserialize, Serialize};
use super::super::super::drawing_app::{canvas};

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Request data structure expected on the draw rectangle route
pub struct DrawRectangleOperation {
    pub position: canvas::Point,
    pub dimensions: canvas::Dimensions,
    pub fill_character: String,
    pub outline_character: String,
}

pub const NONE_CHARACTER: &'static str = "none";

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Request data structure expected on the floor fill route
pub struct FloodFillOperation {
    pub position: canvas::Point,
    pub fill_character: String,
}
