use serde::{Deserialize, Serialize};
use super::super::super::drawing_app::{canvas};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DrawRectangleOperation {
    pub position: canvas::Point,
    pub dimensions: canvas::Dimensions,
    pub fill_character: char,
    pub outline_character: char,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FloodFillOperation {
    pub position: canvas::Point,
    pub fill_character: char,
}
