use warp::{reject::Reject};

#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct ApplyOperationError;

#[derive(Debug)]
/// Error returned when a draw operation is not succesful
pub struct StringTooLong;

impl Reject for ApplyOperationError {}
impl Reject for StringTooLong {}