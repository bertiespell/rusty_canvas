/// Set of all possible operations
#[derive(Clone)]
pub enum CommandName {
    FillRectangle,
    OutlineRectangle,
    FloodFill,
}

/// Represents a drawing operation
#[derive(Clone)]
pub struct DrawCommand {
    pub name: CommandName, // name identifier of the operation
    pub position: super::canvas::Point, // upper-left corner start coordinates
    pub dimensions: Option<super::canvas::Dimensions>, // TODO: possibly a better way to handle this, but flood fill doesn't need to define dimensions
    pub character: char,
}

impl DrawCommand {
    /// A new draw command
    pub fn new(
        command_name: CommandName, 
        position: super::canvas::Point, 
        dimensions: Option<super::canvas::Dimensions>, 
        character: char
    ) -> DrawCommand {
        DrawCommand {
            name: command_name,
            position,
            dimensions,
            character,
        }
    }
}
