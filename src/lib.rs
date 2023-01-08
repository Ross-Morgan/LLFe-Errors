mod error;
mod position;

pub use error::LLFeError;
pub use position::Position;

pub type Result<T> = std::result::Result<T, LLFeError>;

pub fn new_error<T: ToString>(description: T, caused_by: Option<Box<LLFeError>>) -> LLFeError {
    LLFeError {
        description: description.to_string(),
        caused_by,
    }
}
