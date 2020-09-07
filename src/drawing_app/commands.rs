use serde::{Deserialize, Serialize};

/// Set of all possible operations
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CommandName {
    FillRectangle,
    OutlineRectangle,
    FloodFill,
}

/// Represents a drawing operation
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DrawCommand {
    pub name: CommandName, // name identifier of the operation
    pub position: super::canvas::Point, // upper-left corner start coordinates
    pub dimensions: Option<super::canvas::Dimensions>, // flood fill doesn't need to define dimensions
    pub character: char,
}
