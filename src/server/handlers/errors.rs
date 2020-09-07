use warp::{reject::Reject};

#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct ApplyOperationError;

impl Reject for ApplyOperationError {}