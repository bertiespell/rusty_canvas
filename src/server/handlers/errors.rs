use warp::{reject::Reject};

#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct ApplyOperationError;

#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct CharacterTooLong;
#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct CharacterDecodeError;

impl Reject for CharacterTooLong {}
impl Reject for ApplyOperationError {}
impl Reject for CharacterDecodeError {}