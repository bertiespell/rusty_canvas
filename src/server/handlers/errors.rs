use warp::{reject::Reject};

#[derive(Debug)]
pub struct ApplyOperationError;

impl Reject for ApplyOperationError {}