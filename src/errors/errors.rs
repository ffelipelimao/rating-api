use std::error::Error;

#[derive(Debug)]
pub struct InternalError {
    pub message: String,
}

impl InternalError {
    pub fn new<E: Error>(err: E) -> Self {
        InternalError {
            message: format!("An internal error happens: {}", err),
        }
    }
}