
use std::convert::Infallible;
use std::error::Error;

use serde::{Serialize};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// Takes a warp rejection, and processes our custom errors
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    let mut deserialise_error = String::from("Error deserializing JSON: ");

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(super::errors::StringTooLong) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Fill and outline characters should be either 'none' or of length 1.";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let error = match e.source() {
            Some(cause) => cause.to_string(),
            None => String::from("BAD_REQUEST"),
        };
        deserialise_error.push_str(&error);
        message = &deserialise_error;
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(super::errors::ApplyOperationError) = err.find() {
        // We somehow passed an invalid draw operation to the application
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Oops, you must have passed something that wasn't a valid draw command. Validation should have caught that... it's a bug!";
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}